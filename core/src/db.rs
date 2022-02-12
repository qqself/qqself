use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use datetime::{DateTimeRange, TimeDuration};
use goal::{Goal, GoalProgress};
use parser::{ParseError, Parser};
use record::{Entry, PropVal, Record, Tag};

// Parsed collection of all active entries and goals
pub struct DB {
    entries: Vec<Entry>,
    goals: Vec<Goal>,
}

// To query entries filtered by certain conditions
#[derive(PartialEq, Debug, Clone)]
pub struct Query {
    pub query: Vec<Tag>,
    pub date_filter: Option<DateTimeRange>,
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.date_filter {
            None => f.write_str("None. ")?,
            Some(filter) => f.write_str(&filter.to_string())?,
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
    pub fn new(query: &str, date_filter: Option<DateTimeRange>) -> Result<Query, ParseError> {
        let mut parser = Parser::new(query);
        let query = parser.parse_query()?;
        Ok(Query { query, date_filter })
    }

    fn matched_tags(&self, entry: &Entry) -> Vec<Tag> {
        // Check first for date limits
        if let Some(filter) = &self.date_filter {
            if entry.date_range.start < filter.start || entry.date_range.end > filter.end {
                return vec![];
            }
        }
        // We consider query tags as part of OR statements and entry
        // is matched if any of the tags matches, which matched if any of props matches.
        // Probably should optimize it as it's quadratic and inside prop matching is
        // quadratic as well. On the other hand usually we have 1-2 tags with 1-2 props
        let mut tags = Vec::new();
        for query_tag in &self.query {
            for entry_tag in &entry.tags {
                if entry_tag.matches(query_tag) {
                    tags.push(entry_tag.clone());
                }
            }
        }
        tags
    }
}

#[derive(Debug, PartialEq)]
pub struct TagStats {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl TagStats {
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn duration(&self) -> TimeDuration {
        let mut duration = TimeDuration::default();
        for entry in &self.entries {
            duration += entry.date_range.duration()
        }
        duration
    }

    pub fn prop_totals(&self) -> Vec<(String, PropVal)> {
        let mut stats = HashMap::new();
        for entry in &self.entries {
            for tag in &entry.tags {
                if tag.name != self.name {
                    continue;
                }
                for prop in &tag.props {
                    if !stats.contains_key(&prop.name) {
                        match prop.val {
                            PropVal::Time(_) | PropVal::Number(_) => {
                                stats.insert(prop.name.clone(), prop.val.clone());
                            }
                            _ => (),
                        }
                    } else {
                        stats
                            .entry(prop.name.clone())
                            .and_modify(|v| *v += prop.val.clone());
                    }
                }
            }
        }
        let mut stats: Vec<(String, PropVal)> = stats.into_iter().collect();
        stats.sort_by_key(|v| v.0.clone());
        stats
    }
}

// Query execution error
#[derive(Debug)]
pub enum QueryError {
    BadQuery(String),
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

    pub fn query(&self, query: Query) -> Result<Vec<TagStats>, QueryError> {
        let mut tags = Vec::new();
        for entry in &self.entries {
            for tag in query.matched_tags(entry) {
                tags.push((tag, entry.clone()));
            }
        }
        Ok(DB::group_by(tags)
            .iter()
            .map(|(name, entries)| {
                let mut entries = entries.clone();
                entries.sort_by(|a, b| a.date_range.start.cmp(&b.date_range.start));
                TagStats {
                    name: name.clone(),
                    entries,
                }
            })
            .collect())
    }

    pub fn goal_progress(
        &self,
        date_filter: DateTimeRange,
    ) -> Result<Vec<GoalProgress>, QueryError> {
        let mut progress = Vec::new();
        for goal in &self.goals {
            if goal.canceled {
                continue; // TODO Probably DB shouldn't even have cancelled goals
            }
            progress.push(goal.goal_progress(self.query(Query {
                query: goal.query.query.clone(),
                date_filter: Some(date_filter.clone()),
            })?));
        }
        Ok(progress)
    }

    fn group_by(mut tags: Vec<(Tag, Entry)>) -> Vec<(String, Vec<Entry>)> {
        tags.sort_by_key(|t| t.0.name.clone());
        let mut groups = Vec::new();
        let mut group = Vec::new();
        let mut group_name = String::new();
        for (tag, entry) in &tags {
            if tag.name != group_name && !group.is_empty() {
                groups.push((group_name.clone(), group));
                group = Vec::new();
            }
            group_name = tag.name.clone();
            group.push(entry.clone());
        }
        if !group.is_empty() {
            groups.push((group_name, group));
        }
        groups
    }
}

impl Default for DB {
    fn default() -> Self {
        DB::new()
    }
}

#[cfg(test)]
mod tests {
    use datetime::{Date, DateTime, DayTime, TimeDuration};

    use super::*;

    const BASE_DATE: Date = Date::new(2000, 1, 1);
    const BASE_TIME: &str = "00:01"; // Hardcode time for easier testing

    fn db_from_strings(records: Vec<&str>) -> DB {
        let mut db = DB::default();
        for record in records {
            match Record::from_string(
                &format!("{} 00:00 {} {} {}", BASE_DATE, BASE_DATE, BASE_TIME, record),
                BASE_DATE,
                None,
            ) {
                Ok(record) => db.add(record),
                _ => unreachable!(),
            }
        }
        db
    }

    fn parse_entries(entries: Vec<(&str, &str)>) -> Vec<TagStats> {
        entries
            .iter()
            .map(|(name, entry)| {
                match Record::from_string(
                    &format!("{} 00:00 {} {} {}", BASE_DATE, BASE_DATE, BASE_TIME, entry),
                    BASE_DATE.clone(),
                    None,
                ) {
                    Ok(Record::Entry(entry)) => TagStats {
                        name: name.to_string(),
                        entries: vec![entry],
                    },
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    #[test]
    fn db_query() {
        let cases = vec![
            // Query by tag
            (vec!["tag1", "tag2"], "tag1", vec![("tag1", "tag1")]),
            // Query by prop
            (
                vec!["tag1", "tag1 prop1"],
                "tag1 prop1",
                vec![("tag1", "tag1 prop1")],
            ),
            // Query by prop val
            (
                vec!["tag1", "tag1 prop1=val1 prop2=val2", "tag1 prop1=val2"],
                "tag1 prop1=val1",
                vec![("tag1", "tag1 prop1=val1 prop2=val2")],
            ),
            // Query works as OR statement
            (
                vec!["tag1", "tag2", "tag3"],
                "tag1. tag2",
                vec![("tag1", "tag1"), ("tag2", "tag2")],
            ),
            // Query less number
            (
                vec!["tag1 prop1=10", "tag1 prop1=20"],
                "tag1 prop1<15",
                vec![("tag1", "tag1 prop1=10")],
            ),
            // Query more duration
            (
                vec!["tag1 prop1=10:00", "tag1 prop1=20:00"],
                "tag1 prop1<15:00",
                vec![("tag1", "tag1 prop1=10:00")],
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

    #[test]
    fn tagstats() {
        let db = db_from_strings(vec![
            "run distance=9.33. Fine",
            "exercises plank=02:20 pullups=20 intensity=high",
            "run distance=3.44. OK",
            "exercises plank=3:20 pullups=20 tired",
        ]);
        // Simple one
        let query = Query::new("run", None).unwrap();
        let results = db.query(query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].count(), 2);
        assert_eq!(results[0].duration(), TimeDuration::new(0, 2));
        assert_eq!(
            results[0].prop_totals(),
            vec![("distance".to_string(), PropVal::Number(12.77))]
        );
        // Multiple props, duration one and text
        let query = Query::new("exercises", None).unwrap();
        let results = db.query(query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].count(), 2);
        assert_eq!(results[0].duration(), TimeDuration::new(0, 2));
        assert_eq!(
            results[0].prop_totals(),
            vec![
                ("plank".to_string(), PropVal::Time(TimeDuration::new(5, 40))),
                ("pullups".to_string(), PropVal::Number(40.0)),
            ]
        );
    }
}
