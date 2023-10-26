use std::{fmt::Display, str::FromStr};

use crate::{
    date_time::datetime::Duration,
    db::Selector,
    record::{Entry, PropVal, Tag},
};

/*
Skill - activity where we can become better by practicing.

Purpose - Introducing arbitrary checkpoints to have constant feeling of a progress.

Skill level - starts at 1 and increased by spending time on skill practice. Reaches level 100
after 10_000 hours, but can grow even after. First levels reached after just a few hours, the
further it goes the more time is needed to reach the new level. Purpose is to support early
development with frequent achievements. On average multiple progressions per week.

Skill examples: Running, Drums, Programming, Sculpture, etc.
*/

/// Skill represents progression of certain activity
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Skill {
    selector: Selector,
    kind: SkillKind,
    title: String,
    duration_minutes: u64,
    perfect_week: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum SkillKind {
    /// Activities that challenges your brain: academic writing, solving problems, learning languages
    Intelligent = 0,
    /// Activities that mainly target your body: running, strength training, pilates, dancing
    Physical = 1,
    /// Activities where you express yourselves: art, music, novel writing, dancing
    Creative = 2,
}

#[derive(Default, Debug)]
pub struct SkillProgress {
    pub level: usize,
    pub minutes_till_next: usize,
    pub duration_minutes: usize,
}

impl SkillProgress {
    pub fn new(duration_minutes: usize) -> Self {
        let (level, minutes_till_next) = skill_level(duration_minutes);
        SkillProgress {
            level,
            minutes_till_next,
            duration_minutes,
        }
    }
}

impl FromStr for SkillKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO Remove emoji support. Left from PoC phase and should not be used anymore by now
        match s {
            "physical" | "💪" => Ok(Self::Physical),
            "intelligent" | "🧠" => Ok(Self::Intelligent),
            "creative" | "🫀" => Ok(Self::Creative),
            _ => Err("Unknown skill kind".to_string()),
        }
    }
}

impl ToString for SkillKind {
    fn to_string(&self) -> String {
        match self {
            SkillKind::Physical => "physical".to_string(),
            SkillKind::Intelligent => "intelligent".to_string(),
            SkillKind::Creative => "creative".to_string(),
        }
    }
}

impl Skill {
    /// Creates Skill from given record if it is a `skill` tag with correct props
    pub fn from_record(record: &Entry) -> Option<Self> {
        if record.tags.iter().all(|v| v.name != "skill") {
            return None; // Most of the records will be non skills, early return in this case
        }
        let mut query = vec![];
        let mut skill_tag = None;
        for tag in &record.tags {
            if tag.name == "skill" {
                skill_tag = Some(tag);
            } else if tag.name != "entry" {
                // entry is a special internal tag which is not part of the query
                query.push(tag);
            }
        }
        let skill_tag = skill_tag?;
        let symbol = skill_tag.props.iter().find(|v| v.name == "kind")?;
        let mut perfect_week = 0;
        for prop in &skill_tag.props {
            if prop.name != "perfect" {
                continue;
            }
            if let PropVal::Number(perfect) = prop.val {
                perfect_week = perfect as u64;
                break;
            } else {
                // TODO Can it happen? I don't think we have validation in place
            }
        }
        Some(Skill {
            title: record.comment.as_ref().cloned()?,
            kind: symbol.val.to_string().parse().ok()?,
            selector: Selector {
                inclusive_tags: query.into_iter().cloned().collect(),
                exclusive_tags: vec![Tag::new("skill".to_string(), vec![], 0)], // skills entries should be excluded
            },
            duration_minutes: 0,
            perfect_week,
        })
    }

    /// Returns skill progress - current level and minutes till the next level
    pub fn progress(&self) -> SkillProgress {
        SkillProgress::new(self.duration_minutes as usize)
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn kind(&self) -> &SkillKind {
        &self.kind
    }

    pub fn perfect_week(&self) -> u64 {
        self.perfect_week
    }

    pub fn add_duration(&mut self, duration: Duration) {
        self.duration_minutes += duration.minutes();
    }

    pub fn remove_duration(&mut self, duration: Duration) {
        self.duration_minutes -= duration.minutes();
    }

    pub fn merge_selector(&mut self, mut another: Skill) {
        self.selector
            .inclusive_tags
            .append(&mut another.selector.inclusive_tags);
    }
}

impl Ord for Skill {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO Kind comparison depends on symbol code, better idea would be group skills
        //      by kind, but sort depending on best skill within the group. So that main
        //      group (with biggest skill) will be always on top
        self.kind
            .cmp(&other.kind)
            .then_with(|| self.duration_minutes.cmp(&other.duration_minutes).reverse())
            .then_with(|| self.title.cmp(&other.title))
    }
}

impl PartialOrd for Skill {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (level, _) = skill_level(self.duration_minutes as usize);
        f.write_fmt(format_args!(
            "{} {:015} {: >4}",
            self.kind.to_string(),
            self.title,
            level
        ))
    }
}

// Calculates skill level and time left before the next level
// Created in way to produce level 100 around 10_000 hours
// Levelling is fast at start, but higher levels require more time
fn skill_level(minutes: usize) -> (usize, usize) {
    let factor = 1.0673005;
    let mut level = 0;
    let mut total_minutes = 0.0;
    let mut hours_per_level = 1.0;
    while minutes >= total_minutes as usize {
        level += 1;
        hours_per_level *= factor;
        total_minutes += hours_per_level * 60.0;
    }
    (level, total_minutes as usize - minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_progression() {
        let time_level = vec![
            (0, (1, 64)), // Around an hour needed to reach level 2
            (63, (1, 1)),
            (64, (2, 68)),
            (60 * 10, (8, 50)),          // 10 hours is level 8
            (60 * 100, (31, 214)),       // 100 hours is level 31
            (60 * 1_000, (64, 533)),     // 1_000 hours is level 64
            (60 * 10_000, (100, 40392)), // 10_000 hours is level 100. 40_392/60 = 673 hours till level 101
        ];
        for (time, want) in time_level {
            let got = skill_level(time);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn matching() {
        let matches = |skill, entry| {
            let skill = Skill::from_record(&Entry::parse(skill).unwrap()).unwrap();
            let entry = Entry::parse(entry).unwrap();
            skill.selector.matches(&entry)
        };

        // Matching by tag
        assert!(matches(
            "2023-07-13 00:00 00:00 foo. skill kind=physical. Foo",
            "2023-07-13 01:00 02:00 foo"
        ));

        // No match
        assert!(!matches(
            "2023-07-13 00:00 00:00 foo. skill kind=physical. Foo",
            "2023-07-13 01:00 02:00 bar"
        ));

        // Multi tag selector
        assert!(matches(
            "2023-07-13 00:00 00:00 foo. bar. skill kind=physical. Foo",
            "2023-07-13 01:00 02:00 foo"
        ));

        // Entry tag is ignored
        assert!(!matches(
            "2023-07-13 00:00 00:00 foo. entry revision=2. skill kind=physical. Foo",
            "2023-07-13 00:00 00:00 bar. entry revision=2",
        ));

        // No match on skill entries
        assert!(!matches(
            "2023-07-13 00:00 00:00 foo. skill kind=physical. Foo",
            "2023-07-13 00:00 00:00 foo. bar. skill kind=physical. Foo2",
        ));
    }
}
