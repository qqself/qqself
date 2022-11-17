use thiserror::Error;

use crate::{
    date_time::{
        datetime::{DateDay, DateTime, Time},
        datetime_range::DateTimeRange,
    },
    record::{Entry, Prop, PropOperator, PropVal, Tag},
};

/*
    Grammar:
      ENTRY -> DATES TAGS COMMENT?
      DATES -> DATETIME '-'? (DATETIME / TIME)
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

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("No tags were found in an entry")]
    NoTags,
    #[error("Duplicate {0} at position {1}")]
    Duplicate(String, usize),
    #[error("Bad datetime: {0} at position {1}")]
    BadDateTime(String, usize),
    #[error("Bad operator: {0} at position {1}")]
    BadOperator(String, usize),
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
            .filter(|c| c.is_ascii_digit())
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
                    format!("property {}", prop.name),
                    self.pos,
                ));
            }
            props.push(prop)
        }
        Ok(Some(Tag {
            name: name.unwrap(),
            props,
            start_pos,
        }))
    }

    // DATETIMES -> DATETIME '-'? DATETIME?
    fn parse_date_range(&mut self) -> Result<DateTimeRange, ParseError> {
        let start = self.parse_date_time(None)?.ok_or_else(|| {
            ParseError::BadDateTime("missing start datetime".to_string(), self.pos)
        })?;
        // Separators in case of two date/time specified
        self.consume_char('-');
        self.consume_char(' ');
        let end = self
            .parse_date_time(Some(start.date()))?
            .ok_or_else(|| ParseError::BadDateTime("missing end datetime".to_string(), self.pos))?;
        DateTimeRange::new(start, end).map_err(|v| ParseError::BadDateTime(v.to_string(), self.pos))
    }

    // DATETIME -> (DATE' ')? TIME
    fn parse_date_time(
        &mut self,
        base_date: Option<DateDay>,
    ) -> Result<Option<DateTime>, ParseError> {
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
        Ok(Some(DateTime::new(date, time)))
    }

    // DATE -> \d\d\d\d'-'\d\d'-'\d\d
    fn parse_date(&mut self) -> Option<DateDay> {
        let date_len = 10;
        if self.pos + date_len >= self.input.len() {
            return None;
        }
        match self.input[self.pos..self.pos + date_len].parse::<DateDay>() {
            Err(_) => None,
            Ok(date) => {
                self.pos += date_len;
                Some(date)
            }
        }
    }

    // TIME -> \d\d':'\d\d
    fn parse_time(&mut self) -> Option<Time> {
        let time_len = 5;
        if self.pos + time_len >= self.input.len() {
            return None;
        }
        match self.input[self.pos..self.pos + time_len].parse::<Time>() {
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
                        return Err(ParseError::Duplicate(format!("tag {}", tag.name), self.pos));
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
                    // TODO Parser should never panic incase of invalid input, convert it to an error
                    unreachable!("it's not a comment but a tag: {:?}", self.input);
                }
                Some(String::from(comment))
            }
            None => None,
        };
    }

    // ENTRY -> TAGS COMMENT?
    pub fn parse_date_record(&mut self) -> Result<Entry, ParseError> {
        let date_range = self.parse_date_range()?;
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
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::date_time::datetime::Duration;

    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BASE_DATE: DateDay = DateDay::new(2000, 1, 1);
    }

    #[test]
    #[wasm_bindgen_test]
    fn entry_parsing() {
        let dr = |hours: u8, minutes: u8| {
            DateTimeRange::new(
                DateTime::new(*BASE_DATE, Time::new(hours, minutes)),
                DateTime::new(*BASE_DATE, Time::new(hours, minutes)),
            )
            .unwrap()
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
                "2000-01-01 01:01 01:01 tag1",
                Entry::new(dr(1, 1), None, vec![tag("tag1", vec![])]),
            ),
            (
                // Simple tag, simple prop
                "2000-01-01 01:01 01:01 tag1 prop1",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Simple tag, simple prop and val
                "2000-01-01 01:01 01:01 tag1 prop1 val1",
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
                "2000-01-01 01:01 01:01 tag1 prop1. tag2",
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
                "2000-01-01 01:01 01:01 tag1 prop1. tag2 prop2=val2",
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
                "2000-01-01 01:01  01:01  tag1  prop1 =   val2  .    tag2  . Comment    ",
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
                "2000-01-01 01:01 01:01 tag1 prop1=9.5 prop2=6.33.tag2",
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
                "2000-01-01 01:01 01:01 tag1 prop1. Comment",
                Entry::new(
                    dr(1, 1),
                    Some("Comment".to_string()),
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Simple tag, simple prop and a multiline comment
                "2000-01-01 01:01 01:01 tag1 prop1. Line1\nLine2",
                Entry::new(
                    dr(1, 1),
                    Some("Line1\nLine2".to_string()),
                    vec![tag("tag1", vec![prop("prop1", PropVal::None)])],
                ),
            ),
            (
                // Prop value duration
                "2000-01-01 01:01 01:01 tag1 prop1=12:22 prop2=0:01",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag(
                        "tag1",
                        vec![
                            prop("prop1", PropVal::Time(Duration::new(12, 22))),
                            prop("prop2", PropVal::Time(Duration::new(0, 1))),
                        ],
                    )],
                ),
            ),
        ];
        for (input, want) in cases {
            match Entry::parse(input) {
                Ok(entry) => {
                    assert_eq!(entry, want);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    #[wasm_bindgen_test]
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
        ];
        for input in cases {
            let input = format!("2000-01-01 02:02 - 2000-01-01 02:02 {input}");
            let entry = Entry::parse(&input).unwrap();
            let decoded = entry.to_string();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn normalizing() {
        let cases = vec![
            // Spaces trimmed and collapsed
            (
                "2000-01-01 01:01 01:03 tag1  ",
                "2000-01-01 01:01 - 2000-01-01 01:03 tag1",
            ),
            (
                "2000-01-01 01:01 01:02 tag1  prop1   ",
                "2000-01-01 01:01 - 2000-01-01 01:02 tag1 prop1",
            ),
            // Comments are trimmed but comment content is not touched
            (
                "2000-01-01 01:01 01:01 tag1  prop1  . Comment >>  |  <<  ",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1. Comment >>  |  <<",
            ),
            // Properties values has equal signs
            (
                "2000-01-01 01:01 01:01 tag1 prop1 val1 prop2 val2 prop3",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1=val1 prop2=val2 prop3",
            ),
            // Multiline strings
            (
                "2000-01-01 01:01 01:01 tag1. Multiline\n- Some1\n\n-Some2 ",
                "2000-01-01 01:01 - 2000-01-01 01:01 tag1. Multiline\\n- Some1\\n\\n-Some2",
            ),
        ];
        for (input, normalized) in cases {
            let entry = Entry::parse(input).unwrap();
            assert_eq!(entry.to_string(), normalized);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn parsing_errors() {
        let cases = vec![
            ("2010-01-01 01:01 01:01 .", ParseError::NoTags),
            (
                "2010-01-01 01:01 01:01 tag1. tag1",
                ParseError::Duplicate("tag tag1".to_string(), 33),
            ),
            (
                "2010-01-01 01:01 01:01 tag1 prop1=a prop1=b",
                ParseError::Duplicate("property prop1".to_string(), 43),
            ),
            (
                "tag1 prop1=a prop1=b",
                ParseError::BadDateTime("Failed to parse the date".to_string(), 0),
            ),
            (
                "2010-01-01 10:00 09:00 tag1 prop1=a prop1=b",
                ParseError::BadDateTime("end time cannot be before the start".to_string(), 22),
            ),
        ];
        for (input, expected) in cases {
            let got = Entry::parse(input).err().unwrap();
            assert_eq!(got, expected);
        }
    }
}
