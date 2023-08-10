use std::collections::{BTreeMap, BTreeSet};

use crate::data_views::query_results::QueryResultsView;
use crate::data_views::skills::{SkillsNotification, SkillsUpdate, SkillsView};
use crate::date_time::datetime::{DateDay, DateTimeRange};
use crate::parsing::parser::{ParseError, Parser};
use crate::progress::skill::Skill;
use crate::record::{Entry, PropVal, Tag};

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum Record {
    Entry(Entry),
    Conflict(BTreeSet<Entry>),
}

impl Record {
    pub fn revision(&self) -> usize {
        match self {
            Record::Entry(v) => v.revision(),
            Record::Conflict(v) => v.first().expect("conflict contains entries").revision(),
        }
    }

    pub fn date_range(&self) -> &DateTimeRange {
        match self {
            Record::Entry(v) => &v.date_range,
            Record::Conflict(v) => &v.first().expect("conflict contains entries").date_range,
        }
    }

    pub fn to_string(&self, include_date: bool, include_entry_tag: bool) -> String {
        match self {
            Record::Entry(entry) => entry.serialize(include_date, include_entry_tag),
            Record::Conflict(conflict) => {
                let entries: Vec<_> = conflict.iter().map(|v| v.serialize(true, true)).collect();
                entries.join("\n")
            }
        }
    }

    pub fn to_deleted_string(&self) -> String {
        format!(
            "{} entry revision={} deleted. Marker that entry for this data range was deleted",
            self.date_range(),
            self.revision() + 1
        )
    }

    pub fn is_deleted_record(&self) -> bool {
        let Record::Entry(entry) = self else { return false };
        for tag in &entry.tags {
            if tag.name == "entry" {
                for prop in &tag.props {
                    if prop.name == "deleted" {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn next_revision(&self) -> Record {
        match self {
            Record::Entry(entry) => {
                let mut entry = entry.clone();
                entry.increase_revision();
                Record::Entry(entry)
            }
            Record::Conflict(conflict) => {
                let mut conflicts = BTreeSet::new();
                for entry in conflict.iter() {
                    let mut entry = entry.clone();
                    entry.increase_revision();
                    conflicts.insert(entry);
                }
                Record::Conflict(conflicts)
            }
        }
    }

    pub fn parse(input: &str) -> Result<Record, String> {
        // There are multiple lines in the entry, it's a conflict
        if input.lines().count() > 1 {
            let mut entries = BTreeSet::new();
            for input in input.lines() {
                let entry = Entry::parse(input).map_err(|err| err.to_string())?;
                entries.insert(entry);
            }
            Ok(Record::Conflict(entries))
        } else {
            let entry = Entry::parse(input).map_err(|err| err.to_string())?;
            Ok(Record::Entry(entry))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeEvent {
    Added(Record),
    Replaced { from: Record, to: Record },
}

/// Emitted when view data got updated and clients need to re-render the view
#[derive(PartialEq, Debug)]
pub enum ViewUpdate {
    QueryResults,
    Skills(SkillsUpdate),
}

/// Emitter when user interactively added a new record and progress notification has
/// to be shown to the user
pub enum Notification {
    Skills(SkillsNotification),
}

// Parsed collection of all active entries and goals
#[derive(Default)]
pub struct DB {
    entries: BTreeMap<DateTimeRange, Record>,
    on_notification: Option<Box<dyn Fn(Notification)>>,
    on_view_update: Option<Box<dyn Fn(ViewUpdate)>>,
    view_query_results: QueryResultsView,
    view_skills: SkillsView,
}

impl DB {
    pub fn new() -> Self {
        DB {
            entries: BTreeMap::new(),
            view_skills: SkillsView::default(),
            view_query_results: QueryResultsView::default(),
            on_view_update: None,
            on_notification: None,
        }
    }

    pub fn skills(&self) -> &BTreeMap<String, Skill> {
        self.view_skills.data()
    }

    pub fn query_results(&self) -> &BTreeSet<Record> {
        self.view_query_results.data()
    }

    /// Adds new record to the DB. Interactively means user is adding a record right now. If records are restored from
    /// cache, fetched from API then it's considered not interactive. In interactive mode user may benefit from
    /// `Notifications`, so those are emitted in case of noticeable progress
    pub fn add(
        &mut self,
        record: Record,
        interactive: bool,
        now: Option<DateDay>,
    ) -> Option<ChangeEvent> {
        let event = self.merge(record);
        if let Some(event) = &event {
            self.view_query_results.update(event, &self.on_view_update);
            self.view_skills.update(
                self.entries.iter(),
                event,
                interactive,
                now,
                &self.on_view_update,
                &self.on_notification,
            );
        }
        event
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    // Merge new record into existing database and return ChangeEvent telling how exactly DB got changed.
    // Implementation may looks a bit complex as we need to handle all possible edge cases when
    // syncing multiple sources. Goal is that after merging all the records the DB will converge to one
    // single possible state, no matter in which order local or remote events were processed
    fn merge(&mut self, record_new: Record) -> Option<ChangeEvent> {
        // Merging rules:
        // - If no record with the same daterange exists - append.
        // - If record with same daterange exists:
        //   - If new record revision is higher - replace existing.
        //   - If revision is the same:
        //     - If both records are conflicts - append records from new conflict to existing
        //     - If both records are values - replace existing record with new created conflict and append both records there
        //     - If existing record is conflict, but new one is value - append new value to conflict entries
        //     - If new one is conflict, but existing record is value - replace existing record with new conflict with added existing record
        let mut record_new = record_new;
        let entry_new_key = record_new.date_range();
        let mut record_old = match self.entries.get_mut(entry_new_key) {
            None => {
                let event = ChangeEvent::Added(record_new.clone());
                self.entries.insert(*entry_new_key, record_new);
                return Some(event); // New record - append
            }
            Some(v) => v,
        };
        if &record_new == record_old {
            return None; // Same records - ignore
        }
        if record_new.revision() > record_old.revision() {
            let event = ChangeEvent::Replaced {
                from: record_old.clone(),
                to: record_new.clone(),
            };
            *record_old = record_new;
            return Some(event); // Newer record - replace
        }
        if record_new.revision() < record_old.revision() {
            return None; // Older record - ignore
        }

        match (&mut record_old, &mut record_new) {
            (Record::Conflict(conflict_old), Record::Conflict(conflict_new)) => {
                let conflict_before = conflict_old.clone();
                // Existing and new are conflicts - merge it's entries
                conflict_old.append(conflict_new);
                Some(ChangeEvent::Replaced {
                    from: Record::Conflict(conflict_before),
                    to: Record::Conflict(conflict_old.clone()),
                })
            }
            (Record::Entry(value_old), Record::Entry(value_new)) => {
                // Two conflicting values - replace existing record with Conflict value and append new entry there
                let value_before = value_old.clone();
                *record_old =
                    Record::Conflict(BTreeSet::from([value_old.clone(), value_new.to_owned()]));
                Some(ChangeEvent::Replaced {
                    from: Record::Entry(value_before),
                    to: record_old.clone(),
                })
            }
            (Record::Conflict(conflict_old), Record::Entry(value_new)) => {
                let conflict_before = conflict_old.clone();
                // Existing conflict - append new entry
                conflict_old.insert(value_new.to_owned());
                Some(ChangeEvent::Replaced {
                    from: Record::Conflict(conflict_before),
                    to: Record::Conflict(conflict_old.clone()),
                })
            }
            (Record::Entry(value_old), Record::Conflict(conflict_new)) => {
                // Existing value, but new conflict, merge to conflict
                let value_before = value_old.clone();
                let mut entries = conflict_new.to_owned();
                entries.insert(value_old.clone());
                *record_old = Record::Conflict(entries);
                Some(ChangeEvent::Replaced {
                    from: Record::Entry(value_before),
                    to: record_old.clone(),
                })
            }
        }
    }

    pub fn update_query(&mut self, query: Query) {
        self.view_query_results
            .update_query(query, self.entries.iter(), &self.on_view_update);
    }

    pub fn on_view_update(&mut self, cb: Box<dyn Fn(ViewUpdate)>) {
        self.on_view_update.replace(cb);
    }

    pub fn on_notification(&mut self, cb: Box<dyn Fn(Notification)>) {
        self.on_notification.replace(cb);
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Selector {
    pub tags: Vec<Tag>,
}

impl Selector {
    pub fn matched_tags(&self, entry: &Entry) -> Vec<Tag> {
        // We consider query tags as part of OR statements and entry
        // is matched if any of the tags matches, which matched if any of props matches.
        // Probably should optimize it as it's quadratic and inside prop matching is
        // quadratic as well. On the other hand usually we have 1-2 tags with 1-2 props
        let mut tags = Vec::new();
        for query_tag in &self.tags {
            for entry_tag in &entry.tags {
                if entry_tag.matches(query_tag) {
                    tags.push(entry_tag.clone());
                }
            }
        }
        tags
    }

    // TODO Query should never match skills itself
    pub fn matches(&self, entry: &Entry) -> bool {
        for query_tag in &self.tags {
            for entry_tag in &entry.tags {
                if entry_tag.matches(query_tag) {
                    return true;
                }
            }
        }
        false
    }
}

/// To query entries filtered by certain conditions
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Query {
    pub selector: Selector,
    pub date_start: Option<DateDay>,
    pub date_end: Option<DateDay>,
}

impl Query {
    pub fn new(query: &str) -> Result<Query, ParseError> {
        if query.is_empty() {
            return Ok(Query::default());
        }
        let mut parser = Parser::new(query);
        let (tags, _) = parser.parse_record()?;
        let mut date_start = None;
        let mut date_end = None;
        for tag in &tags {
            if tag.name == "filter" {
                for prop in &tag.props {
                    if let (PropVal::String(val), "after") = (&prop.val, prop.name.as_str()) {
                        date_start = Some(val.parse::<DateDay>().map_err(|_| {
                            ParseError::Unexpected(
                                format!("'date' in YYYY-MM-DD format is expected, got |{val}|"),
                                prop.start_pos,
                            )
                        })?);
                    } else if let (PropVal::String(val), "before") = (&prop.val, prop.name.as_str())
                    {
                        date_end = Some(val.parse::<DateDay>().map_err(|_| {
                            ParseError::Unexpected(
                                format!("'date' in YYYY-MM-DD format is expected, got |{val}|"),
                                prop.start_pos,
                            )
                        })?);
                    } else {
                        return Err(ParseError::Unexpected(
                            "'after' or 'before' property is expected".to_string(),
                            tag.start_pos,
                        ));
                    };
                }
            }
        }
        let selector = Selector {
            tags: tags.into_iter().filter(|v| v.name != "filter").collect(), // filter is a special tag and should not be considered as a selector
        };
        Ok(Query {
            selector,
            date_start,
            date_end,
        })
    }

    pub fn matches(&self, record: &Record) -> bool {
        // Check first for date limits
        if let Some(min_date) = self.date_start {
            if record.date_range().start().date() < min_date {
                return false;
            }
        }
        if let Some(max_date) = self.date_end {
            if record.date_range().end().date() > max_date {
                return false;
            }
        }
        if self.selector.tags.is_empty() {
            return true; // It's just a date filter
        }
        match record {
            Record::Entry(entry) => self.selector.matches(entry),
            Record::Conflict(_) => true, // All conflicts matches any tag selector to make it visible for the user
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY_PREFIX: &str = "2000-01-01 00:00";

    fn new_conflict(records: Vec<&Record>) -> BTreeSet<Entry> {
        records
            .iter()
            .map(|v| {
                if let Record::Entry(v) = v {
                    return v.clone();
                }
                unreachable!()
            })
            .collect()
    }

    fn modified_conflict(conflict: &BTreeSet<Entry>, record: Record) -> BTreeSet<Entry> {
        let mut conflict = conflict.clone();
        if let Record::Entry(v) = record {
            conflict.insert(v);
            return conflict;
        }
        unreachable!()
    }

    fn parse_entry(text: &str) -> Record {
        let entry = Entry::parse(&format!("{ENTRY_PREFIX} {}", text)).unwrap();
        Record::Entry(entry)
    }

    #[derive(Default)]
    struct TestDB {
        db: DB,
        events: Vec<ChangeEvent>,
    }
    impl TestDB {
        fn add(&mut self, record: Record) {
            if let Some(event) = self.db.add(record, false, None) {
                self.events.push(event);
            }
        }

        fn add_entry(&mut self, entry: &'static str) {
            self.add(parse_entry(entry));
        }

        fn assert_events(&self, want: Vec<ChangeEvent>) {
            assert_eq!(self.events, want);
        }

        fn assert_record(&self, want: Vec<&Record>) {
            let got: Vec<(DateTimeRange, &Record)> = self
                .db
                .entries
                .iter()
                .map(|(date_range, record)| (*date_range, record))
                .collect();
            let want: Vec<(DateTimeRange, &Record)> =
                want.into_iter().map(|v| (*v.date_range(), v)).collect();
            assert_eq!(got, want);
        }

        fn assert_query_results(&mut self, query: &'static str, want: Vec<&'static str>) {
            let query = Query::new(query).unwrap();
            self.db.update_query(query);
            let result: Vec<_> = self
                .db
                .query_results()
                .iter()
                .map(|v| {
                    let s = &v.to_string(true, true)[ENTRY_PREFIX.len() + 1..].to_string();
                    s.clone()
                })
                .collect();
            assert_eq!(result, want);
        }
    }

    #[test]
    fn merge_logic_append() {
        // Adding entries with different dateranges just appends
        let rec1 = parse_entry("00:01 a");
        let rec2 = parse_entry("00:02 a");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        db.add(rec2.clone());
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Added(rec2.clone()),
        ]);
        db.assert_record(vec![&rec1, &rec2])
    }

    #[test]
    fn merge_logic_ignore() {
        // Adding the same entry is ignored
        let rec1 = parse_entry("00:01 a. entry revision=2");
        let rec2 = parse_entry("00:01 a. entry revision=2");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        db.add(rec2);
        db.assert_events(vec![ChangeEvent::Added(rec1.clone())]);
        db.assert_record(vec![&rec1]);
        // Adding entry with lower revision is ignored
        let rec3 = parse_entry("00:01 c. entry revision=1");
        db.add(rec3);
        db.assert_events(vec![ChangeEvent::Added(rec1.clone())]);
        db.assert_record(vec![&rec1]);
    }

    #[test]
    fn merge_logic_replace() {
        // Adding entry with higher revision replaces
        let rec1 = parse_entry("00:01 a. entry revision=1");
        let rec2 = parse_entry("00:01 b. entry revision=2");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        db.add(rec2.clone());
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: rec2.clone(),
            },
        ]);
        db.assert_record(vec![&rec2]);
    }

    #[test]
    fn merge_logic_conflict() {
        // Two records with the same daterange and revision creates a conflict
        let rec1 = parse_entry("00:01 a");
        let rec2 = parse_entry("00:01 b");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        db.add(rec2.clone());
        let conflict1 = new_conflict(vec![&rec1, &rec2]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1.clone(),
                to: Record::Conflict(conflict1.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict1.clone())]);
        // Adding new value will be added to the conflict
        let rec3 = parse_entry("00:01 c");
        db.add(rec3.clone());
        let conflict2 = modified_conflict(&conflict1, rec3);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict1.clone()),
            },
            ChangeEvent::Replaced {
                from: Record::Conflict(conflict1),
                to: Record::Conflict(conflict2.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict2)]);
    }

    #[test]
    fn merge_logic_two_conflicts() {
        let rec1 = parse_entry("00:01 a");
        let rec2 = parse_entry("00:01 b");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        db.add(rec2.clone());
        let rec3 = parse_entry("00:01 c");
        let rec4 = parse_entry("00:01 d");
        let conflict = new_conflict(vec![&rec3, &rec4]);
        db.add(Record::Conflict(conflict));
        let conflict1 = new_conflict(vec![&rec1, &rec2]);
        let conflict2 = new_conflict(vec![&rec1, &rec2, &rec3, &rec4]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict1.clone()),
            },
            ChangeEvent::Replaced {
                from: Record::Conflict(conflict1),
                to: Record::Conflict(conflict2.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict2)]);
    }

    #[test]
    fn merge_logic_external_conflict() {
        let rec1 = parse_entry("00:01 a");
        let mut db = TestDB::default();
        db.add(rec1.clone());
        let rec2 = parse_entry("00:01 b");
        let rec3 = parse_entry("00:01 c");
        let conflict = new_conflict(vec![&rec2, &rec3]);
        db.add(Record::Conflict(conflict));
        let conflict = new_conflict(vec![&rec1, &rec2, &rec3]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict)]);
    }

    #[test]
    fn query() {
        let mut db = TestDB::default();
        db.add_entry("00:01 foo");
        db.add_entry("00:02 bar");
        db.add_entry("00:03 foo");
        db.add_entry("00:04 run. skill kind=physical. Runner");

        // Found entries
        db.assert_query_results("foo", vec!["00:01 foo", "00:03 foo"]);

        // No entries
        db.assert_query_results("foobar", vec![]);

        // Entries with multiple tags
        db.assert_query_results("skill", vec!["00:04 run. skill kind=physical. Runner"]);

        // Filter by date
        db.assert_query_results(
            "filter after=2000-01-01",
            vec![
                "00:01 foo",
                "00:02 bar",
                "00:03 foo",
                "00:04 run. skill kind=physical. Runner",
            ],
        );

        // Filter by date and tag
        db.assert_query_results("bar. filter after=2000-01-01", vec!["00:02 bar"]);

        // Nothing found
        db.assert_query_results("filter before=1999-01-01", vec![]);

        // Multiple filter by date
        db.assert_query_results(
            "bar. filter after=2000-01-01 before=2000-01-02",
            vec!["00:02 bar"],
        );

        // Empty query matches all
        db.assert_query_results(
            "",
            vec![
                "00:01 foo",
                "00:02 bar",
                "00:03 foo",
                "00:04 run. skill kind=physical. Runner",
            ],
        )
    }
}
