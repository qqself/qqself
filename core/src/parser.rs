#[derive(Debug, PartialEq)]
pub struct Entry {
    tags: Vec<Tag>,
    comment: Option<String>,
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
}

/*
    Grammar:
      ENTRY -> TAGS COMMENT?
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

    fn consume(&mut self, expected: char) -> bool {
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
    fn parse_prop(&mut self) -> Result<Option<Prop>, ParseError> {
        let (read, name) = self.read_string();
        self.pos += read;
        if name.is_empty() {
            return Ok(None);
        }
        self.consume('='); // Optional = sign
        let (read, val) = self.read_string();
        self.pos += read;
        let val = if val.is_empty() {
            None
        } else {
            Some(val.to_string())
        };
        Ok(Some(Prop {
            name: name.to_string(),
            val,
        }))
    }

    // TAGNAME -> \w+
    fn tagname(&mut self) -> Result<Option<String>, ParseError> {
        let (read, name) = self.read_string();
        self.pos += read;
        if name.is_empty() {
            return Ok(None);
        }
        return if name.to_lowercase() == name {
            Ok(Some(name.to_string()))
        } else {
            // Read ahead until the comment, step back
            self.pos -= read;
            Ok(None)
        };
    }

    // TAG -> TAGNAME (TAGPROP)*
    fn parse_tag(&mut self) -> Result<Option<Tag>, ParseError> {
        let name = self.tagname()?;
        if name.is_none() {
            return Ok(None);
        }
        let mut props: Vec<Prop> = Vec::new();
        loop {
            match self.parse_prop()? {
                Some(prop) => {
                    if props.iter().any(|t| t.name == prop.name) {
                        return Err(ParseError::Duplicate(
                            format!("Duplicate prop: {}", prop.name),
                            self.pos,
                        ));
                    }
                    props.push(prop)
                }
                None => {
                    break;
                }
            }
        }
        Ok(Some(Tag {
            name: name.unwrap(),
            val: props,
        }))
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
            if !self.consume('.') {
                break;
            }
        }
        Ok(tags)
    }

    // COMMENT -> \W \w*
    fn parse_comment(&mut self) -> Result<Option<String>, ParseError> {
        let comment: String = self.input.chars().skip(self.pos).collect();
        let comment = comment.trim();
        if comment.len() == 0 {
            return Ok(None);
        }
        return match comment.chars().next() {
            Some(first) => {
                if !first.is_uppercase() {
                    unreachable!("it's not a comment but a tag")
                }
                Ok(Some(String::from(comment)))
            }
            None => Ok(None),
        };
    }

    // ENTRY -> TAGS COMMENT?
    fn parse(&mut self) -> Result<Entry, ParseError> {
        let tags = self.parse_tags()?;
        let comment = self.parse_comment()?;
        return Ok(Entry { tags, comment });
    }
}

impl Entry {
    pub fn from_string(input: &str) -> Result<Entry, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse()
    }

    pub fn to_string(&self) -> String {
        let tags: Vec<String> = self
            .tags
            .iter()
            .map(|tag| {
                let mut s = String::new();
                s.push_str(&tag.name);
                for prop in &tag.val {
                    s.push_str(" ");
                    s.push_str(&prop.name);
                    if prop.val.is_some() {
                        s.push_str("=");
                        s.push_str(prop.val.as_ref().unwrap().as_str());
                    }
                }
                s
            })
            .collect();
        let mut s: String = tags.join(". ");
        if self.comment.is_some() {
            s.push_str(". ");
            s.push_str(self.comment.as_ref().unwrap());
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        #[rustfmt::skip]
        let cases = vec![
            ("tag1", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![]},
            ]}),
            ("tag1 prop1", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
            ]}),
            ("tag1 prop1 val1", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: Some("val1".to_string())}]},
            ]}),
            ("tag1 prop1. tag2", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
                Tag{name: "tag2".to_string(), val: vec![]},
            ]}),
            ("tag1 prop1. tag2 prop2=val2", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
                Tag{name: "tag2".to_string(), val: vec![Prop{name: "prop2".to_string(), val: Some("val2".to_string())}]},
            ]}),
            ("tag1 prop1. tag2 prop2=val2. tag3", Entry{comment: None, tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
                Tag{name: "tag2".to_string(), val: vec![Prop{name: "prop2".to_string(), val: Some("val2".to_string())}]},
                Tag{name: "tag3".to_string(), val: vec![]},
            ]}),
            ("tag1 prop1. tag2 prop2=val2. Comment here we are", Entry{comment: Some("Comment here we are".to_string()), tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
                Tag{name: "tag2".to_string(), val: vec![Prop{name: "prop2".to_string(), val: Some("val2".to_string())}]},
            ]}),
            (" tag1  prop1.  tag2  prop2= val2.   Comment here we are  ", Entry{comment: Some("Comment here we are".to_string()), tags: vec![
                Tag{name: "tag1".to_string(), val: vec![Prop{name: "prop1".to_string(), val: None}]},
                Tag{name: "tag2".to_string(), val: vec![Prop{name: "prop2".to_string(), val: Some("val2".to_string())}]},
            ]}),
        ];
        for (input, want) in cases {
            let got = Entry::from_string(input).unwrap();
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
        ];
        for input in cases {
            let entry = Entry::from_string(input).unwrap();
            let decoded = entry.to_string();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn normalising() {
        #[rustfmt::skip]
        let cases = vec![
            // Spaces trimmed and collapsed
            ("  tag1  ", "tag1"), (" tag1  prop1   ", "tag1 prop1"),
            // Comments are trimmed but comment content is not touched
            (" tag1  prop1  . Comment >>  |  <<  ", "tag1 prop1. Comment >>  |  <<"),
            // Properties values has equal sign before
            (" tag1 prop1 val1 prop2 val2 prop3", "tag1 prop1=val1 prop2=val2 prop3"),
        ];
        for (input, normalised) in cases {
            let got = Entry::from_string(input).unwrap();
            assert_eq!(got.to_string(), normalised);
        }
    }

    #[test]
    fn parsing_errors() {
        #[rustfmt::skip]
        let cases = vec![
            ("  .", ParseError::NoTags),
            ("tag1. tag1", ParseError::Duplicate("Duplicate tag: tag1".to_string(), 10)),
            ("tag1 prop1=a prop1=b", ParseError::Duplicate("Duplicate prop: prop1".to_string(), 20)),
        ];
        for (input, expected) in cases {
            let got = Entry::from_string(input).err().unwrap();
            assert_eq!(got, expected);
        }
    }
}
