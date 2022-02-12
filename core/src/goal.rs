use std::fmt::{Display, Formatter};
use std::str::FromStr;

use datetime::{DatePeriod, TimeDuration};
use db::{Query, TagStats};
use parser::ParseError;
use record::{Prop, PropOperator, PropVal, Tag};

#[derive(PartialEq)]
pub struct Goal {
    pub aggregate: Aggregate,
    pub canceled: bool,
    pub comment: Option<String>,
    pub count: Option<(PropOperator, usize)>,
    pub duration: Option<TimeDuration>,
    pub period: DatePeriod,
    pub properties: Vec<Prop>,
    pub query: Query,
    str: String,
}

impl Goal {
    pub fn goal_progress(&self, tags: Vec<TagStats>) -> GoalProgress {
        todo!()
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
        let str: Vec<String> = tags.iter().map(|t| format!("{}", t)).collect();
        let mut str = str.join(" ");
        if comment.is_some() {
            str = format!("{}. {}", str, comment.clone().unwrap());
        }
        let mut goal = Goal {
            aggregate: Aggregate::Sum,
            canceled: false,
            comment,
            count: Option::None,
            duration: Option::None,
            // TODO Goal without period is invalid, but here we init it with default day
            period: DatePeriod::Day,
            properties: vec![],
            query: Default::default(),
            str,
        };
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
                goal.query.query.push(tag);
            }
        }
        if goal.query.query.is_empty() {
            return Err(ParseError::BadQuery(
                "Query is required for the goal".to_string(),
                0,
            ));
        }
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
        f.write_str(&self.str)
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
pub struct GoalProgress {
    pub name: String,
    pub completion: usize,
    pub minutes_actual: usize,
    pub minutes_planned: usize,
}

#[cfg(test)]
mod tests {
    use datetime::{Date, DateTime, DayTime};
    use record::Record;

    use super::*;

    const BASE_DATE: DateTime = DateTime::new(Date::new(2000, 1, 1), DayTime::new(0, 0));

    fn parse(s: &str) -> Goal {
        if let Ok(Record::Goal(goal)) = Record::from_string(&s, BASE_DATE) {
            return goal;
        }
        unreachable!()
    }

    #[test]
    fn parse_simple() {
        let goal = parse("00:01 run distance>50. goal for week duration=5:00");
        assert_eq!(goal.duration, Some(TimeDuration::new(5, 0)));
        assert_eq!(goal.period, DatePeriod::Week);
        assert_eq!(goal.query, Query::new("run distance>50", None).unwrap());
    }

    #[test]
    fn parse_min_and_custom() {
        let goal = parse("00:01 foo. goal for month duration=10:30 count=3 distance=50");
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
        let goal = parse("00:01 foo. goal for year count>10");
        assert_eq!(goal.period, DatePeriod::Year);
        assert_eq!(goal.duration, None);
        assert_eq!(goal.count, Some((PropOperator::More, 10)));
    }

    #[test]
    fn parse_aggregate() {
        let goal = parse("01:01 foo. goal for year income=1000 aggregate=last");
        assert_eq!(goal.aggregate, Aggregate::Last);
    }

    #[test]
    fn parse_cancelled() {
        let goal = parse("01:01 foo. goal for year cancelled");
        assert!(goal.canceled);
    }

    #[test]
    fn parse_comment() {
        let goal = parse("01:01 foo. goal for week. New goal");
        assert_eq!(goal.comment, Some("New goal".to_string()));
    }
}
