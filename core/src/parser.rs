use std::fmt::{Display, Formatter};

use crate::datetime::{Date, DateTime, DateTimeRange, DayTime};
use crate::record::{Entry, Prop, PropOperator, PropVal, Tag};

/*
    Grammar:
      ENTRY -> DATES TAGS COMMENT?
      DATES -> DATETIME '-'? DATETIME?
      DATETIME -> (DATE)? TIME
      DATE -> \d\d\d\d'-'\d\d'-'\d\d
      TIME -> \d\d':'\d\d
      TAGS -> TAG ('.' TAGS)*
      TAG -> TAGNAME (PROP)*
      PROP_OP -> '=' / '<' / '>'
      PROP -> PROPNAME (PROP_OP? PROPVALUE)?
      COMMENT -> \W \w*
      TAGNAME -> \w+
      PROPNAME -> (\w\W)+
      PROPVALUE -> (\w|\W)+
*/

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NoTags,
    Duplicate(String, usize),
    BadDateTime(String, usize),
    BadOperator(String, usize),
    MissingProperty(String, usize),
    BadValue(String, usize),
    BadQuery(String, usize),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn read_string(&self) -> (usize, String) {
        let sep = &[' ', '.', '=', '<', '>'];
        let mut str_end = 0;
        let mut spaces = 0;
        let mut skip_spaces = true;
        for c in self.input.chars().skip(self.pos) {
            if c.is_whitespace() && skip_spaces {
                spaces += 1;
                continue;
            }
            skip_spaces = false;
            if sep.contains(&c) {
                break;
            }
            str_end += 1;
        }
        let s: String = self
            .input
            .chars()
            .skip(self.pos + spaces)
            .take(str_end)
            .collect();
        (spaces + str_end, s)
    }

    fn consume_char(&mut self, expected: char) -> bool {
        for c in self.input.chars().skip(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
                continue;
            } else if c == expected {
                self.pos += 1;
                return true;
            } else {
                return false;
            }
        }
        false
    }

    // PROP -> PROPNAME (' ' PROPVALUE)?
    fn parse_prop(&mut self) -> Option<Prop> {
        let start_pos = self.pos;
        let (read, name) = self.read_string();
        self.pos += read;
        if name.is_empty() {
            return None;
        }
        let operator = if self.consume_char('>') {
            PropOperator::More
        } else if self.consume_char('<') {
            PropOperator::Less
        } else {
            // TODO Return error in case of invalid symbol
            self.consume_char('='); // It's optional, but if exists go to next char
            PropOperator::Eq
        };
        let (read, mut val) = self.read_string();
        self.pos += read;

        // Prop values could be a float written with dot as a separator
        // Dot is used as a tag separator, so such floats wouldn't be parsed correctly
        // To make UX better treat such cases in a special way. It's safe to assume
        // that no tags would start with a digit
        let mut chars = self.input.chars().skip(self.pos);
        if chars
            .next()
            .filter(|c| *c == '.')
            .and_then(|_| chars.next())
            .filter(|c| c.is_digit(10))
            .is_some()
        {
            self.consume_char('.');
            let (read_more, val_more) = self.read_string();
            val = format!("{}.{}", val, val_more);
            self.pos += read_more;
        }
        Some(Prop {
            name,
            val: PropVal::parse(&val),
            operator,
            start_pos,
        })
    }

    // TAGNAME -> \w+
    fn tagname(&mut self) -> Option<String> {
        let (read, name) = self.read_string();
        self.pos += read;
        if name.is_empty() {
            return None;
        }
        if name.to_lowercase() == name {
            Some(name)
        } else {
            // Read ahead until the comment, step back
            self.pos -= read;
            None
        }
    }

    // TAG -> TAGNAME (TAGPROP)*
    fn parse_tag(&mut self) -> Result<Option<Tag>, ParseError> {
        let start_pos = self.pos;
        let name = self.tagname();
        if name.is_none() {
            return Ok(None);
        }
        let mut props: Vec<Prop> = Vec::new();
        while let Some(prop) = self.parse_prop() {
            if props.iter().any(|t| t.name == prop.name) {
                return Err(ParseError::Duplicate(
                    format!("Duplicate prop: {}", prop.name),
                    self.pos,
                ));
            }
            props.push(prop)
        }
        Ok(Some(Tag {
            name: name.unwrap(),
            props: props,
            start_pos,
        }))
    }

    // DATETIMES -> DATETIME '-'? DATETIME?
    // TODO Simplify it, and before think if we actually need all this functionality
    fn parse_date_range(
        &mut self,
        base_date: Option<Date>,
        prev_time: Option<DayTime>,
    ) -> Result<DateTimeRange, ParseError> {
        let parsed_date = match self.parse_date_time(base_date.clone())? {
            Some(parsed) => parsed,
            None => {
                return Err(ParseError::BadDateTime(
                    "Entry should start with date/time".to_string(),
                    self.pos,
                ));
            }
        };
        // Separators in case of two date/time specified
        self.consume_char('-');
        self.consume_char(' ');
        match self.parse_date_time(Some(parsed_date.clone().date))? {
            Some(date_to) => Ok(DateTimeRange {
                start: parsed_date,
                end: date_to,
            }),
            None => {
                // No second date found. If we know time when previous record has ended via
                // prev_time use that as a start of a new record. If not known we assume it's
                // a first record of the day with same start and end
                if let Some(prev_time) = prev_time {
                    Ok(DateTimeRange {
                        // TODO Looks like all this logic of handling records without datetime ranges is too complex
                        start: DateTime::new(base_date.unwrap(), prev_time),
                        end: parsed_date,
                    })
                } else {
                    Ok(DateTimeRange {
                        start: parsed_date.clone(),
                        end: parsed_date,
                    })
                }
            }
        }
    }

    // DATETIME -> (DATE' ')? TIME
    fn parse_date_time(&mut self, base_date: Option<Date>) -> Result<Option<DateTime>, ParseError> {
        let date = match (self.parse_date(), base_date) {
            (None, None) => {
                return Err(ParseError::BadDateTime(
                    "Failed to parse the date".to_string(),
                    self.pos,
                ))
            }
            (None, Some(v)) => v,
            (Some(v), None) => v,
            (Some(v), Some(_)) => v,
        };
        self.consume_char(' '); // Date and Time separator
        let time = match self.parse_time() {
            None => return Ok(None),
            Some(time) => time,
        };
        Ok(Some(DateTime { date, time }))
    }

    // DATE -> \d\d\d\d'-'\d\d'-'\d\d
    fn parse_date(&mut self) -> Option<Date> {
        let date_len = 10;
        if self.pos + date_len >= self.input.len() {
            return None;
        }
        match self.input[self.pos..self.pos + date_len].parse::<Date>() {
            Err(_) => None,
            Ok(date) => {
                self.pos += date_len;
                Some(date)
            }
        }
    }

    // TIME -> \d\d':'\d\d
    fn parse_time(&mut self) -> Option<DayTime> {
        let time_len = 5;
        if self.pos + time_len >= self.input.len() {
            return None;
        }
        match self.input[self.pos..self.pos + time_len].parse::<DayTime>() {
            Err(_) => None,
            Ok(time) => {
                self.pos += time_len;
                Some(time)
            }
        }
    }

    // TAGS -> TAG ('.' TAGS)*
    fn parse_tags(&mut self) -> Result<Vec<Tag>, ParseError> {
        let mut tags: Vec<Tag> = Vec::new();
        loop {
            match self.parse_tag()? {
                Some(tag) => {
                    if tags.iter().any(|t| t.name == tag.name) {
                        return Err(ParseError::Duplicate(
                            format!("Duplicate tag: {}", tag.name),
                            self.pos,
                        ));
                    }
                    tags.push(tag)
                }
                None => {
                    if tags.is_empty() {
                        return Err(ParseError::NoTags);
                    }
                    break;
                }
            }
            if !self.consume_char('.') {
                break;
            }
        }
        Ok(tags)
    }

    // COMMENT -> \W \w*
    fn parse_comment(&mut self) -> Option<String> {
        let comment: String = self.input.chars().skip(self.pos).collect();
        let comment = comment.trim();
        if comment.is_empty() {
            return None;
        }
        return match comment.chars().next() {
            Some(first) => {
                if !first.is_uppercase() {
                    unreachable!("it's not a comment but a tag: {:?}", self.input);
                }
                Some(String::from(comment))
            }
            None => None,
        };
    }

    // ENTRY -> TAGS COMMENT?
    pub fn parse_date_record(
        &mut self,
        date: Option<Date>,
        time: Option<DayTime>,
    ) -> Result<Entry, ParseError> {
        let date_range = self.parse_date_range(date, time)?;
        let tags = self.parse_tags()?;
        let comment = self.parse_comment();
        for tag in &tags {
            for prop in &tag.props {
                if prop.operator != PropOperator::Eq {
                    let err = "Only `=` is supported in entry or additional `goal` tag is missed"
                        .to_string();
                    return Err(ParseError::BadOperator(err, self.pos));
                }
            }
        }
        Ok(Entry {
            date_range,
            tags,
            comment,
        })
    }

    // Parse record without a date prefix
    pub fn parse_record(&mut self) -> Result<(Vec<Tag>, Option<String>), ParseError> {
        let tags = self.parse_tags()?;
        let comment = self.parse_comment();
        Ok((tags, comment))
    }
}

#[cfg(test)]
mod tests {
    use crate::datetime::TimeDuration;

    use super::*;

    const BASE_DATE: Date = Date::new(2000, 1, 1);

    #[test]
    fn entry_parsing() {
        let dr = |hours: u8, minutes: u8| DateTimeRange {
            start: DateTime::new(BASE_DATE.clone(), DayTime::new(hours, minutes)),
            end: DateTime::new(BASE_DATE.clone(), DayTime::new(hours, minutes)),
        };
        let prop = |name: &str, val: PropVal| Prop {
            name: name.to_string(),
            val,
            operator: PropOperator::Eq,
            start_pos: 0,
        };
        let tag = |name: &str, props: Vec<Prop>| Tag::new(name.to_string(), props, 0);

        let cases = vec![
            (
                // Simple tag
                "01:01 tag1",
                Entry::new(dr(1, 1), None, vec![tag("tag1", vec![])]),
            ),
            (
                // Simple tag, simple prop
                "01:01 tag1 prop1",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Simple tag, simple prop and val
                "01:01 tag1 prop1 val1",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag(
                        "tag1",
                        vec![prop("prop1", PropVal::String("val1".to_string()))],
                    )],
                ),
            ),
            (
                // Multiple tags
                "01:01 tag1 prop1. tag2",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![
                        tag("tag1", vec![prop("prop1", PropVal::None)]),
                        tag("tag2", vec![]),
                    ],
                ),
            ),
            (
                // Multiple tags with props and vals
                "01:01 tag1 prop1. tag2 prop2=val2",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![
                        tag("tag1", vec![prop("prop1", PropVal::None)]),
                        tag(
                            "tag2",
                            vec![prop("prop2", PropVal::String("val2".to_string()))],
                        ),
                    ],
                ),
            ),
            (
                // Spaces are ignored and results are trimmed
                "  01:01  tag1  prop1 =   val2  .    tag2  . Comment    ",
                Entry::new(
                    dr(1, 1),
                    Some("Comment".to_string()),
                    vec![
                        tag(
                            "tag1",
                            vec![prop("prop1", PropVal::String("val2".to_string()))],
                        ),
                        tag("tag2", vec![]),
                    ],
                ),
            ),
            (
                // Special handling of floats with dot as a separator
                "01:01 tag1 prop1=9.5 prop2=6.33.tag2",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![
                        tag(
                            "tag1",
                            vec![
                                prop("prop1", PropVal::Number(9.5f32)),
                                prop("prop2", PropVal::Number(6.33f32)),
                            ],
                        ),
                        tag("tag2", vec![]),
                    ],
                ),
            ),
            (
                // Simple tag, simple prop and a comment
                "01:01 tag1 prop1. Comment",
                Entry::new(
                    dr(1, 1),
                    Some("Comment".to_string()),
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Simple tag, simple prop and a multiline comment
                "01:01 tag1 prop1. Line1\nLine2",
                Entry::new(
                    dr(1, 1),
                    Some("Line1\nLine2".to_string()),
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Prop value duration
                "01:01 tag1 prop1=12:22 prop2=0:01",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag(
                        "tag1",
                        vec![
                            prop("prop1", PropVal::Time(TimeDuration::new(12, 22))),
                            prop("prop2", PropVal::Time(TimeDuration::new(0, 1))),
                        ],
                    )],
                ),
            ),
        ];
        for (input, want) in cases {
            match Entry::parse(input, BASE_DATE.clone(), None) {
                Ok(entry) => {
                    assert_eq!(entry, want);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn encoding_decoding() {
        let cases = vec![
            "tag1",
            "tag1 prop1",
            "tag1 prop1=val1",
            "tag1. tag2",
            "tag1 prop1=val1. tag2 prop2=val2",
            "tag1 prop1=val1 prop2=val2. tag2",
            "tag1. Comment",
            "tag1. tag2. Comment And More Text",
            "tag1 Prop1",
            "tag1 Prop1=Val1",
            "tag1 Prop1=Val1 Prop2",
            "tag1 Prop1=Val1 Prop2",
            "tag1 Prop1=Val1. Comment And More Text",
            "tag1 tagref=#tag2",
        ];
        for input in cases {
            let input = format!("{} 02:02 - {} 02:02 {}", BASE_DATE, BASE_DATE, input);
            let entry = Entry::parse(&input, BASE_DATE, None).unwrap();
            let decoded = entry.to_string();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn normalizing() {
        let cases = vec![
            // Spaces trimmed and collapsed
            ("01:01  tag1  ", "2000-01-01 01:01 - 2000-01-01 01:01 tag1"),
            (
                "01:01 tag1  prop1   ",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1",
            ),
            // Comments are trimmed but comment content is not touched
            (
                "01:01 tag1  prop1  . Comment >>  |  <<  ",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1. Comment >>  |  <<",
            ),
            // Properties values has equal signs
            (
                "01:01 tag1 prop1 val1 prop2 val2 prop3",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1=val1 prop2=val2 prop3",
            ),
            // Multiline strings
            (
                "01:01 tag1. Multiline\n- Some1\n\n-Some2 ",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1. Multiline\\n- Some1\\n\\n-Some2",
            ),
        ];
        for (input, normalized) in cases {
            let entry = Entry::parse(input, BASE_DATE, None).unwrap();
            assert_eq!(entry.to_string(), normalized);
        }
    }

    #[test]
    fn parsing_errors() {
        let cases = vec![
            ("01:01  .", ParseError::NoTags),
            (
                "01:01 tag1. tag1",
                ParseError::Duplicate("Duplicate tag: tag1".to_string(), 16),
            ),
            (
                "01:01 tag1 prop1=a prop1=b",
                ParseError::Duplicate("Duplicate prop: prop1".to_string(), 26),
            ),
            (
                "tag1 prop1=a prop1=b",
                ParseError::BadDateTime("Entry should start with date/time".to_string(), 0),
            ),
        ];
        for (input, expected) in cases {
            let got = Entry::parse(input, BASE_DATE, None).err().unwrap();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn parse_datetime_relevance() {
        // First record of the day, no prev time known, record will be of zero duration
        let entry = Entry::parse("07:00 foo", BASE_DATE, None).unwrap();
        assert_eq!(entry.date_range.duration(), TimeDuration::new(0, 0));
        // Following records duration calculated relevant to previous record end time
        let entry = Entry::parse("07:00 foo", BASE_DATE, Some(DayTime::new(6, 30))).unwrap();
        assert_eq!(entry.date_range.duration(), TimeDuration::new(0, 30));
        // Short daterange notation: Date Time Time
        let entry = Entry::parse("2011-11-21 07:00 08:10 foo", BASE_DATE, None).unwrap();
        assert_eq!(entry.date_range.start.date, Date::new(2011, 11, 21));
        assert_eq!(entry.date_range.duration(), TimeDuration::new(1, 10));
    }
}
