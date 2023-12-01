use std::{
    collections::{btree_map::Iter, BTreeMap},
    fmt::Display,
};

use crate::{
    date_time::datetime::{DateDay, DateTimeRange},
    db::{ChangeEvent, Notification, Record, ViewUpdate},
    progress::skill::{Skill, SkillProgress},
    record::Entry,
};

/// View shows data from perspective of skill development
#[derive(Default)]
pub struct SkillsView {
    data: BTreeMap<String, Skill>,
}

#[derive(PartialEq, Debug)]
pub struct SkillsUpdate {
    pub skill: String,
}

#[derive(PartialEq, Debug)]
pub enum SkillsNotification {
    LevelUp(String),
    HourProgress(String),
}

// TODO Skills view become quite complex, we should refactor it and split into multiple structs
impl SkillsView {
    pub fn update(
        &mut self,
        all: Iter<DateTimeRange, Record>,
        event: &ChangeEvent,
        interactive: bool,
        now: Option<DateDay>,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
        on_notification: &Option<Box<dyn Fn(Notification)>>,
    ) {
        let entry = match event {
            ChangeEvent::Added(Record::Entry(entry)) => entry,
            ChangeEvent::Replaced {
                from: Record::Entry(from),
                to: Record::Entry(to),
            } => {
                // It's a replacement, remove previous entry from calculation and continue processing new one as usual
                self.delete_entry(from, on_view_update);
                to
            }
            _ => return, // TODO Handle conflicts
        };

        if let Some(mut skill) = Skill::from_record(entry) {
            // If it's a Skill - go back and re-read all previous record to accumulate duration
            for (_, record) in all.clone() {
                let Record::Entry(entry) = record else {
                    continue;
                };
                if skill.selector().matches(entry) {
                    skill.add_duration(entry.date_range.duration());
                }
            }
            self.data.insert(skill.title().to_string(), skill.clone());
            self.process_update(&skill, on_view_update);
            if let (Some(on_notification), Some(now), true) = (on_notification, now, interactive) {
                self.process_notification(&skill, on_notification, now, all.clone(), None, true)
            }
        } else {
            // If it's a record - add it to the corresponding Skill if exists
            for (_, skill) in self.data.iter_mut() {
                if skill.selector().matches(entry) {
                    skill.add_duration(entry.date_range.duration());
                }
            }
            // Second iteration to notify about skills progress
            let mut send_total_notification = true;
            for (_, skill) in self.data.iter() {
                if skill.selector().matches(entry) {
                    self.process_update(skill, on_view_update);
                    if let (Some(on_notification), Some(now), true) =
                        (on_notification, now, interactive)
                    {
                        self.process_notification(
                            skill,
                            on_notification,
                            now,
                            all.clone(),
                            Some(entry.date_range()),
                            send_total_notification,
                        );
                        // Entry may have multiple tags/skills attached, but we want notification about total processed only once
                        send_total_notification = false;
                    }
                }
            }
        }
    }

    fn delete_entry(&mut self, entry: &Entry, on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>) {
        if let Some(skill) = Skill::from_record(entry) {
            self.data.remove(skill.title());
            self.process_update(&skill, on_view_update);
            return;
        }
        // If it's a record - remove it from the corresponding skills if any
        for (_, skill) in self.data.iter_mut() {
            if skill.selector().matches(entry) {
                skill.remove_duration(entry.date_range.duration());
                // self.process_update cannot be used as self is borrowed as mut inside a loop
                if let Some(on_view_update) = on_view_update {
                    on_view_update(ViewUpdate::Skills(SkillsUpdate {
                        skill: skill.title().to_string(),
                    }))
                }
            }
        }
    }

    pub fn data(&self) -> &BTreeMap<String, Skill> {
        // TODO Now Skills are sorted by it's Title, it should be sorted by our custom logic, see `Skill::Ord`
        &self.data
    }

    fn process_update(&self, skill: &Skill, on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>) {
        if let Some(on_view_update) = on_view_update {
            // Emit event that view got updated
            on_view_update(ViewUpdate::Skills(SkillsUpdate {
                skill: skill.title().to_string(),
            }))
        }
    }

    fn process_notification(
        &self,
        skill: &Skill,
        on_notification: &dyn Fn(Notification),
        now: DateDay,
        all: Iter<DateTimeRange, Record>,
        entry_duration: Option<&DateTimeRange>,
        send_total_notifications: bool,
    ) {
        // Skill level got increased
        let progress_now = skill.progress();
        let progress_before = entry_duration.map_or(SkillProgress::default(), |v| {
            SkillProgress::new(skill.progress().duration_minutes - v.duration().minutes() as usize)
        });
        if progress_before.level < progress_now.level {
            on_notification(Notification::Skills(SkillsNotification::LevelUp(format!(
                "{} level increased to {}",
                skill.title(),
                progress_now.level
            ))))
        }

        // Accumulate time
        // ------------------------------------------------------------------|
        // | Type                | Period   | Checkpoints                    |
        // ------------------------------------------------------------------|
        // | All skills combined | Lifetime | Every 500h                     |
        // | All skills combined | Year     | Every 100h                     |
        // | All skills combined | Month    | Every 50h                      |
        // | All skills combined | Week     | Every 20h                      |
        // | Per skill           | Lifetime | Every 100h                     |
        // | Per skill           | Year     | Every 50h                      |
        // | Per skill           | Month    | Every 10h                      |
        // | Per skill           | Week     | 1h, 3h, 5h, then every 5 hours |
        // ------------------------------------------------------------------|
        let mut checkpoints_total = if send_total_notifications {
            vec![
                Checkpoint::by_total_time(now, Period::Lifetime, &[], 500),
                Checkpoint::by_total_time(now, Period::Year, &[], 100),
                Checkpoint::by_total_time(now, Period::Month, &[], 50),
                Checkpoint::by_total_time(now, Period::Week, &[], 20),
            ]
        } else {
            vec![] // When total notification is disabled we skip any calculation of it
        };
        let mut checkpoints_skills = vec![
            Checkpoint::by_skill(now, Period::Lifetime, &[], 100, skill.title().to_string()),
            Checkpoint::by_skill(now, Period::Year, &[], 50, skill.title().to_string()),
            Checkpoint::by_skill(now, Period::Month, &[], 10, skill.title().to_string()),
            Checkpoint::by_skill(now, Period::Week, &[1, 3, 5], 5, skill.title().to_string()),
        ];
        for (_, rec) in all {
            let Record::Entry(entry) = rec else {
                continue;
            };
            for checkpoint in checkpoints_total.iter_mut() {
                // If entry belongs to any skill then it's added to total notification calculations
                if self.data.iter().any(|(_, s)| s.selector().matches(entry)) {
                    checkpoint.add(entry.date_range);
                }
            }
            if skill.selector().matches(entry) {
                for checkpoint in checkpoints_skills.iter_mut() {
                    checkpoint.add(entry.date_range)
                }
            }
        }
        for checkpoint in checkpoints_total.iter().chain(checkpoints_skills.iter()) {
            checkpoint.notify_if_needed(entry_duration, on_notification)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Period {
    Lifetime,
    Year,
    Month,
    Week,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Period::Lifetime => "",
            Period::Year => "this year",
            Period::Month => "this month",
            Period::Week => "this week",
        })
    }
}

#[derive(Debug)]
struct Checkpoint {
    min_date: DateDay,
    skill: Option<String>,
    checkpoints_start: &'static [usize],
    period: Period,
    checkpoints_every: usize,
    duration: usize,
}

impl Checkpoint {
    fn new(
        now: DateDay,
        skill: Option<String>,
        period: Period,
        checkpoints_start: &'static [usize],
        checkpoints_every: usize,
    ) -> Self {
        let min_date = match period {
            Period::Lifetime => DateDay::new(1, 1, 1),
            Period::Year => now.as_start_of_year(),
            Period::Month => now.as_start_of_month(),
            Period::Week => now.as_start_of_week(),
        };
        Self {
            min_date,
            checkpoints_start,
            checkpoints_every,
            period,
            skill,
            duration: 0,
        }
    }

    fn by_total_time(
        now: DateDay,
        period: Period,
        checkpoints_start: &'static [usize],
        checkpoints_every: usize,
    ) -> Self {
        Checkpoint::new(now, None, period, checkpoints_start, checkpoints_every)
    }

    fn by_skill(
        now: DateDay,
        period: Period,
        checkpoints_start: &'static [usize],
        checkpoints_every: usize,
        skill: String,
    ) -> Self {
        Checkpoint::new(
            now,
            Some(skill),
            period,
            checkpoints_start,
            checkpoints_every,
        )
    }

    fn add(&mut self, date_range: DateTimeRange) {
        if date_range.start().date() >= self.min_date {
            self.duration += date_range.duration().minutes() as usize
        }
    }

    fn notify_if_needed(
        &self,
        entry_duration: Option<&DateTimeRange>,
        on_notification: &dyn Fn(Notification),
    ) {
        let notify = |hours| {
            let msg = match &self.skill {
                Some(skill) => format!(
                    "Great job - you've practiced {} hours of {} {}",
                    hours, skill, self.period
                ),
                None => format!(
                    "Great job - across all skills you've practiced {} hours {}",
                    hours, self.period
                ),
            };
            on_notification(Notification::Skills(SkillsNotification::HourProgress(msg)));
        };

        // We've processed all events and already added an entry_duration, starting point is without it
        let hours_now = self.duration / 60;
        // TODO If we've added a new Skill then total hours notifications will be missed because this new skill
        //      already has all the hours in self.duration, so we cannot calculate how much new hours this skill added
        let entry_duration = entry_duration.map_or(0, |v| v.duration().minutes() as usize);
        let hours_before = if self.duration >= entry_duration {
            (self.duration - entry_duration) / 60
        } else {
            0 // It may happen if we adding an entry to the period before than a current one, otherwise overflow happens
        };

        if hours_before != hours_now {
            for checkpoint in self.checkpoints_start {
                if hours_now == *checkpoint {
                    notify(hours_now);
                    return; // Ignore checkpoint periods if we matched on checkpoint starts to avoid duplicate notifications
                }
            }
        }

        let periods_now = hours_now / self.checkpoints_every;
        let periods_before = hours_before / self.checkpoints_every;
        if periods_before != periods_now {
            notify(periods_now * self.checkpoints_every);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::record::Entry;

    use super::*;

    #[derive(Default)]
    struct TestSkillView {
        records: BTreeMap<DateTimeRange, Record>,
        skill_view: SkillsView,
    }

    impl TestSkillView {
        fn add(&mut self, entry: &str) -> Record {
            let record = Record::Entry(Entry::parse(entry).unwrap());
            self.records.insert(*record.date_range(), record.clone());
            self.skill_view.update(
                self.records.iter(),
                &ChangeEvent::Added(record.clone()),
                false,
                None,
                &None,
                &None,
            );
            record
        }

        fn check_notification(
            &mut self,
            event: ChangeEvent,
            now: Option<DateDay>,
        ) -> Vec<SkillsNotification> {
            match &event {
                ChangeEvent::Added(record) => {
                    self.records.insert(*record.date_range(), record.clone())
                }
                ChangeEvent::Replaced { from: _, to } => {
                    self.records.insert(*to.date_range(), to.clone())
                }
            };
            let called = Rc::new(RefCell::new(Vec::new()));
            let called_clone = called.clone();
            self.skill_view.update(
                self.records.iter(),
                &event,
                true,
                now,
                &None,
                &Some(Box::new(move |got| {
                    match got {
                        Notification::Skills(update) => {
                            let mut foo = called_clone.borrow_mut();
                            foo.push(update);
                        }
                    };
                })),
            );
            called.take()
        }

        fn check_skills(&self, want: Vec<(&'static str, usize)>) {
            let got: Vec<_> = self
                .skill_view
                .data
                .values()
                .map(|v| (v.title(), v.progress().duration_minutes))
                .collect();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn progress_level_up() {
        let mut view = TestSkillView::default();
        let now = Some(DateDay::new(2022, 6, 6));

        // No skill attached for the entity
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-06 10:00 12:00 run").unwrap()),
                now
            ),
            vec![]
        );

        // Adding skill afterwards recalculates all previously added entities
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(
                    Record::parse("2022-06-06 13:00 13:00 run. skill kind=physical. Running")
                        .unwrap()
                ),
                now
            ),
            vec![SkillsNotification::LevelUp(
                "Running level increased to 2".to_string()
            )]
        );

        // Adding not enough for level up
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-06 14:00 14:05 run").unwrap()),
                now
            ),
            vec![]
        );

        // Adding more to cause another level up
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-06 15:00 17:00 run").unwrap()),
                now
            ),
            vec![SkillsNotification::LevelUp(
                "Running level increased to 4".to_string()
            )]
        );
    }

    #[test]
    fn progress_hours_total() {
        let mut view = TestSkillView::default();
        let now = Some(DateDay::new(2022, 6, 8));

        // Total time is ignored for non skill entries
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-08 00:00 23:00 foo").unwrap()),
                now
            ),
            vec![]
        );

        // Total time in a week
        view.check_notification(
            ChangeEvent::Added(
                Record::parse("2022-06-08 00:00 00:00 bar1. skill kind=physical. Bar1").unwrap(),
            ),
            now,
        );
        view.check_notification(
            ChangeEvent::Added(
                Record::parse("2022-06-08 00:00 00:00 bar2. skill kind=physical. Bar2").unwrap(),
            ),
            now,
        );
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-08 00:00 19:00 bar1").unwrap()),
                now
            ),
            vec![
                SkillsNotification::LevelUp("Bar1 level increased to 13".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 10 hours of Bar1 this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 15 hours of Bar1 this week".to_string()
                )
            ]
        );

        // Another skill
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2022-06-08 00:00 05:00 bar2").unwrap()),
                now
            ),
            vec![
                SkillsNotification::LevelUp("Bar2 level increased to 5".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - across all skills you've practiced 20 hours this week".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 5 hours of Bar2 this week".to_string()
                ),
            ]
        );
    }

    #[test]
    fn progress_hours_total_periods() {
        let mut view = TestSkillView::default();
        view.add("2023-07-13 00:00 00:00 run. skill kind=physical. Running");
        view.add("2023-07-13 00:00 10:00 run"); // Thursday week before
        view.add("2023-07-17 00:00 10:00 run"); // Monday

        // Adding entry on Tuesday should emit notification about 20 hours of running in a week
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2023-07-18 00:00 10:00 run").unwrap()),
                Some(DateDay::new(2023, 7, 18))
            ),
            vec![
                SkillsNotification::LevelUp("Running level increased to 17".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - across all skills you've practiced 20 hours this week".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 30 hours of Running this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Running this week".to_string()
                )
            ]
        );
    }

    #[test]
    fn progress_hours_total_multiple_tags() {
        let mut view = TestSkillView::default();
        view.add("2023-07-13 00:00 00:00 run. skill kind=physical. Running");
        view.add("2023-07-13 00:00 00:00 swim. skill kind=physical. Swimming");

        // Adding entry with multiple tags should be calculated once for total notifications
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(
                    Record::parse("2023-07-13 00:00 20:00 swim. run. Practices swimrun first time")
                        .unwrap()
                ),
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![
                SkillsNotification::LevelUp("Running level increased to 13".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - across all skills you've practiced 20 hours this week".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Running this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Running this week".to_string()
                ),
                SkillsNotification::LevelUp("Swimming level increased to 13".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Swimming this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Swimming this week".to_string()
                )
            ]
        );
    }

    #[test]
    fn delete_entry() {
        let mut view = TestSkillView::default();
        view.add("2023-07-13 00:00 00:00 run. skill kind=physical. Running");
        let entry = "2023-07-13 00:00 01:00 run";
        view.add(entry);
        view.check_skills(vec![("Running", 60)]);
        let record = Record::parse(entry).unwrap();

        // Delete a record should reset running to the initial value
        assert_eq!(
            view.check_notification(
                ChangeEvent::Replaced {
                    from: record.clone(),
                    to: Record::parse(&record.to_deleted_string()).unwrap()
                },
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![]
        );
        view.check_skills(vec![("Running", 0)]);
    }

    #[test]
    fn delete_skill() {
        let mut view = TestSkillView::default();
        let skill = "2023-07-13 00:00 00:00 run1. skill kind=physical. Running1";
        view.add(skill);
        view.add("2023-07-13 00:01 00:01 run2. skill kind=physical. Running2");
        view.add("2023-07-13 00:00 10:00 run1");
        view.check_skills(vec![("Running1", 600), ("Running2", 0)]);

        // Delete a skill should remove it from the skills and from the total calculations
        let record = Record::parse(skill).unwrap();
        assert_eq!(
            view.check_notification(
                ChangeEvent::Replaced {
                    from: record.clone(),
                    to: Record::parse(&record.to_deleted_string()).unwrap()
                },
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![]
        );
        view.check_skills(vec![("Running2", 0)]);

        // Add another 10 hours to ensure that notification about 20 hours of total is not triggered
        assert_eq!(
            view.check_notification(
                ChangeEvent::Added(Record::parse("2023-07-13 10:00 20:00 run2").unwrap()),
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![
                SkillsNotification::LevelUp("Running2 level increased to 8".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 10 hours of Running2 this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 10 hours of Running2 this week".to_string()
                ),
            ]
        );
        view.check_skills(vec![("Running2", 600)]);
    }

    #[test]
    fn replace_entry() {
        let mut view = TestSkillView::default();
        view.add("2023-07-13 00:00 00:00 run. skill kind=physical. Running");
        view.add("2023-07-13 00:00 01:00 run");
        let entry = "2023-07-13 01:00 02:00 run";
        view.add(entry);
        view.check_skills(vec![("Running", 120)]);

        // Replacing with bigger amounts
        assert_eq!(
            view.check_notification(
                ChangeEvent::Replaced {
                    from: Record::parse(entry).unwrap(),
                    to: Record::parse("2023-07-13 00:00 19:00 run").unwrap()
                },
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![
                SkillsNotification::LevelUp("Running level increased to 13".to_string()),
                SkillsNotification::HourProgress(
                    "Great job - across all skills you've practiced 20 hours this week".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Running this month".to_string()
                ),
                SkillsNotification::HourProgress(
                    "Great job - you've practiced 20 hours of Running this week".to_string()
                ),
            ]
        );
        view.check_skills(vec![("Running", 1200)]);

        // Replacing with smaller amounts
        assert_eq!(
            view.check_notification(
                ChangeEvent::Replaced {
                    from: Record::parse("2023-07-13 00:00 19:00 run").unwrap(),
                    to: Record::parse("2023-07-13 00:00 00:30 run").unwrap()
                },
                Some(DateDay::new(2023, 7, 13))
            ),
            vec![SkillsNotification::LevelUp(
                "Running level increased to 2".to_string()
            )]
        );
        view.check_skills(vec![("Running", 90)]);
    }

    #[test]
    fn add_to_previous_month() {
        let mut view = TestSkillView::default();
        view.add("2023-07-13 00:00 00:00 read. skill kind=intelligent. Reading");
        view.check_notification(
            ChangeEvent::Added(Record::parse("2023-11-30 17:00 20:00 read").unwrap()),
            Some(DateDay::new(2023, 12, 1)),
        );
    }
}
