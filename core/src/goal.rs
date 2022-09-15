use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::datetime::{DatePeriod, TimeDuration};
use crate::db::{Query, TagStats};
use crate::parser::ParseError;
use crate::record::{Prop, PropOperator, PropVal, Tag};

#[derive(PartialEq)]
pub struct Goal {
    pub aggregate: Aggregate,
    pub canceled: bool,
    pub comment: Option<String>,
    pub count: Option<(PropOperator, usize)>,
    pub duration: Option<TimeDuration>,
    pub period: DatePeriod,
    pub properties: Vec<Prop>,
    pub query: Option<Query>,
    pub level: usize,
}

impl Goal {
    pub fn is_meta(&self) -> bool {
        self.query.is_none() || self.query.as_ref().map(|v| v.query.is_empty()) == Some(true)
    }

    pub fn goal_progress(&self, _: Vec<TagStats>) -> GoalProgress {
        GoalProgress {
            name: "ok".to_string(),
            completion: 5,
            minutes_actual: 24,
            minutes_planned: 45,
        }
    }
    // pub fn target_in_days(&self, days_count: usize) -> (usize, Option<usize>) {
    //     let mut target = (0, None);
    //     let days_count = days_count as f32;
    //     let divider = match self.period {
    //         // TODO A lot of rounding errors here
    //         DatePeriod::Day => 1.0 / days_count,
    //         DatePeriod::Week => 7.0 / days_count,
    //         DatePeriod::Month => 30.0 / days_count,
    //         DatePeriod::Year => 365.0 / days_count,
    //     };
    //     target.0 = self
    //         .count
    //         .as_ref()
    //         .map(|v| {
    //             v.max
    //                 .map(|v| ((v as f32) * divider) as usize)
    //                 .unwrap_or_else(|| {
    //                     v.min
    //                         .map(|v| (v as f32 * divider) as usize)
    //                         .unwrap_or_default()
    //                 })
    //         })
    //         .unwrap_or_default();
    //     target.1 = self
    //         .duration
    //         .as_ref()
    //         .map(|v| (v.as_minutes() as f32 / divider) as usize);
    //     target
    // }

    pub fn create(tags: Vec<Tag>, comment: Option<String>) -> Result<Goal, ParseError> {
        Goal::create_with_level(tags, comment, 0)
    }

    pub fn create_with_level(
        tags: Vec<Tag>,
        comment: Option<String>,
        level: usize,
    ) -> Result<Goal, ParseError> {
        let mut goal = Goal {
            aggregate: Aggregate::Sum,
            canceled: false,
            comment,
            count: Option::None,
            duration: Option::None,
            // TODO Goal without period is invalid, but here we init it with default day
            period: DatePeriod::Day,
            properties: vec![],
            query: None,
            level,
        };
        let mut query: Query = Default::default();
        for tag in tags {
            if tag.name == "goal" {
                for prop in tag.props {
                    match prop.name.as_ref() {
                        "aggregate" => goal.aggregate = Goal::parse_aggregate(prop)?,
                        "cancelled" => goal.canceled = true,
                        "count" => goal.count = Some(Goal::parse_count(prop)?),
                        "duration" => goal.duration = Goal::parse_duration(prop)?,
                        "for" => goal.period = Goal::parse_period(prop)?,
                        _ => goal.properties.push(prop),
                    }
                }
            } else {
                query.query.push(tag);
            }
        }
        goal.query = Some(query);
        Ok(goal)
    }

    fn parse_count(prop: Prop) -> Result<(PropOperator, usize), ParseError> {
        if let PropVal::Number(n) = prop.val {
            return Ok((prop.operator, n as usize));
        }
        Err(ParseError::BadValue(
            "Goal count cannot be parsed".to_string(),
            prop.start_pos,
        ))
    }

    fn parse_aggregate(prop: Prop) -> Result<Aggregate, ParseError> {
        match prop.val {
            PropVal::None => Ok(Aggregate::Sum),
            PropVal::String(s) => match s.parse::<Aggregate>() {
                Ok(v) => Ok(v),
                Err(err) => Err(ParseError::BadValue(err, prop.start_pos)),
            },
            _ => Err(ParseError::BadValue(
                "Bad aggregate value".to_string(),
                prop.start_pos,
            )),
        }
    }

    fn parse_duration(prop: Prop) -> Result<Option<TimeDuration>, ParseError> {
        if let PropVal::Time(v) = prop.val {
            return Ok(Some(v));
        }
        Err(ParseError::BadQuery(
            "Goal duration cannot be parsed".to_string(),
            prop.start_pos,
        ))
    }

    fn parse_period(prop: Prop) -> Result<DatePeriod, ParseError> {
        let err = "`of` property value can be either week, month or year".to_string();
        let err = Err(ParseError::BadValue(err, prop.start_pos));
        if let PropVal::String(v) = prop.val {
            return match v.parse::<DatePeriod>() {
                Ok(v) => Ok(v),
                Err(_) => err,
            };
        }
        err
    }
}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        if let Some(q) = &self.query {
            if !q.query.is_empty() {
                let tags: Vec<String> = q.query.iter().map(|t| format!("{}", t)).collect();
                str.push_str(&tags.join(". "));
                str.push_str(". ");
            }
        }
        str.push_str("goal");
        str.push_str(" for ");
        str.push_str(&self.period.to_string());
        if let Some(v) = &self.count {
            str.push_str(" count");
            str.push_str(&v.0.to_string());
            str.push_str(&v.1.to_string());
        }
        if let Some(v) = &self.duration {
            str.push_str(" duration=");
            str.push_str(&v.to_string());
        }
        str.push_str(" level=");
        str.push_str(&self.level.to_string());

        if self.comment.is_some() {
            str.push_str(". ");
            str.push_str(&self.comment.as_ref().unwrap());
        }
        f.write_str(&str)
    }
}

#[derive(Debug, PartialEq)]
pub enum Aggregate {
    Last,
    Sum,
}

impl FromStr for Aggregate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_lowercase().as_str() {
            "sum" => Ok(Aggregate::Sum),
            "last" => Ok(Aggregate::Last),
            s => Err(format!("Unexpected aggregate value {}", s)),
        };
    }
}

// Goal progress stats
#[derive(Debug)]
pub struct GoalProgress {
    pub name: String,
    pub completion: usize,
    pub minutes_actual: usize,
    pub minutes_planned: usize,
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    fn parse(s: &str) -> Goal {
        let mut parser = Parser::new(s);
        match parser.parse_record() {
            Ok((tags, comment)) => Goal::create(tags, comment).unwrap(),
            Err(err) => unreachable!("{}", err.to_string()),
        }
    }

    #[test]
    fn parse_simple() {
        let goal = parse("run distance>50. goal for week duration=5:00");
        assert_eq!(goal.duration, Some(TimeDuration::new(5, 0)));
        assert_eq!(goal.period, DatePeriod::Week);
        assert_eq!(
            goal.query,
            Some(Query::new("run distance>50", None).unwrap())
        );
    }

    #[test]
    fn parse_min_and_custom() {
        let goal = parse("foo. goal for month duration=10:30 count=3 distance=50");
        assert_eq!(goal.duration, Some(TimeDuration::new(10, 30)));
        assert_eq!(goal.period, DatePeriod::Month);
        assert_eq!(goal.count, Some((PropOperator::Eq, 3)));
        assert_eq!(
            goal.properties,
            vec![Prop {
                name: "distance".to_string(),
                val: PropVal::Number(50.0),
                operator: PropOperator::Eq,
                start_pos: 0,
            }]
        );
    }

    #[test]
    fn parse_just_count() {
        let goal = parse("foo. goal for year count>10");
        assert_eq!(goal.period, DatePeriod::Year);
        assert_eq!(goal.duration, None);
        assert_eq!(goal.count, Some((PropOperator::More, 10)));
    }

    #[test]
    fn parse_aggregate() {
        let goal = parse("foo. goal for year income=1000 aggregate=last");
        assert_eq!(goal.aggregate, Aggregate::Last);
    }

    #[test]
    fn parse_cancelled() {
        let goal = parse("foo. goal for year cancelled");
        assert!(goal.canceled);
    }

    #[test]
    fn parse_comment() {
        let goal = parse("foo. goal for week. New goal");
        assert_eq!(goal.comment, Some("New goal".to_string()));
    }

    #[test]
    fn parse_selector() {
        let goal = parse("goal for week. Meta goal");
        assert_eq!(goal.comment, Some("Meta goal".to_string()));
        assert_eq!(goal.is_meta(), true);
    }

    #[test]
    fn parse_duration() {
        let goal = parse("goal for day duration=5:00");
        assert_eq!(goal.duration, Some(TimeDuration::new(5, 0)));
        let goal = parse("goal for day duration=51:24");
        assert_eq!(goal.duration, Some(TimeDuration::new(51, 24)));
        let goal = parse("goal for week duration=999:00");
        assert_eq!(goal.duration, Some(TimeDuration::new(999, 0)));
    }
}
