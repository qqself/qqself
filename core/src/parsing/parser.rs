use std::{
    iter::{Peekable, Zip},
    slice::Iter,
    str::Chars,
};

use thiserror::Error;

use crate::{
    date_time::datetime::DateTimeRange,
    record::{Entry, Prop, PropOperator, PropVal, Tag},
};

use super::tokenizer::{Token, Tokenizer};

type InputIterator<'a> = Peekable<Zip<Chars<'a>, Iter<'a, Token>>>;

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

// TODO TokenParser error handling is shallow intentionally to follow the legacy `Parser`
//      Once we fully migrate to the new parser we will improve error reporting a lot
impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub(crate) fn parse_date_record(&mut self) -> Result<Entry, ParseError> {
        let tokens = Tokenizer::new(self.input, true);
        if let Some(err) = tokens.error {
            match err {
                super::tokenizer::TokenizingError::TagsNotFound(_) => {
                    return Err(ParseError::NoTags)
                }
                super::tokenizer::TokenizingError::Expected(_, _, _) => {}
                super::tokenizer::TokenizingError::DateOrTimeExpected(_) => {}
            }
        }
        self.entry_from_tokens(&tokens.tokens)
    }

    pub(crate) fn parse_record(&mut self) -> Result<(Vec<Tag>, Option<String>), ParseError> {
        todo!()
    }

    fn entry_from_tokens(&mut self, tokens: &[Token]) -> Result<Entry, ParseError> {
        let mut iter = self.input.chars().zip(tokens).peekable();
        let date_range = self.parse_daterange(&mut iter)?;
        let tags = self.parse_tags(&mut iter)?;
        let comment = self.parse_comment(&mut iter);
        Ok(Entry::new(date_range, comment, tags))
    }

    fn parse_daterange(&mut self, iter: &mut InputIterator) -> Result<DateTimeRange, ParseError> {
        let date_range_input = self.read_while(
            iter,
            &[
                Token::Date,
                Token::DateSeparator,
                Token::Space,
                Token::Time,
                Token::TimeSeparator,
            ],
        );
        // Plus/Minus one is for the extra space at the end
        if date_range_input.len() != DateTimeRange::SIZE_LONG + 1
            && date_range_input.len() != DateTimeRange::SIZE_SHORT + 1
        {
            return Err(ParseError::BadDateTime(
                "Failed to parse the date because of unexpected string length".to_string(),
                date_range_input.len(),
            ));
        }
        date_range_input[0..date_range_input.len() - 1]
            .parse::<DateTimeRange>()
            .map_err(|err| ParseError::BadDateTime(err, date_range_input.len()))
    }

    fn parse_tags(&mut self, iter: &mut InputIterator) -> Result<Vec<Tag>, ParseError> {
        let mut tags = Vec::new();
        let mut names = Vec::new();
        loop {
            self.read_while(iter, &[Token::Space, Token::TagSeparator]);
            let name = self.read_while(iter, &[Token::TagName]);
            if name.is_empty() {
                break;
            }
            if names.contains(&name) {
                return Err(ParseError::Duplicate(format!("tag '{}'", name), self.pos));
            }
            names.push(name.clone());
            let props = self.parse_props(iter)?;
            tags.push(Tag {
                name,
                props,
                start_pos: 0,
            })
        }
        Ok(tags)
    }

    fn parse_props(&mut self, iter: &mut InputIterator) -> Result<Vec<Prop>, ParseError> {
        let mut props = Vec::new();
        let mut names = Vec::new();
        loop {
            self.read_while(iter, &[Token::Space]);
            let name = self.read_while(iter, &[Token::PropertyName]);
            if name.is_empty() {
                break;
            }
            if names.contains(&name) {
                return Err(ParseError::Duplicate(
                    format!("property '{}'", name),
                    self.pos,
                ));
            }
            names.push(name.clone());
            self.read_while(iter, &[Token::Space, Token::PropertyOperator]);
            let val = self.read_while(iter, &[Token::PropertyValue]);
            // Property value may be surrounded with the quotes, remove those as unnecessary noise
            let val = val.trim_matches('"').to_string();
            props.push(Prop {
                name,
                val: PropVal::parse(val),
                operator: PropOperator::Eq,
                start_pos: 0, // TODO Remove
            })
        }
        Ok(props)
    }

    fn parse_comment(&mut self, iter: &mut InputIterator) -> Option<String> {
        self.read_while(iter, &[Token::Space]);
        let comment = self.read_while(iter, &[Token::Comment]);
        if comment.is_empty() {
            None
        } else {
            Some(comment)
        }
    }

    /// Similar to standard take_while which consumes first non match, while this one doesn't via peekable check
    fn read_while(&mut self, iter: &mut InputIterator, predicate: &[Token]) -> String {
        let mut out = String::new();
        while iter.peek().filter(|v| predicate.contains(v.1)).is_some() {
            out.push(iter.next().unwrap().0);
            self.pos += 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        date_time::datetime::{DateDay, DateTime, Duration, Time},
        record::{PropOperator, PropVal},
    };

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
                // Short date range, simple tag
                "2000-01-01 01:01 01:01 abc",
                Entry::new(dr(1, 1), None, vec![tag("abc", vec![])]),
            ),
            (
                // Long date range, simple tag
                "2000-01-01 01:01 - 2000-01-01 01:01 foo",
                Entry::new(dr(1, 1), None, vec![tag("foo", vec![])]),
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
                "2000-01-01 01:01 01:01 tag1  prop1 =   val2  .    tag2  . Comment",
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
                // Duration in property values
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
            (
                // Quoted values
                "2000-01-01 01:01 01:01 tag1 prop1=\"Hello.World\". Comment",
                Entry::new(
                    dr(1, 1),
                    Some("Comment".to_string()),
                    vec![tag(
                        "tag1",
                        vec![prop("prop1", PropVal::String("Hello.World".to_string()))],
                    )],
                ),
            ),
        ];
        for (input, want) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse_date_record().unwrap(), want);
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
        ];
        for input in cases {
            let input = format!("2000-01-01 02:02 - 2000-01-02 02:02 {input}");
            let mut parser = Parser::new(&input);
            let entry = parser.parse_date_record().unwrap();
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
                "2000-01-01 01:01 01:02 tag1  prop1   ",
                "2000-01-01 01:01 01:02 tag1 prop1",
            ),
            // Comments are trimmed but comment content is not touched
            (
                "2000-01-01 01:01 01:01 tag1  prop1  . Comment >>  |  <<",
                "2000-01-01 01:01 01:01 tag1 prop1. Comment >>  |  <<",
            ),
            // Properties values has equal signs
            (
                "2000-01-01 01:01 01:01 tag1 prop1 val1 prop2 val2 prop3",
                "2000-01-01 01:01 01:01 tag1 prop1=val1 prop2=val2 prop3",
            ),
            // Multiline strings
            (
                "2000-01-01 01:01 01:01 tag1. Multiline\n- Some1\n\n-Some2",
                "2000-01-01 01:01 01:01 tag1. Multiline\\n- Some1\\n\\n-Some2",
            ),
        ];
        for (input, normalized) in cases {
            let mut parser = Parser::new(input);
            let entry = parser.parse_date_record().unwrap();
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
                ParseError::Duplicate("tag 'tag1'".to_string(), 33),
            ),
            (
                "2010-01-01 01:01 01:01 tag1 prop1=a prop1=b",
                ParseError::Duplicate("property 'prop1'".to_string(), 41),
            ),
            (
                "tag1 prop1=a prop1=b",
                ParseError::BadDateTime(
                    "Failed to parse the date because of unexpected string length".to_string(),
                    0,
                ),
            ),
            (
                "2010-01-01 10:00 09:00 tag1 prop1=a prop1=b",
                ParseError::BadDateTime("end time cannot be before the start".to_string(), 23),
            ),
        ];
        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            let got = parser.parse_date_record().err().unwrap();
            assert_eq!(got, expected);
        }
    }
}
