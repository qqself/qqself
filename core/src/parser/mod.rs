use std::collections::HashMap;
use std::time::Duration;

use parser::goal::Goal;
use parser::parser::{DateRange, Entry, ParseError, Parser, Tag};

mod goal;
mod parser;

// Record represent parsed text line
#[derive(Debug, PartialEq)]
pub enum Record {
    // Entry record - new data input
    Entry(Entry),

    // Goal record - new goal set or canceled
    Goal(Goal),
}

// Query for finding matching entries
pub struct Query {
    query: Vec<Tag>,
    entries: Vec<Entry>,
}

impl Query {
    pub fn new(query: &str) -> Result<Query, ParseError> {
        // TODO Parser should be able to parse query as collection of tags
        //      without date range and comment. For now hack it with extra text
        let fixed_query = format!("2000-01-01 00:00 {}", query);
        let mut parser = Parser::new(&fixed_query);
        match parser.parse("", "") {
            Ok(record) => match record {
                Record::Entry(entry) => Ok(Query {
                    query: entry.tags,
                    entries: vec![],
                }),
                _ => Err(ParseError::BadQuery("Not an entry".to_string(), 0)),
            },
            Err(err) => Err(ParseError::BadQuery(err.to_string(), 0)),
        }
    }

    fn is_match(&self, tag: &Tag) -> bool {
        // TODO Check also properties: comparison operators, type parsing, etc.
        for tag_query in self.query.iter() {
            if tag_query.name == tag.name {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, entry: Entry) -> bool {
        // Entry and entries likely to have 2-3 tags most of the time, so O(n*m) is fine here
        for tag in entry.tags.iter() {
            if self.is_match(tag) {
                self.entries.push(entry);
                return true;
            }
        }
        false
    }

    pub fn render_stats(&self) {
        let mut tags: HashMap<String, Vec<(&Tag, &DateRange)>> = HashMap::new();
        for entry in &self.entries {
            for tag in &entry.tags {
                if !self.is_match(tag) {
                    continue;
                }
                if tags.contains_key(&tag.name) {
                    tags.get_mut(&tag.name)
                        .unwrap()
                        .push((tag, &entry.date_range));
                } else {
                    tags.insert(tag.name.clone(), vec![(tag, &entry.date_range)]);
                }
            }
        }
        for (name, tags) in tags.iter() {
            let mut duration = Duration::from_secs(0);
            for (_, range) in tags {
                duration += range.duration();
            }
            let duration_hours = duration.as_secs() / 60 / 60;
            let duration_minutes = (duration.as_secs() - duration_hours * 60 * 60) / 60;
            println!(
                "{:<10} \t Count {:?} \t Duration {:0>2}:{:0>2}",
                name,
                tags.len(),
                duration_hours,
                duration_minutes,
            );
        }
    }
}

impl Record {
    pub fn from_string(
        input: &str,
        time_prefix: &str,
        previous_time_end: &str,
    ) -> Result<Record, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse(time_prefix, previous_time_end)
    }
}
