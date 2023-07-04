use std::collections::{btree_map::Iter, BTreeMap};

use crate::{
    date_time::datetime::{DateDay, DateTimeRange},
    db::{ChangeEvent, Record, RecordValue},
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

pub enum JournalUpdate {
    DayUpdated(DateDay),
}

impl JournalView {
    pub fn update(
        &mut self,
        all: Iter<DateTimeRange, Record>,
        event: &ChangeEvent,
    ) -> Option<JournalUpdate> {
        // TODO Terribly inefficient, but we re-create whole journal every time we add a new entry
        //      Done as a part of PoC, rewrite to append an event instead and avoid extra work
        self.recalculate(all);
        // TODO As we recalculate whole thing every time there is no way to emit proper JournalUpdate
        //      We can use ChangeEvent as a shortcut for now
        if let ChangeEvent::Added(Record::Value(RecordValue::Entry(entry))) = event {
            return Some(JournalUpdate::DayUpdated(
                entry.entry().date_range().start().date(),
            ));
        }
        None
    }

    fn recalculate(&mut self, records: Iter<DateTimeRange, Record>) -> Option<JournalUpdate> {
        self.data.clear();
        for (_, record) in records {
            let entry = match record {
                Record::Value(RecordValue::Entry(entry)) => entry.entry(),
                _ => continue, // We don't care about non entries
            };
            let entry_day = entry.date_range().start().date();
            if self.data.is_empty() {
                self.data.insert(
                    entry_day,
                    JournalDay {
                        day: entry_day,
                        entries: vec![],
                    },
                );
            }
            let cur_day = self.data.entry(entry_day).or_insert(JournalDay {
                day: entry_day,
                entries: vec![],
            });
            if cur_day.day == entry_day {
                cur_day.entries.push(entry.clone());
            } else {
                self.data.insert(
                    entry_day,
                    JournalDay {
                        day: entry_day,
                        entries: vec![entry.clone()],
                    },
                );
            }
        }
        None
    }

    pub fn data(&self) -> &BTreeMap<DateDay, JournalDay> {
        &self.data
    }
}
