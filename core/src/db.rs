use std::fmt::{Display, Formatter};

use datetime::DateTime;
use parser::{ParseError, Parser};
use record::{Entry, Goal, Record, Tag};

// Parsed collection of all active entries and goals
pub struct DB {
    entries: Vec<Entry>,
    goals: Vec<Goal>,
}

// To query entries filtered by certain conditions
#[derive(PartialEq, Debug)]
pub struct Query {
    pub query: Vec<Tag>,
    pub date_filter: Option<FilterDate>,
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.date_filter {
            None => f.write_str("None. ")?,
            Some(filter) => f.write_fmt(format_args!("{} {}. ", filter.min, filter.max))?,
        }
        for tag in &self.query {
            f.write_str(&tag.to_string())?;
        }
        std::fmt::Result::Ok(())
    }
}

impl Default for Query {
    fn default() -> Self {
        Query {
            query: vec![],
            date_filter: None,
        }
    }
}

impl Query {
    fn new(query: &str, date_filter: Option<FilterDate>) -> Result<Query, ParseError> {
        let mut parser = Parser::new(query);
        let query = parser.parse_query()?;
        Ok(Query { query, date_filter })
    }

    fn matches(&self, entry: &Entry) -> bool {
        // Check first for date limits
        if let Some(filter) = &self.date_filter {
            if entry.date_range.start < filter.min || entry.date_range.end > filter.max {
                return false;
            }
        }
        // We consider query tags as part of OR statements and entry
        // is matched if any of the tags matches, which matched if any of props matches.
        // Probably should optimize it as it's quadratic and inside prop matching is
        // quadratic as well. On the other hand usually we have 1-2 tags with 1-2 props
        for query_tag in &self.query {
            for entry_tag in &entry.tags {
                if entry_tag.matches(query_tag) {
                    return true;
                }
            }
        }
        false
    }
}

// Entries filter by date
#[derive(PartialEq, Debug)]
pub struct FilterDate {
    pub min: DateTime,
    pub max: DateTime,
}

// Query execution error
pub enum QueryError {
    BadQuery(String),
}

// Goal progress stats
pub struct GoalProgress {
    pub name: String,
    pub completion: usize,
    pub minutes_actual: usize,
    pub minutes_planned: usize,
}

impl DB {
    pub fn new() -> Self {
        DB {
            entries: vec![],
            goals: vec![],
        }
    }

    // Add new record to database
    pub fn add(&mut self, record: Record) {
        match record {
            Record::Entry(entry) => self.entries.push(entry),
            Record::Goal(goal) => self.goals.push(goal),
        }
    }

    pub fn query(&self, query: Query) -> Result<Vec<Entry>, QueryError> {
        let mut entries: Vec<Entry> = Vec::new();
        for entry in &self.entries {
            if query.matches(entry) {
                entries.push(entry.clone());
            }
        }
        Ok(entries)
    }

    pub fn goal_progress(&self, filter: FilterDate) -> Vec<GoalProgress> {
        vec![]
    }
}

impl Default for DB {
    fn default() -> Self {
        DB::new()
    }
}

#[cfg(test)]
mod tests {
    use datetime::{Date, Time};

    use super::*;

    const BASE_DATE: DateTime = DateTime::new(Date::new(2000, 1, 1), Time::new(0, 0));
    const BASE_TIME: &str = "00:01"; // Hardcode time for easier testing

    fn db_from_strings(records: Vec<&str>) -> DB {
        let mut db = DB::default();
        for record in records {
            match Record::from_string(&format!("{} {}", BASE_TIME, record), BASE_DATE.clone()) {
                Ok(record) => db.add(record),
                _ => unreachable!(),
            }
        }
        db
    }

    fn parse_entries(entries: Vec<&str>) -> Vec<Entry> {
        entries
            .iter()
            .map(|v| {
                match Record::from_string(&format!("{} {}", BASE_TIME, v), BASE_DATE.clone()) {
                    Ok(Record::Entry(entry)) => entry,
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<Entry>>()
    }

    #[test]
    fn db_query() {
        let cases = vec![
            // Query by tag
            (vec!["tag1", "tag2"], "tag1", vec!["tag1"]),
            // Query by prop
            (vec!["tag1", "tag1 prop1"], "tag1 prop1", vec!["tag1 prop1"]),
            // Query by prop val
            (
                vec!["tag1", "tag1 prop1=val1 prop2=val2", "tag1 prop1=val2"],
                "tag1 prop1=val1",
                vec!["tag1 prop1=val1 prop2=val2"],
            ),
            // Query works as OR statement
            (
                vec!["tag1", "tag2", "tag3"],
                "tag1. tag2",
                vec!["tag1", "tag2"],
            ),
            // Query less number
            (
                vec!["tag1 prop1=10", "tag1 prop1=20"],
                "tag1 prop1<15",
                vec!["tag1 prop1=10"],
            ),
            // Query more duration
            (
                vec!["tag1 prop1=10:00", "tag1 prop1=20:00"],
                "tag1 prop1<15:00",
                vec!["tag1 prop1=10:00"],
            ),
        ];
        for (records, query, results) in cases {
            let want = parse_entries(results);
            let db = db_from_strings(records);
            let query = Query::new(query, None).unwrap();
            match db.query(query) {
                Ok(got) => assert_eq!(got, want),
                _ => unreachable!(),
            }
        }
    }
}
