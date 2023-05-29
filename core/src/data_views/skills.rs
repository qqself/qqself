use std::collections::btree_map::Iter;

use crate::{
    date_time::datetime::DateTimeRange,
    db::{ChangeEvent, Record, RecordValue},
    progress::skill::Skill,
};

/// View shows data from perspective of skill development
#[derive(Default)]
pub struct SkillsView {
    data: Vec<Skill>,
}

impl SkillsView {
    pub fn update(&mut self, all: Iter<DateTimeRange, Record>, _: &ChangeEvent) {
        // TODO We can optimize by processing added records instead of recalculating things every time
        self.recalculate(all);
    }

    fn recalculate(&mut self, records: Iter<DateTimeRange, Record>) {
        self.data.clear(); // Start over discarding all existing skills
        for (_, record) in records {
            let entry = match record {
                Record::Value(RecordValue::Entry(entry)) => entry.entry(),
                _ => continue, // We don't care about non entries
            };
            // TODO Skill calculation works only if skill definition appears before it's entries
            match Skill::from_record(entry) {
                Some(skill) => {
                    if let Some(existing) =
                        self.data.iter_mut().find(|v| v.title() == skill.title())
                    {
                        existing.merge_selector(skill); // Skill exists, merge it's selectors
                    } else {
                        self.data.push(skill); // New skill - add as is
                    }
                }
                None => {
                    // Normal entry, iterate all the skills and append time if matches
                    for skill in self.data.iter_mut() {
                        if skill.selector().matches(entry) {
                            skill.add_duration(entry.date_range.duration());
                            continue; // Only append duration once per skill
                        }
                    }
                }
            }
        }
        self.data.sort(); // Keep skills sorted
    }

    pub fn data(&self) -> &Vec<Skill> {
        &self.data
    }
}
