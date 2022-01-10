use std::fmt::{Display, Formatter};
use std::time::Duration;

use parser::Goal;
use parser::Record;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub tags: Vec<Tag>,
    pub comment: Option<String>,
    pub date_range: DateRange,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub val: Vec<Prop>,
    pub(crate) start_pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    pub name: String,
    pub val: Option<String>,
    pub operator: PropOperator,
    pub(crate) start_pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum PropOperator {
    Eq,
    Less,
    More,
}

#[derive(Debug, PartialEq)]
pub struct DateRange {
    pub from: String,
    pub to: String,
}

impl DateRange {
    pub fn duration(&self) -> Duration {
        // TODO I'm still not sure about including chrono to core
        //      for now quick and dirty duration parsing
        let parse = |s: &String| {
            let minutes = &s[s.len() - 2..s.len()];
            let hours = &s[s.len() - 5..s.len() - 3];
            let hours: usize = hours.parse().unwrap();
            let minutes: usize = minutes.parse().unwrap();
            Duration::from_secs(((minutes + hours * 60) * 60) as u64)
        };
        let from = parse(&self.from);
        let to = parse(&self.to);
        to - from
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tags: Vec<String> = self
            .tags
            .iter()
            .map(|tag| {
                let mut s = String::new();
                s.push_str(&tag.name);
                for prop in &tag.val {
                    s.push(' ');
                    s.push_str(&prop.name);
                    if prop.val.is_some() {
                        s.push(match prop.operator {
                            PropOperator::Eq => '=',
                            PropOperator::Less => '<',
                            PropOperator::More => '>',
                        });
                        s.push_str(prop.val.as_ref().unwrap().as_str());
                    }
                }
                s
            })
            .collect();
        let mut s: String = String::new();
        s.push_str(&*self.date_range.from);
        s.push_str(" - ");
        s.push_str(&*self.date_range.to);
        s.push(' ');
        s.push_str(&tags.join(". "));
        if self.comment.is_some() {
            s.push_str(". ");
            // Escape line breaks in the comment
            let comment = self.comment.as_ref().unwrap();
            let escaped = comment.replace('\n', "\\n");
            s.push_str(&escaped);
        }
        write!(f, "{}", s)
    }
}

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

/*
    Grammar:
      ENTRY -> DATES TAGS COMMENT?
      DATETIMES -> DATETIME '-'? DATETIME?
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

    fn consume_formatted_number(&mut self, format: Vec<char>) -> Option<String> {
        let mut idx = 0;
        for c in self.input.chars().skip(self.pos) {
            let expected = format[idx];
            if expected == 'd' && !c.is_digit(10) {
                return None;
            }
            if expected != 'd' && expected != c {
                return None;
            }
            idx += 1;
            if idx == format.len() {
                break;
            }
        }
        if idx != format.len() {
            return None; // Too short line
        }
        let s = Some(
            self.input
                .chars()
                .skip(self.pos)
                .take(format.len())
                .collect(),
        );
        self.pos += format.len();
        s
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
        let (read, val) = self.read_string();
        self.pos += read;
        let val = if val.is_empty() { None } else { Some(val) };
        Some(Prop {
            name,
            val,
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
            val: props,
            start_pos,
        }))
    }

    // DATETIMES -> DATETIME '-'? DATETIME?
    fn parse_date_range(
        &mut self,
        time_prefix: &str,
        previous_time_end: &str,
    ) -> Result<DateRange, ParseError> {
        let parsed_date = match self.parse_date_time(time_prefix)? {
            None => {
                return Err(ParseError::BadDateTime(
                    "Entry should start with date/time".to_string(),
                    self.pos,
                ));
            }
            Some(parsed) => parsed,
        };
        self.consume_char('-');
        self.consume_char(' ');
        match self.parse_date_time(time_prefix)? {
            None => {
                let from = if previous_time_end.is_empty() {
                    parsed_date.clone()
                } else {
                    previous_time_end.to_string()
                };
                Ok(DateRange {
                    from,
                    to: parsed_date,
                })
            }
            Some(date_to) => Ok(DateRange {
                from: parsed_date,
                to: date_to,
            }),
        }
    }

    // DATETIME -> (DATE' ')? TIME
    fn parse_date_time(&mut self, time_prefix: &str) -> Result<Option<String>, ParseError> {
        let date = self.parse_date();
        if date.is_some() {
            self.consume_char(' ');
        }
        let time = self.parse_time();
        if time.is_none() {
            return Ok(None);
        }
        let time = time.unwrap();
        if date.is_none() && time_prefix.is_empty() {
            return Err(ParseError::BadDateTime(
                format!(
                    "No date specified for time {} and time prefix is empty",
                    time
                ),
                self.pos,
            ));
        }
        let date_time = format!(
            "{} {}",
            date.unwrap_or_else(|| time_prefix.to_string()),
            time
        );
        Ok(Some(date_time))
    }

    // DATE -> \d\d\d\d'-'\d\d'-'\d\d
    fn parse_date(&mut self) -> Option<String> {
        self.consume_formatted_number(vec!['d', 'd', 'd', 'd', '-', 'd', 'd', '-', 'd', 'd'])
    }

    // TIME -> \d\d':'\d\d
    fn parse_time(&mut self) -> Option<String> {
        self.consume_formatted_number(vec!['d', 'd', ':', 'd', 'd'])
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
    pub(crate) fn parse(
        &mut self,
        time_prefix: &str,
        previous_time_end: &str,
    ) -> Result<Record, ParseError> {
        if !previous_time_end.is_empty() && previous_time_end.len() != 16 {
            return Err(ParseError::BadDateTime(
                format!(
                    "Previous time end should in format: YYYY-MM-DD HH:MM, got {}",
                    previous_time_end
                ),
                0,
            ));
        }
        let date_range = self.parse_date_range(time_prefix, previous_time_end)?;
        let tags = self.parse_tags()?;
        let comment = self.parse_comment();
        self.create_record(date_range, tags, comment)
    }

    fn create_record(
        &self,
        date_range: DateRange,
        tags: Vec<Tag>,
        comment: Option<String>,
    ) -> Result<Record, ParseError> {
        // Check if record is a goal
        if tags.iter().any(|v| v.name == "goal") {
            return Ok(Record::Goal(Goal::create(tags, comment)?));
        }

        // Just a regular entry - validate and return
        for tag in &tags {
            for prop in &tag.val {
                if prop.operator != PropOperator::Eq {
                    let err = "Only `=` is supported in entry or additional `goal` tag is missed"
                        .to_string();
                    return Err(ParseError::BadOperator(err, self.pos));
                }
            }
        }

        Ok(Record::Entry(Entry {
            date_range,
            tags,
            comment,
        }))
    }
}

#[cfg(test)]
mod tests {
    use parser::Record;

    use super::*;

    #[test]
    fn parsing() {
        let d = || DateRange {
            from: "2000-01-01 01:01".to_string(),
            to: "2000-01-01 01:01".to_string(),
        };
        let prop = |name: &str, val: Option<&str>| Prop {
            name: name.to_string(),
            val: val.map(|v| v.to_string()),
            operator: PropOperator::Eq,
            start_pos: 0,
        };
        let tag = |name: &str, val: Vec<Prop>| Tag {
            name: name.to_string(),
            val,
            start_pos: 0,
        };

        let cases = vec![
            (
                "01:01 tag1",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![tag("tag1", vec![])],
                },
            ),
            (
                "01:01 tag1 prop1",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![tag("tag1", vec![prop("prop1", None)])],
                },
            ),
            (
                "01:01 tag1 prop1 val1",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![tag("tag1", vec![prop("prop1", Some("val1"))])],
                },
            ),
            (
                "01:01 tag1 prop1. tag2",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![tag("tag1", vec![prop("prop1", None)]), tag("tag2", vec![])],
                },
            ),
            (
                "01:01 tag1 prop1. tag2 prop2=val2",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![
                        tag("tag1", vec![prop("prop1", None)]),
                        tag("tag2", vec![prop("prop2", Some("val2"))]),
                    ],
                },
            ),
            (
                "01:01 tag1 prop1. tag2 prop2=val2. tag3",
                Entry {
                    date_range: d(),
                    comment: None,
                    tags: vec![
                        tag("tag1", vec![prop("prop1", None)]),
                        tag("tag2", vec![prop("prop2", Some("val2"))]),
                        tag("tag3", vec![]),
                    ],
                },
            ),
            (
                "01:01 tag1 prop1. tag2 prop2=val2. Comment here we are",
                Entry {
                    date_range: d(),
                    comment: Some("Comment here we are".to_string()),
                    tags: vec![
                        tag("tag1", vec![prop("prop1", None)]),
                        tag("tag2", vec![prop("prop2", Some("val2"))]),
                    ],
                },
            ),
            (
                "01:01 tag1  prop1.  tag2  prop2= val2.   Comment here we are  ",
                Entry {
                    date_range: d(),
                    comment: Some("Comment here we are".to_string()),
                    tags: vec![
                        tag("tag1", vec![prop("prop1", None)]),
                        tag("tag2", vec![prop("prop2", Some("val2"))]),
                    ],
                },
            ),
            (
                "01:01 tag1. Multiline\n- Something\n-Something else  ",
                Entry {
                    date_range: d(),
                    comment: Some("Multiline\n- Something\n-Something else".to_string()),
                    tags: vec![tag("tag1", vec![])],
                },
            ),
        ];
        for (input, want) in cases {
            let got = match Record::from_string(input, "2000-01-01", "").unwrap() {
                Record::Entry(mut entry) => {
                    // Ignore start_pos for comparison
                    for i in 0..entry.tags.len() {
                        entry.tags[i].start_pos = 0;
                        for y in 0..entry.tags[i].val.len() {
                            entry.tags[i].val[y].start_pos = 0;
                        }
                    }
                    entry
                }
                Record::Goal(_) => unreachable!(),
            };
            assert_eq!(got, want);
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
            let input = format!("2000-01-01 01:01 - 2000-01-01 02:02 {}", input);
            if let Record::Entry(entry) = Record::from_string(&input, "", "").unwrap() {
                let decoded = entry.to_string();
                assert_eq!(decoded, input);
            } else {
                unreachable!();
            };
        }
    }

    #[test]
    fn normalising() {
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
            // Properties values has equal sign before
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
        for (input, normalised) in cases {
            if let Record::Entry(got) = Record::from_string(input, "2000-01-01", "").unwrap() {
                assert_eq!(got.to_string(), normalised);
            } else {
                unreachable!();
            }
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
            let got = Record::from_string(input, "2000-11-11", "").err().unwrap();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn date_parsing() {
        let cases = vec![
            // First one
            (
                "01:22 tag1 prop1 val1",
                "",
                "2000-11-11",
                ("2000-11-11 01:22", "2000-11-11 01:22"),
            ),
            // Supplied previous entry end time
            (
                "01:42 tag1 prop1 val1",
                "2000-11-11 01:22",
                "2000-11-11",
                ("2000-11-11 01:22", "2000-11-11 01:42"),
            ),
            // Both values regardless if supplied previous entry end time
            (
                "01:25 - 01:45 tag1 prop1 val1",
                "2000-11-11 01:22",
                "2000-11-11",
                ("2000-11-11 01:25", "2000-11-11 01:45"),
            ),
            // Dash time separator is optional
            (
                "15:25 15:28 tag1 prop1 val1",
                "",
                "2000-11-11",
                ("2000-11-11 15:25", "2000-11-11 15:28"),
            ),
            // Absolute time
            (
                "2000-12-12 23:12 tag1 prop2 val1",
                "",
                "2000-11-11",
                ("2000-12-12 23:12", "2000-12-12 23:12"),
            ),
            // Absolute time with previous one
            (
                "2000-11-11 01:42 tag1 prop1 val1",
                "2000-11-11 01:22",
                "",
                ("2000-11-11 01:22", "2000-11-11 01:42"),
            ),
            // Absolute two times
            (
                "2000-12-12 23:12 - 2000-12-12 23:42 tag1 prop2 val1",
                "",
                "2000-11-11",
                ("2000-12-12 23:12", "2000-12-12 23:42"),
            ),
        ];
        for (input, previous, prefix, (from, to)) in cases {
            if let Record::Entry(parsed) = Record::from_string(input, prefix, previous).unwrap() {
                assert_eq!(parsed.date_range.from, from);
                assert_eq!(parsed.date_range.to, to);
            } else {
                unreachable!();
            }
        }
    }
}
