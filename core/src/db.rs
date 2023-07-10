use std::collections::btree_map::Iter;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Weak;
use std::vec;

use crate::data_views::journal::{JournalDay, JournalUpdate, JournalView};
use crate::data_views::skills::{SkillsNotification, SkillsUpdate, SkillsView};
use crate::date_time::datetime::{DateDay, DateTimeRange};
use crate::parsing::parser::{ParseError, Parser};
use crate::progress::skill::Skill;
use crate::record::{Entry, Tag};

#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub struct RecordEntry {
    revision: usize,
    entry: Entry,
}

impl RecordEntry {
    pub fn new(revision: usize, entry: Entry) -> Self {
        Self { revision, entry }
    }
    pub fn entry(&self) -> &Entry {
        &self.entry
    }
}

#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub struct RecordEmpty {
    revision: usize,
    date_range: DateTimeRange,
}

#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum RecordValue {
    Entry(RecordEntry),
    Empty(RecordEmpty),
}

impl RecordValue {
    fn date_range(&self) -> DateTimeRange {
        match self {
            RecordValue::Entry(v) => v.entry.date_range,
            RecordValue::Empty(v) => v.date_range,
        }
    }
    fn revision(&self) -> usize {
        match self {
            RecordValue::Entry(v) => v.revision,
            RecordValue::Empty(v) => v.revision,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RecordConflict {
    revision: usize,
    entries: BTreeSet<RecordValue>,
}

impl RecordConflict {
    fn date_range(&self) -> DateTimeRange {
        let entry = self
            .entries
            .iter()
            .next()
            .expect("Conflict should always have an entry");
        entry.date_range()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Record {
    Value(RecordValue),
    Conflict(RecordConflict),
}

impl Record {
    fn revision(&self) -> usize {
        match self {
            Record::Value(v) => v.revision(),
            Record::Conflict(v) => v.revision,
        }
    }
    pub fn daterange(&self) -> DateTimeRange {
        match self {
            Record::Value(v) => v.date_range(),
            Record::Conflict(v) => v.date_range(),
        }
    }

    pub fn from_entry(entry: Entry, revision: usize) -> Self {
        Self::Value(RecordValue::Entry(RecordEntry { revision, entry }))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeEvent {
    Added(Record),
    Replaced {
        from: Record,
        to: Record,
    },
    ConflictUpdated {
        from: RecordConflict,
        to: RecordConflict,
    },
}

/// Emitted when view data got updated and clients need to re-render the view
#[derive(PartialEq, Debug)]
pub enum ViewUpdate {
    Journal(JournalUpdate),
    Skills(SkillsUpdate),
}

/// Emitter when user interactively added a new record and progress notification has
/// to be shown to the user
pub enum Notification {
    Skills(SkillsNotification),
}

pub trait DBSubscriber {
    fn notify(&self, all: Iter<DateTimeRange, Record>, event: &ChangeEvent);
}

// Parsed collection of all active entries and goals
pub struct DB {
    entries: BTreeMap<DateTimeRange, Record>,
    // TODO: DBSubscriber is an old functionality from PoC and should be removed
    on_new_record: Vec<Weak<dyn DBSubscriber>>,
    on_notification: Option<Box<dyn Fn(Notification)>>,
    on_view_update: Option<Box<dyn Fn(ViewUpdate)>>,
    view_journal: JournalView,
    view_skills: SkillsView,
}

impl DB {
    pub fn new() -> Self {
        DB {
            entries: BTreeMap::new(),
            on_new_record: vec![],
            view_skills: SkillsView::default(),
            view_journal: JournalView::default(),
            on_view_update: None,
            on_notification: None,
        }
    }

    pub fn skills(&self) -> &BTreeMap<String, Skill> {
        self.view_skills.data()
    }

    pub fn journal(&self) -> &BTreeMap<DateDay, JournalDay> {
        self.view_journal.data()
    }

    /// Adds new record to the DB. Interactively means user is adding a record right now. If records are restored from
    /// cache, fetched from API then it's considered not interactive and simple `DB::add` should be called instead.
    /// In interactive mode user may benefit from `Notifications`, so those are emitted in case of noticeable progress
    pub fn add_interactively(&mut self, record: Record, now: DateDay) {
        let Some(event) = self.merge(record) else { return };
        self.view_journal.update(&event, &self.on_view_update);
        self.view_skills.update(
            self.entries.iter(),
            &event,
            true,
            Some(now),
            &self.on_view_update,
            &self.on_notification,
        );
    }

    /// Adds new record to the DB. Notifications are not emitted as adding considered not interactive
    pub fn add(&mut self, record: Record, interactive: bool, now: Option<DateDay>) {
        let Some(event) = self.merge(record) else { return };

        self.view_journal.update(&event, &self.on_view_update);
        self.view_skills.update(
            self.entries.iter(),
            &event,
            interactive,
            now,
            &self.on_view_update,
            &self.on_notification,
        );

        // TODO: Remove when `on_new_record` will be removed
        let mut cleanup = false;
        for s in self.on_new_record.iter() {
            if let Some(subscriber) = s.upgrade() {
                subscriber.notify(self.entries.iter(), &event)
            } else {
                cleanup = true;
            }
        }
        if cleanup {
            self.on_new_record.retain(|s| s.upgrade().is_some());
        }
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
        let entry_new_key = record_new.daterange();
        let mut record_old = match self.entries.get_mut(&entry_new_key) {
            None => {
                let event = ChangeEvent::Added(record_new.clone());
                self.entries.insert(entry_new_key, record_new);
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
                conflict_old.entries.append(&mut conflict_new.entries);
                conflict_old.revision += 1;
                Some(ChangeEvent::ConflictUpdated {
                    from: conflict_before,
                    to: conflict_old.clone(),
                })
            }
            (Record::Value(value_old), Record::Value(value_new)) => {
                // Two conflicting values - replace existing record with Conflict value and append new entry there
                let value_before = value_old.clone();
                *record_old = Record::Conflict(RecordConflict {
                    revision: value_new.revision(),
                    entries: BTreeSet::from([value_old.clone(), value_new.to_owned()]),
                });
                Some(ChangeEvent::Replaced {
                    from: Record::Value(value_before),
                    to: record_old.clone(),
                })
            }
            (Record::Conflict(conflict_old), Record::Value(value_new)) => {
                let conflict_before = conflict_old.clone();
                // Existing conflict - append new entry
                conflict_old.entries.insert(value_new.to_owned());
                conflict_old.revision += 1;
                Some(ChangeEvent::ConflictUpdated {
                    from: conflict_before,
                    to: conflict_old.clone(),
                })
            }
            (Record::Value(value_old), Record::Conflict(conflict_new)) => {
                // Existing value, but new conflict, merge to conflict
                let value_before = value_old.clone();
                let mut entries = conflict_new.entries.to_owned();
                entries.insert(value_old.clone());
                *record_old = Record::Conflict(RecordConflict {
                    revision: value_old.revision(),
                    entries,
                });
                Some(ChangeEvent::Replaced {
                    from: Record::Value(value_before),
                    to: record_old.clone(),
                })
            }
        }
    }

    pub fn query(&self, _: Query) -> Result<Vec<Entry>, QueryError> {
        todo!()
    }

    pub fn subscribe_entry_updates(&mut self, subscriber: Weak<dyn DBSubscriber>) {
        self.on_new_record.push(subscriber);
    }

    pub fn on_view_update(&mut self, cb: Box<dyn Fn(ViewUpdate)>) {
        self.on_view_update.replace(cb);
    }

    pub fn on_notification(&mut self, cb: Box<dyn Fn(Notification)>) {
        self.on_notification.replace(cb);
    }
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
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
    pub date_filter: Option<DateTimeRange>,
}

impl Query {
    pub fn new(query: &str, date_filter: Option<DateTimeRange>) -> Result<Query, ParseError> {
        let mut parser = Parser::new(query);
        let (tags, _) = parser.parse_record()?;
        let selector = Selector { tags };
        Ok(Query {
            selector,
            date_filter,
        })
    }

    pub fn matched_tags(&self, entry: &Entry) -> Vec<Tag> {
        // Check first for date limits
        if let Some(filter) = &self.date_filter {
            if entry.date_range.start() < filter.start() || entry.date_range.end() > filter.end() {
                return vec![];
            }
        }
        self.selector.matched_tags(entry)
    }
}

// Query execution error
#[derive(Debug)]
pub enum QueryError {
    BadQuery(String),
}

#[cfg(test)]
mod tests {
    use crate::date_time::datetime::DateDay;

    use super::DBSubscriber;
    use super::*;
    use std::{rc::Rc, sync::Mutex};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref BASE_DATE: DateDay = DateDay::new(2000, 1, 1);
    }

    fn new_conflict(revision: usize, records: Vec<&Record>) -> RecordConflict {
        let entries: BTreeSet<_> = records
            .iter()
            .map(|v| {
                if let Record::Value(v) = v {
                    return v.clone();
                }
                unreachable!()
            })
            .collect();
        RecordConflict { revision, entries }
    }

    fn modified_conflict(conflict: &RecordConflict, record: Record) -> RecordConflict {
        let mut conflict = conflict.clone();
        if let Record::Value(v) = record {
            conflict.entries.insert(v);
            conflict.revision += 1;
            return conflict;
        }
        unreachable!()
    }

    struct TestSubscriber {
        events: Mutex<Vec<ChangeEvent>>,
    }

    impl DBSubscriber for TestSubscriber {
        fn notify(&self, _: Iter<DateTimeRange, Record>, event: &ChangeEvent) {
            let mut events = self.events.lock().unwrap();
            events.push(event.clone())
        }
    }

    fn parse_entry(text: &str, revision: usize) -> Record {
        let entry =
            Entry::parse(&format!("{} 00:00 - {} {}", *BASE_DATE, *BASE_DATE, text)).unwrap();
        Record::Value(RecordValue::Entry(RecordEntry { revision, entry }))
    }

    struct TestDB {
        db: DB,
        sub: Rc<TestSubscriber>,
    }
    impl TestDB {
        fn new() -> Self {
            let mut db = DB::new();
            let sub = Rc::new(TestSubscriber {
                events: Mutex::new(Vec::new()),
            });
            db.subscribe_entry_updates(Rc::downgrade(&(sub.clone() as Rc<dyn DBSubscriber>)));
            Self { db, sub }
        }
        fn add(&mut self, record: Record) {
            self.db.add(record, false, None)
        }
        fn assert_events(&self, want: Vec<ChangeEvent>) {
            let got = self.sub.events.lock().unwrap();
            assert_eq!(*got, want);
        }
        fn assert_record(&self, want: Vec<&Record>) {
            let got: Vec<(DateTimeRange, &Record)> = self
                .db
                .entries
                .iter()
                .map(|(date_range, record)| (*date_range, record))
                .collect();
            let want: Vec<(DateTimeRange, &Record)> =
                want.into_iter().map(|v| (v.daterange(), v)).collect();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn db_subscribers() {
        let mut db = DB::new();
        {
            // New scope to see that subscribers got removed later on
            let sub = Rc::new(TestSubscriber {
                events: Mutex::new(Vec::new()),
            });
            db.add(parse_entry("00:01 a", 0), false, None);
            assert_eq!(sub.events.lock().unwrap().len(), 0);
            db.subscribe_entry_updates(Rc::downgrade(&(sub.clone() as Rc<dyn DBSubscriber>)));
            db.add(parse_entry("00:02 b", 0), false, None);
            assert_eq!(sub.events.lock().unwrap().len(), 1);
            assert_eq!(db.on_new_record.len(), 1);
        }
        db.add(parse_entry("00:03 c", 0), false, None);
        assert_eq!(db.on_new_record.len(), 0)
    }

    #[test]
    fn merge_logic_append() {
        // Adding entries with different dateranges just appends
        let rec1 = parse_entry("00:01 a", 0);
        let rec2 = parse_entry("00:02 a", 0);
        let mut db = TestDB::new();
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
        let rec1 = parse_entry("00:01 a", 1);
        let rec2 = parse_entry("00:01 a", 1);
        let mut db = TestDB::new();
        db.add(rec1.clone());
        db.add(rec2);
        db.assert_events(vec![ChangeEvent::Added(rec1.clone())]);
        db.assert_record(vec![&rec1]);
        // Adding entry with lower revision is ignored
        let rec3 = parse_entry("00:01 c", 0);
        db.add(rec3);
        db.assert_events(vec![ChangeEvent::Added(rec1.clone())]);
        db.assert_record(vec![&rec1]);
    }

    #[test]
    fn merge_logic_replace() {
        // Adding entry with higher revision replaces
        let rec1 = parse_entry("00:01 a", 1);
        let rec2 = parse_entry("00:01 b", 2);
        let mut db = TestDB::new();
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
        let rec1 = parse_entry("00:01 a", 1);
        let rec2 = parse_entry("00:01 b", 1);
        let mut db = TestDB::new();
        db.add(rec1.clone());
        db.add(rec2.clone());
        let conflict1 = new_conflict(1, vec![&rec1, &rec2]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1.clone(),
                to: Record::Conflict(conflict1.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict1.clone())]);
        // Adding new value will be added to the conflict
        let rec3 = parse_entry("00:01 c", 1);
        db.add(rec3.clone());
        let conflict2 = modified_conflict(&conflict1, rec3);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict1.clone()),
            },
            ChangeEvent::ConflictUpdated {
                from: conflict1,
                to: conflict2.clone(),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict2)]);
    }

    #[test]
    fn merge_logic_two_conflicts() {
        let rec1 = parse_entry("00:01 a", 1);
        let rec2 = parse_entry("00:01 b", 1);
        let mut db = TestDB::new();
        db.add(rec1.clone());
        db.add(rec2.clone());
        let rec3 = parse_entry("00:01 c", 1);
        let rec4 = parse_entry("00:01 d", 1);
        let conflict = new_conflict(1, vec![&rec3, &rec4]);
        db.add(Record::Conflict(conflict));
        let conflict1 = new_conflict(1, vec![&rec1, &rec2]);
        let conflict2 = new_conflict(2, vec![&rec1, &rec2, &rec3, &rec4]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict1.clone()),
            },
            ChangeEvent::ConflictUpdated {
                from: conflict1,
                to: conflict2.clone(),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict2)]);
    }

    #[test]
    fn merge_logic_external_conflict() {
        let rec1 = parse_entry("00:01 a", 1);
        let mut db = TestDB::new();
        db.add(rec1.clone());
        let rec2 = parse_entry("00:01 b", 1);
        let rec3 = parse_entry("00:01 c", 1);
        let conflict = new_conflict(1, vec![&rec2, &rec3]);
        db.add(Record::Conflict(conflict));
        let conflict = new_conflict(1, vec![&rec1, &rec2, &rec3]);
        db.assert_events(vec![
            ChangeEvent::Added(rec1.clone()),
            ChangeEvent::Replaced {
                from: rec1,
                to: Record::Conflict(conflict.clone()),
            },
        ]);
        db.assert_record(vec![&Record::Conflict(conflict)]);
    }
}
