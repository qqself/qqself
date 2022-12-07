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

impl JournalView {
    pub fn update(&mut self, all: Iter<DateTimeRange, Record>, _: &ChangeEvent) {
        self.recalculate(all) // TODO Use change event to make it efficient
    }

    fn recalculate(&mut self, records: Iter<DateTimeRange, Record>) {
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
    }

    pub fn data(&self) -> &BTreeMap<DateDay, JournalDay> {
        &self.data
    }
}
