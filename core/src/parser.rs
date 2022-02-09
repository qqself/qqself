use std::cmp::min;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use datetime::{Date, DatePeriod, DateTime, DateTimeRange, Time};
use record::{Aggregate, Count, Entry, Goal, Prop, PropOperator, Record, Tag};

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
    fn parse_date_range(&mut self, base_time: DateTime) -> Result<DateTimeRange, ParseError> {
        let parsed_date = match self.parse_date_time(&base_time.date)? {
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
        match self.parse_date_time(&base_time.date)? {
            Some(date_to) => Ok(DateTimeRange {
                start: parsed_date,
                end: date_to,
            }),
            None => Ok(DateTimeRange {
                start: base_time,
                end: parsed_date,
            }),
        }
    }

    // DATETIME -> (DATE' ')? TIME
    fn parse_date_time(&mut self, base_date: &Date) -> Result<Option<DateTime>, ParseError> {
        let date = self.parse_date().unwrap_or_else(|| base_date.clone());
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
        match self.input[self.pos..min(self.pos + date_len, self.input.len() - 1)].parse::<Date>() {
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
        match self.input[self.pos..min(self.pos + time_len, self.input.len() - 1)].parse::<Time>() {
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
    pub fn parse_record(&mut self, relevant_to: DateTime) -> Result<Record, ParseError> {
        let date_range = self.parse_date_range(relevant_to)?;
        let tags = self.parse_tags()?;
        let comment = self.parse_comment();
        self.create_record(date_range, tags, comment)
    }

    pub fn parse_query(&mut self) -> Result<Vec<Tag>, ParseError> {
        self.parse_tags()
    }

    fn create_record(
        &self,
        date_range: DateTimeRange,
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

// TODO Whole Goal is copy pasted as is from PoC phase, refactor it

impl Goal {
    pub fn is_valid(&self) -> bool {
        !self.canceled
    }

    pub fn name(&self) -> String {
        match &self.comment {
            None => self.to_string(),
            Some(comment) => comment.clone(),
        }
    }

    pub fn target_in_days(&self, days_count: usize) -> (usize, Option<usize>) {
        let mut target = (0, None);
        let days_count = days_count as f32;
        let divider = match self.period {
            // TODO A lot of rounding errors here
            DatePeriod::Day => 1.0 / days_count,
            DatePeriod::Week => 7.0 / days_count,
            DatePeriod::Month => 30.0 / days_count,
            DatePeriod::Year => 365.0 / days_count,
        };
        target.0 = self
            .count
            .as_ref()
            .map(|v| {
                v.max
                    .map(|v| ((v as f32) * divider) as usize)
                    .unwrap_or_else(|| {
                        v.min
                            .map(|v| (v as f32 * divider) as usize)
                            .unwrap_or_default()
                    })
            })
            .unwrap_or_default();
        target.1 = self
            .duration
            .as_ref()
            .map(|v| (v.as_minutes() as f32 / divider) as usize);
        target
    }

    pub fn create(tags: Vec<Tag>, comment: Option<String>) -> Result<Goal, ParseError> {
        let str: Vec<String> = tags.iter().map(|t| format!("{}", t)).collect();
        let str = str.join(" ");
        let mut goal = Goal {
            aggregate: Aggregate::Sum,
            canceled: false,
            comment,
            count: Option::None,
            duration: Option::None,
            period: DatePeriod::Day,
            properties: vec![],
            query: Default::default(),
            str,
        };
        for tag in tags {
            if tag.name == "goal" {
                for prop in tag.val {
                    match prop.name.as_ref() {
                        "type" => goal.aggregate = Goal::parse_aggregate(prop)?,
                        "cancelled" => goal.canceled = true,
                        "min" => {
                            if goal.count.is_none() {
                                goal.count = Some(Count {
                                    min: None,
                                    max: None,
                                })
                            }
                            goal.count.as_mut().unwrap().min = Some(Goal::parse_count(prop)?);
                        }
                        // TODO Remove duplication
                        "max" => {
                            if goal.count.is_none() {
                                goal.count = Some(Count {
                                    min: None,
                                    max: None,
                                })
                            }
                            goal.count.as_mut().unwrap().max = Some(Goal::parse_count(prop)?);
                        }
                        "duration" => goal.duration = Goal::parse_duration(prop)?,
                        "for" => goal.period = Goal::parse_period(prop)?,
                        _ => goal.properties.push(prop),
                    }
                }
            } else {
                goal.query.query.push(tag);
            }
        }
        if goal.query.query.is_empty() {
            return Err(ParseError::BadQuery(
                "Query is required for the goal".to_string(),
                0,
            ));
        }
        Ok(goal)
    }

    pub fn is_relevant_entry(&self, entry: &Entry) -> bool {
        for tag_entry in &entry.tags {
            for tag_query in &self.query.query {
                if tag_entry.name == tag_query.name {
                    let tag_entry: &Tag = tag_entry;
                    let entry_props: HashMap<_, _> =
                        tag_entry.val.iter().map(|v| (v.name.clone(), v)).collect();
                    let tag_query: &Tag = tag_query;
                    let mut match_found = true;
                    for prop in &tag_query.val {
                        // TODO: Handle any other operators
                        assert_eq!(prop.operator, PropOperator::Eq);
                        if !entry_props.contains_key(&prop.name) {
                            match_found = false;
                            break;
                        }
                        let entry_prop = entry_props[&prop.name];
                        assert_eq!(entry_prop.operator, PropOperator::Eq);
                        if prop.val != entry_prop.val {
                            match_found = false;
                            break;
                        }
                    }
                    if match_found {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn parse_count(prop: Prop) -> Result<usize, ParseError> {
        let err = "goal min/max value should be an integer";
        if prop.val.is_none() {
            return Err(ParseError::BadValue(err.to_string(), prop.start_pos));
        }
        return match prop.val.as_ref().unwrap().parse::<usize>() {
            Ok(index) => Ok(index),
            Err(_) => Err(ParseError::BadValue(err.to_string(), prop.start_pos)),
        };
    }

    fn parse_aggregate(prop: Prop) -> Result<Aggregate, ParseError> {
        if prop.val.is_none() {
            return Ok(Aggregate::Sum);
        }
        match prop
            .val
            .as_ref()
            .unwrap()
            .to_lowercase()
            .parse::<Aggregate>()
        {
            Ok(aggregate) => Ok(aggregate),
            Err(err) => Err(ParseError::BadValue(err, prop.start_pos)),
        }
    }

    fn parse_duration(prop: Prop) -> Result<Option<Time>, ParseError> {
        match prop.val.map(|v| v.parse::<Time>()) {
            None => Ok(None),
            Some(Ok(time)) => Ok(Some(time)),
            Some(Err(err)) => Err(ParseError::BadQuery(err, prop.start_pos)),
        }
    }

    fn parse_period(prop: Prop) -> Result<DatePeriod, ParseError> {
        match prop.val.map(|v| v.parse::<DatePeriod>()) {
            None => {
                let err = "`of` property is required for `goal` tag".to_string();
                Err(ParseError::MissingProperty(err, prop.start_pos))
            }
            Some(Ok(date_period)) => Ok(date_period),
            Some(Err(_)) => {
                let err = "`of` property value can be either week, month or year".to_string();
                Err(ParseError::BadValue(err, prop.start_pos))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_DATE: DateTime = DateTime::new(Date::new(2000, 1, 1), Time::new(0, 0));

    #[test]
    fn entry_parsing() {
        let dr = |hours: u8, minutes: u8| DateTimeRange {
            start: BASE_DATE.clone(),
            end: DateTime::new(BASE_DATE.date.clone(), Time::new(hours, minutes)),
        };
        let prop = |name: &str, val: Option<&str>| Prop {
            name: name.to_string(),
            val: val.map(|v| v.to_string()),
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
                Entry::new(dr(1, 1), None, vec![tag("tag1", vec![prop("prop1", None)])]),
            ),
            (
                // Simple tag, simple prop and val
                "01:01 tag1 prop1 val1",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag("tag1", vec![prop("prop1", Some("val1"))])],
                ),
            ),
            (
                // Multiple tags
                "01:01 tag1 prop1. tag2",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![tag("tag1", vec![prop("prop1", None)]), tag("tag2", vec![])],
                ),
            ),
            (
                // Multiple tags with props and vals
                "01:01 tag1 prop1. tag2 prop2=val2",
                Entry::new(
                    dr(1, 1),
                    None,
                    vec![
                        tag("tag1", vec![prop("prop1", None)]),
                        tag("tag2", vec![prop("prop2", Some("val2"))]),
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
                        tag("tag1", vec![prop("prop1", Some("val2"))]),
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
                            vec![prop("prop1", Some("9.5")), prop("prop2", Some("6.33"))],
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
                    vec![tag("tag1", vec![prop("prop1", None)])],
                ),
            ),
            (
                // Simple tag, simple prop and a multiline comment
                "01:01 tag1 prop1. Line1\nLine2",
                Entry::new(
                    dr(1, 1),
                    Some("Line1\nLine2".to_string()),
                    vec![tag("tag1", vec![prop("prop1", None)])],
                ),
            ),
        ];
        for (input, want) in cases {
            match Record::from_string(input, BASE_DATE.clone()) {
                Ok(Record::Entry(mut entry)) => {
                    // Ignore start_pos for parsed entry to make testing easier
                    for i in 0..entry.tags.len() {
                        entry.tags[i].start_pos = 0;
                        for y in 0..entry.tags[i].val.len() {
                            entry.tags[i].val[y].start_pos = 0;
                        }
                    }
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
            let input = format!("{} - {} 01:01 {}", BASE_DATE, BASE_DATE.date, input);
            if let Record::Entry(entry) = Record::from_string(&input, BASE_DATE.clone()).unwrap() {
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
            ("01:01  tag1  ", "2000-01-01 00:00 - 2000-01-01 01:01 tag1"),
            (
                "01:01 tag1  prop1   ",
                "2000-01-01 00:00 - 2000-01-01 01:01 tag1 prop1",
            ),
            // Comments are trimmed but comment content is not touched
            (
                "01:01 tag1  prop1  . Comment >>  |  <<  ",
                "2000-01-01 00:00 - 2000-01-01 01:01 tag1 prop1. Comment >>  |  <<",
            ),
            // Properties values has equal signs
            (
                "01:01 tag1 prop1 val1 prop2 val2 prop3",
                "2000-01-01 00:00 - 2000-01-01 01:01 tag1 prop1=val1 prop2=val2 prop3",
            ),
            // Multiline strings
            (
                "01:01 tag1. Multiline\n- Some1\n\n-Some2 ",
                "2000-01-01 00:00 - 2000-01-01 01:01 tag1. Multiline\\n- Some1\\n\\n-Some2",
            ),
        ];
        for (input, normalised) in cases {
            if let Record::Entry(got) = Record::from_string(input, BASE_DATE.clone()).unwrap() {
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
            let got = Record::from_string(input, BASE_DATE.clone()).err().unwrap();
            assert_eq!(got, expected);
        }
    }
}

// TODO We've refactored parser a lot, and those tests are not valid anymore. We anyway
//      going to rewrite Goal from scratch, so let's leave those tests as functional requirements
// #[cfg(test)]
// mod tests {
//     use parser::parser::PropOperator;
//     use parser::Record;
//
//     use super::*;
//
//     fn parse(s: &str) -> Goal {
//         // Hack Goal is an entry still and requires time
//         let s = format!("2000-01-01 00:00 {}", s);
//         match Record::from_string(&s, "", "") {
//             Ok(record) => match record {
//                 Record::Entry(_) => unreachable!("Goal is expected"),
//                 Record::Goal(goal) => goal,
//             },
//             Err(err) => unreachable!("Valid goal is expected: {:?}", err),
//         }
//     }
//
//     #[test]
//     fn parse_simple() {
//         let goal = parse("run distance>50. goal for week duration=5:00");
//         assert_eq!(goal.duration, Some((5, 0)));
//         assert_eq!(goal.period, Period::Week);
//         assert_eq!(
//             goal.query,
//             vec![Tag {
//                 name: "run".to_string(),
//                 val: vec![Prop {
//                     name: "distance".to_string(),
//                     val: Some("50".to_string()),
//                     operator: PropOperator::More,
//                     start_pos: 20,
//                 }],
//                 start_pos: 17
//             }]
//         );
//     }
//
//     #[test]
//     fn parse_min_and_custom() {
//         let goal = parse("foo. goal for month duration=10:30 min=3 distance=50");
//         assert_eq!(goal.duration, Some((10, 30)));
//         assert_eq!(goal.period, Period::Month);
//         assert_eq!(goal.count.unwrap().min, Some(3));
//         assert_eq!(
//             goal.properties,
//             vec![Prop {
//                 name: "distance".to_string(),
//                 val: Some("50".to_string()),
//                 operator: PropOperator::Eq,
//                 start_pos: 57,
//             }]
//         );
//     }
//
//     #[test]
//     fn parse_just_count() {
//         let goal = parse("foo. goal for year max=10");
//         assert_eq!(goal.period, Period::Year);
//         assert_eq!(goal.duration, None);
//         assert_eq!(goal.count.unwrap().max, Some(10));
//     }
//
//     #[test]
//     fn parse_aggregate() {
//         let goal = parse("foo. goal for year income=1000 type=last");
//         assert_eq!(goal.aggregate, Aggregate::Last);
//     }
//
//     #[test]
//     fn parse_cancelled() {
//         let goal = parse("foo. goal for year cancelled");
//         assert!(goal.canceled);
//     }
//
//     #[test]
//     fn parse_comment() {
//         let goal = parse("foo. goal for week. New goal");
//         assert_eq!(goal.comment, Some("New goal".to_string()));
//     }
// }
