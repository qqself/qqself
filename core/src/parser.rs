use parser::ParseError::BadDateTime;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct DateRange {
    from: String,
    to: String,
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    tags: Vec<Tag>,
    comment: Option<String>,
    date_range: DateRange,
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
                        s.push('=');
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
pub struct Tag {
    name: String,
    val: Vec<Prop>,
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    name: String,
    val: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    // String contains no tag characters
    NoTags,
    // Duplicate tag or property at position
    Duplicate(String, usize),
    // Bad datetime format
    BadDateTime(String, usize),
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
      PROP -> PROPNAME ('='? PROPVALUE)?
      COMMENT -> \W \w*
      TAGNAME -> \w+
      PROPNAME -> (\w\W)+
      PROPVALUE -> (\w|\W)+
*/

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn read_string(&self) -> (usize, String) {
        let sep = &[' ', '.', '='];
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
        let (read, name) = self.read_string();
        self.pos += read;
        if name.is_empty() {
            return None;
        }
        self.consume_char('='); // Optional = sign
        let (read, val) = self.read_string();
        self.pos += read;
        let val = if val.is_empty() {
            None
        } else {
            Some(val.to_string())
        };
        Some(Prop {
            name: name.to_string(),
            val,
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
            Some(name.to_string())
        } else {
            // Read ahead until the comment, step back
            self.pos -= read;
            None
        }
    }

    // TAG -> TAGNAME (TAGPROP)*
    fn parse_tag(&mut self) -> Result<Option<Tag>, ParseError> {
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
        return match self.parse_date_time(time_prefix)? {
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
        };
    }

    // DATETIME -> (DATE' ')? TIME
    fn parse_date_time(&mut self, time_prefix: &str) -> Result<Option<String>, ParseError> {
        let date = self.parse_date();
        if date.is_some() {
            self.consume_char(' ');
        }
        let time = self.parse_time();
        return if time.is_none() {
            Ok(None)
        } else {
            let time = time.unwrap();
            if date.is_none() && time_prefix.is_empty() {
                return Err(BadDateTime(
                    format!(
                        "No date specified for time {} and time prefix is empty",
                        time
                    ),
                    self.pos,
                ));
            }
            let date_time = format!("{} {}", date.unwrap_or(time_prefix.to_string()), time);
            Ok(Some(date_time))
        };
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
                    unreachable!("it's not a comment but a tag")
                }
                Some(String::from(comment))
            }
            None => None,
        };
    }

    // ENTRY -> TAGS COMMENT?
    fn parse(&mut self, time_prefix: &str, previous_time_end: &str) -> Result<Entry, ParseError> {
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
        Ok(Entry {
            tags,
            comment,
            date_range,
        })
    }
}

impl Entry {
    pub fn from_string(
        input: &str,
        time_prefix: &str,
        previous_time_end: &str,
    ) -> Result<Entry, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse(time_prefix, previous_time_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let d = || DateRange {
            from: "2000-01-01 01:01".to_string(),
            to: "2000-01-01 01:01".to_string(),
        };
        #[rustfmt::skip]
        let cases = vec![
            ("01:01 tag1", Entry { date_range: d(), comment: None, tags: vec![Tag { name: "tag1".to_string(), val: vec![] }]}),
            ("01:01 tag1 prop1", Entry { date_range: d(), comment: None, tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
            ]}),
            ("01:01 tag1 prop1 val1", Entry { date_range: d(),comment: None, tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: Some("val1".to_string()) }] },
            ]}),
            ("01:01 tag1 prop1. tag2", Entry { date_range: d(), comment: None, tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
                Tag { name: "tag2".to_string(), val: vec![] },
            ]}),
            ("01:01 tag1 prop1. tag2 prop2=val2", Entry { date_range: d(), comment: None, tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
                Tag { name: "tag2".to_string(), val: vec![Prop { name: "prop2".to_string(), val: Some("val2".to_string()) }] },
            ]}),
            ("01:01 tag1 prop1. tag2 prop2=val2. tag3", Entry { date_range: d(), comment: None, tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
                Tag { name: "tag2".to_string(), val: vec![Prop { name: "prop2".to_string(), val: Some("val2".to_string()) }] },
                Tag { name: "tag3".to_string(), val: vec![] },
            ]}),
            ("01:01 tag1 prop1. tag2 prop2=val2. Comment here we are", Entry { date_range: d(),  comment: Some("Comment here we are".to_string()), tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
                Tag { name: "tag2".to_string(), val: vec![Prop { name: "prop2".to_string(), val: Some("val2".to_string()) }] },
            ]}),
            ("01:01 tag1  prop1.  tag2  prop2= val2.   Comment here we are  ", Entry { date_range: d(), comment: Some("Comment here we are".to_string()), tags: vec![
                Tag { name: "tag1".to_string(), val: vec![Prop { name: "prop1".to_string(), val: None }] },
                Tag { name: "tag2".to_string(), val: vec![Prop { name: "prop2".to_string(), val: Some("val2".to_string()) }] },
            ]}),
            ("01:01 tag1. Multiline\n- Something\n-Something else  ", Entry { date_range: d(), comment: Some("Multiline\n- Something\n-Something else".to_string()), tags: vec![
                Tag { name: "tag1".to_string(), val: vec![] },
            ]}),
        ];
        for (input, want) in cases {
            let got = Entry::from_string(input, "2000-01-01", "").unwrap();
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
            let entry = Entry::from_string(&input, "", "").unwrap();
            let decoded = entry.to_string();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn normalising() {
        #[rustfmt::skip]
        let cases = vec![
            // Spaces trimmed and collapsed
            ("01:01  tag1  ", "2000-01-01 01:01 - 2000-01-01 01:01 tag1"),
            ("01:01 tag1  prop1   ", "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1"),
            // Comments are trimmed but comment content is not touched
            ("01:01 tag1  prop1  . Comment >>  |  <<  ", "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1. Comment >>  |  <<"),
            // Properties values has equal sign before
            ("01:01 tag1 prop1 val1 prop2 val2 prop3", "2000-01-01 01:01 - 2000-01-01 01:01 tag1 prop1=val1 prop2=val2 prop3"),
            // Multiline strings
            ("01:01 tag1. Multiline\n- Some1\n\n-Some2 ", "2000-01-01 01:01 - 2000-01-01 01:01 tag1. Multiline\\n- Some1\\n\\n-Some2"),
        ];
        for (input, normalised) in cases {
            let got = Entry::from_string(input, "2000-01-01", "").unwrap();
            assert_eq!(got.to_string(), normalised);
        }
    }

    #[test]
    fn parsing_errors() {
        #[rustfmt::skip]
        let cases = vec![
            ("01:01  .", ParseError::NoTags),
            ("01:01 tag1. tag1", ParseError::Duplicate("Duplicate tag: tag1".to_string(), 16)),
            ("01:01 tag1 prop1=a prop1=b", ParseError::Duplicate("Duplicate prop: prop1".to_string(), 26)),
            ("tag1 prop1=a prop1=b", ParseError::BadDateTime("Entry should start with date/time".to_string(), 0)),
        ];
        for (input, expected) in cases {
            let got = Entry::from_string(input, "2000-11-11", "").err().unwrap();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn date_parsing() {
        #[rustfmt::skip]
        let cases = vec![
            // First one
            ("01:22 tag1 prop1 val1", "", "2000-11-11", ("2000-11-11 01:22", "2000-11-11 01:22")),
            // Supplied previous entry end time
            ("01:42 tag1 prop1 val1", "2000-11-11 01:22", "2000-11-11", ("2000-11-11 01:22", "2000-11-11 01:42")),
            // Both values regardless if supplied previous entry end time
            ("01:25 - 01:45 tag1 prop1 val1", "2000-11-11 01:22", "2000-11-11", ("2000-11-11 01:25", "2000-11-11 01:45")),
            // Dash time separator is optional
            ("15:25 15:28 tag1 prop1 val1", "", "2000-11-11", ("2000-11-11 15:25", "2000-11-11 15:28")),
            // Absolute time
            ("2000-12-12 23:12 tag1 prop2 val1", "", "2000-11-11", ("2000-12-12 23:12", "2000-12-12 23:12")),
            // Absolute time with previous one
            ("2000-11-11 01:42 tag1 prop1 val1", "2000-11-11 01:22", "", ("2000-11-11 01:22", "2000-11-11 01:42")),
            // Absolute two times
            ("2000-12-12 23:12 - 2000-12-12 23:42 tag1 prop2 val1", "", "2000-11-11", ("2000-12-12 23:12", "2000-12-12 23:42")),
        ];
        for (input, previous, prefix, (from, to)) in cases {
            let parsed = Entry::from_string(input, prefix, previous).unwrap();
            assert_eq!(parsed.date_range.from, from);
            assert_eq!(parsed.date_range.to, to);
        }
    }
}
