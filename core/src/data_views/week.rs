use std::collections::{btree_map::Iter, BTreeMap};

use crate::{
    date_time::datetime::{DateDay, DateTimeRange},
    db::{ChangeEvent, Record, ViewUpdate},
    progress::skill::Skill,
    record::Entry,
};

/// Views shows progress relevant to the current week
#[derive(Default)]
pub struct WeekView {
    data: BTreeMap<String, WeekProgress>,
}

#[derive(Debug)]
pub struct WeekProgress {
    skill: Skill,
    progress: u64,
}

impl WeekProgress {
    pub fn skill(&self) -> &str {
        self.skill.title()
    }

    pub fn progress(&self) -> u64 {
        self.progress
    }

    pub fn target(&self) -> u64 {
        self.skill.perfect_week() * 60
    }
}

#[derive(PartialEq, Debug)]
pub struct WeekUpdate;

impl WeekView {
    pub fn update(
        &mut self,
        all: Iter<DateTimeRange, Record>,
        event: &ChangeEvent,
        now: DateDay,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
    ) {
        let week_start = now.as_start_of_week();
        let entry = match event {
            ChangeEvent::Added(Record::Entry(entry)) => entry,
            ChangeEvent::Replaced {
                from: Record::Entry(from),
                to: Record::Entry(to),
            } => {
                if from.date_range.start().date() >= week_start {
                    self.delete_entry(from, on_view_update);
                }
                to
            }
            _ => return, // TODO Handle conflicts
        };

        if let Some(skill) = Skill::from_record(entry) {
            if skill.perfect_week() == 0 {
                return; // It's a skill without a perfect target, nothing to do here
            }
            // If it's a Skill - go back and re-read all previous record to accumulate duration
            let mut progress = 0;
            for (_, record) in all.filter(|v| v.0.start().date() >= week_start).clone() {
                let Record::Entry(entry) = record else {
                    continue;
                };
                if skill.selector().matches(entry) {
                    progress += entry.date_range.duration().minutes();
                }
            }
            self.data.insert(
                skill.title().to_string(),
                WeekProgress {
                    skill: skill.clone(),
                    progress,
                },
            );
            if let Some(on_view_update) = on_view_update {
                on_view_update(ViewUpdate::Week)
            }
        } else {
            if entry.date_range.start().date() < week_start {
                return; // Entry is too old and not relevant for the week - skip
            }
            for (_, week_progress) in self.data.iter_mut() {
                if week_progress.skill.selector().matches(entry) {
                    week_progress.progress += entry.date_range.duration().minutes();
                    if let Some(on_view_update) = on_view_update {
                        on_view_update(ViewUpdate::Week)
                    }
                }
            }
        }
    }

    fn delete_entry(&mut self, entry: &Entry, on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>) {
        if let Some(skill) = Skill::from_record(entry) {
            self.data.remove(skill.title());
            if let Some(on_view_update) = on_view_update {
                on_view_update(ViewUpdate::Week)
            }
            return;
        }
        // If it's a record - remove it from the corresponding skills if any
        for (_, skill) in self.data.iter_mut() {
            if skill.skill.selector().matches(entry) {
                skill.progress -= entry.date_range.duration().minutes();
                if let Some(on_view_update) = on_view_update {
                    on_view_update(ViewUpdate::Week)
                }
            }
        }
    }

    pub fn data(&self) -> &BTreeMap<String, WeekProgress> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestView {
        records: BTreeMap<DateTimeRange, Record>,
        view: WeekView,
    }

    impl TestView {
        fn add(&mut self, now: DateDay, entry: &str) -> Record {
            let record = Record::Entry(Entry::parse(entry).unwrap());
            self.records.insert(*record.date_range(), record.clone());
            self.view.update(
                self.records.iter(),
                &ChangeEvent::Added(record.clone()),
                now,
                &None,
            );
            record
        }

        fn check_progress(&self, want: Vec<(&'static str, u64)>) {
            let got: Vec<_> = self
                .view
                .data
                .values()
                .map(|v| (v.skill.title(), v.progress))
                .collect();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn update() {
        let now = DateDay::new(2022, 1, 1);
        let mut view = TestView::default();
        view.add(
            now,
            "2022-01-01 00:00 00:00 run. skill kind=physical. Running",
        );
        // By default skills without `perfect` prop are ignored
        view.check_progress(vec![]);

        // Adding a `perfect` property makes it visible for the week
        view.add(
            now,
            "2022-01-01 00:00 00:01 run. skill kind=physical perfect=10. Running",
        );
        view.check_progress(vec![("Running", 0)]);

        // Adding an entry should updates weekly progress
        view.add(now, "2022-01-01 00:00 01:00 run");
        view.check_progress(vec![("Running", 60)]);

        // Adding en entry from outside of this week should have no effect
        view.add(now, "2000-01-01 00:00 01:00 run");
        view.check_progress(vec![("Running", 60)]);
    }

    #[test]
    fn delete_too_old() {
        // We can't use TestView and helpers in here, so a bit verbose to reproduce the bug
        // with overflow error when we delete replaced entries even in case they are out
        // of current week
        let now = DateDay::new(2023, 11, 27);
        let mut view = WeekView::default();
        let mut all = BTreeMap::default();

        // First init it with some skill
        let record = Record::Entry(
            Entry::parse("2023-11-01 00:00 00:00 run. skill kind=physical perfect=10. Running")
                .unwrap(),
        );
        all.insert(*record.date_range(), record.clone());
        view.update(all.iter(), &ChangeEvent::Added(record.clone()), now, &None);

        // Not add a too old record
        let record = Record::Entry(Entry::parse("2023-11-01 00:00 01:00 run. Comment1").unwrap());
        all.insert(*record.date_range(), record.clone());
        view.update(all.iter(), &ChangeEvent::Added(record.clone()), now, &None);

        // And not replace it with another one
        let record_new = Record::Entry(
            Entry::parse("2023-11-01 00:00 01:00 run. entry revision=2. Comment2").unwrap(),
        );
        all.insert(*record_new.date_range(), record_new.clone());
        view.update(
            all.iter(),
            &ChangeEvent::Replaced {
                from: record.clone(),
                to: record_new,
            },
            now,
            &None,
        );
        let got: Vec<_> = view
            .data
            .values()
            .map(|v| (v.skill.title(), v.progress))
            .collect();
        assert_eq!(got, vec![("Running", 0)]);
    }
}
