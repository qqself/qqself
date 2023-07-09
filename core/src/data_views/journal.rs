use std::collections::BTreeMap;

use crate::{
    date_time::datetime::{DateDay},
    db::{ChangeEvent, Record, RecordValue, ViewUpdate},
    record::Entry,
};

#[derive(Clone, Debug)]
pub struct JournalDay {
    pub entries: Vec<Entry>,
    pub day: DateDay,
}

impl JournalDay {
    pub fn new(day: DateDay) -> Self {
        Self {
            day,
            entries: vec![],
        }
    }
}

#[derive(Default)]
pub struct JournalView {
    data: BTreeMap<DateDay, JournalDay>,
}

#[derive(PartialEq, Debug)]
pub struct JournalUpdate {
    pub day: DateDay,
}

impl JournalView {
    pub fn update(
        &mut self,
        event: &ChangeEvent,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
    ) {
        let ChangeEvent::Added(Record::Value(RecordValue::Entry(entry))) = event else { return };
        let entry_day = entry.entry().date_range().start().date();
        let journal_day = self.data.entry(entry_day).or_insert(JournalDay {
            day: entry_day,
            entries: vec![],
        });
        journal_day.entries.push(entry.entry().clone());
        if let Some(update) = on_view_update {
            update(ViewUpdate::Journal(JournalUpdate { day: entry_day }));
        }
    }

    pub fn data(&self) -> &BTreeMap<DateDay, JournalDay> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use crate::db::RecordEntry;

    use super::*;

    fn change_event(s: &str) -> ChangeEvent {
        ChangeEvent::Added(Record::Value(RecordValue::Entry(RecordEntry::new(
            1,
            Entry::parse(s).unwrap(),
        ))))
    }

    fn expect_entries(journal: &JournalView, day: DateDay, length: usize) {
        assert_eq!(journal.data().get(&day).unwrap().entries.len(), length);
    }

    #[test]
    fn update() {
        let mut journal = JournalView::default();
        // New day
        journal.update(&change_event("2020-03-07 10:00 11:00 qqself"), &None);
        expect_entries(&journal, DateDay::new(2020, 3, 7), 1);

        // Add entry to the same day
        journal.update(&change_event("2020-03-07 13:00 14:00 qqself"), &None);
        expect_entries(&journal, DateDay::new(2020, 3, 7), 2);
    }
}
