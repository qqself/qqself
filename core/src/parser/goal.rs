use parser::parser::{ParseError, Prop, Tag};

#[derive(Debug, PartialEq)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    fn from_string(s: &str) -> Option<Period> {
        return match s.to_lowercase().as_str() {
            "day" => Some(Period::Day),
            "week" => Some(Period::Week),
            "month" => Some(Period::Month),
            "year" => Some(Period::Year),
            _ => None,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum Aggregate {
    Average,
    Sum,
    Last,
}

impl Aggregate {
    fn from_string(s: &str) -> Option<Aggregate> {
        return match s.to_lowercase().as_str() {
            "average" => Some(Aggregate::Average),
            "sum" => Some(Aggregate::Sum),
            "last" => Some(Aggregate::Last),
            _ => None,
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Count {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Goal {
    pub aggregate: Aggregate,
    pub canceled: bool,
    pub comment: Option<String>,
    pub count: Option<Count>,
    pub duration: Option<(usize, usize)>,
    pub period: Period,
    pub properties: Vec<Prop>,
    pub query: Vec<Tag>,
}

impl Goal {
    pub fn create(tags: Vec<Tag>, comment: Option<String>) -> Result<Goal, ParseError> {
        let mut goal = Goal {
            aggregate: Aggregate::Sum,
            canceled: false,
            comment,
            count: Option::None,
            duration: Option::None,
            period: Period::Day,
            properties: vec![],
            query: vec![],
        };
        for tag in tags {
            if tag.name == "goal" {
                for prop in tag.val {
                    match prop.name.as_ref() {
                        "type" => goal.aggregate = Goal::parse_aggregate(prop)?,
                        "cancelled" => goal.canceled = true,
                        "min" => {
                            if goal.count.is_none() {
                                goal.count = Some(Count {
                                    min: None,
                                    max: None,
                                })
                            }
                            goal.count.as_mut().unwrap().min = Some(Goal::parse_count(prop)?);
                        }
                        // TODO Remove duplication
                        "max" => {
                            if goal.count.is_none() {
                                goal.count = Some(Count {
                                    min: None,
                                    max: None,
                                })
                            }
                            goal.count.as_mut().unwrap().max = Some(Goal::parse_count(prop)?);
                        }
                        "duration" => goal.duration = Goal::parse_duration(prop)?,
                        "for" => goal.period = Goal::parse_period(prop)?,
                        _ => goal.properties.push(prop),
                    }
                }
            } else {
                goal.query.push(tag);
            }
        }
        if goal.query.is_empty() {
            return Err(ParseError::BadQuery(
                "Query is required for the goal".to_string(),
                0,
            ));
        }
        Ok(goal)
    }

    fn parse_count(prop: Prop) -> Result<usize, ParseError> {
        let err = "goal min/max value should be an integer";
        if prop.val.is_none() {
            return Err(ParseError::BadValue(err.to_string(), prop.start_pos));
        }
        return match prop.val.as_ref().unwrap().parse::<usize>() {
            Ok(index) => Ok(index),
            Err(_) => Err(ParseError::BadValue(err.to_string(), prop.start_pos)),
        };
    }

    fn parse_aggregate(prop: Prop) -> Result<Aggregate, ParseError> {
        if prop.val.is_none() {
            return Ok(Aggregate::Sum);
        }
        match Aggregate::from_string(&prop.val.as_ref().unwrap().to_lowercase()) {
            Some(aggregate) => Ok(aggregate),
            None => {
                let err = "goal aggregate should be one of: average, sum, last";
                Err(ParseError::BadValue(err.to_string(), prop.start_pos))
            }
        }
    }

    fn parse_duration(prop: Prop) -> Result<Option<(usize, usize)>, ParseError> {
        if prop.val.is_none() {
            return Ok(None);
        }
        let err = || {
            Err(ParseError::BadValue(
                "goal duration should be in hours:minutes format, example 5:00".to_string(),
                prop.start_pos,
            ))
        };
        let val: Vec<&str> = prop.val.as_ref().unwrap().split(':').collect();
        if val.len() != 2 {
            return err();
        }
        let hours = val[0].parse::<usize>();
        let minutes = val[1].parse::<usize>();
        if hours.is_err() || minutes.is_err() {
            return err();
        }
        Ok(Some((hours.unwrap(), minutes.unwrap())))
    }

    fn parse_period(prop: Prop) -> Result<Period, ParseError> {
        if prop.val.is_none() {
            let err = "`of` property is required for `goal` tag".to_string();
            return Err(ParseError::MissingProperty(err, prop.start_pos));
        }
        if let Some(period) = prop.val.and_then(|v| Period::from_string(&v)) {
            return Ok(period);
        }
        let err = "`of` property value can be either week, month or year".to_string();
        Err(ParseError::BadValue(err, prop.start_pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::parser::PropOperator;
    use parser::Record;

    fn parse(s: &str) -> Goal {
        // Hack Goal is an entry still and requires time
        let s = format!("2000-01-01 00:00 {}", s);
        match Record::from_string(&s, "", "") {
            Ok(record) => match record {
                Record::Entry(_) => unreachable!("Goal is expected"),
                Record::Goal(goal) => goal,
            },
            Err(err) => unreachable!("Valid goal is expected: {:?}", err),
        }
    }

    #[test]
    fn parse_simple() {
        let goal = parse("run distance>50. goal for week duration=5:00");
        assert_eq!(goal.duration, Some((5, 0)));
        assert_eq!(goal.period, Period::Week);
        assert_eq!(
            goal.query,
            vec![Tag {
                name: "run".to_string(),
                val: vec![Prop {
                    name: "distance".to_string(),
                    val: Some("50".to_string()),
                    operator: PropOperator::More,
                    start_pos: 20,
                }],
                start_pos: 17
            }]
        );
    }

    #[test]
    fn parse_min_and_custom() {
        let goal = parse("foo. goal for month duration=10:30 min=3 distance=50");
        assert_eq!(goal.duration, Some((10, 30)));
        assert_eq!(goal.period, Period::Month);
        assert_eq!(goal.count.unwrap().min, Some(3));
        assert_eq!(
            goal.properties,
            vec![Prop {
                name: "distance".to_string(),
                val: Some("50".to_string()),
                operator: PropOperator::Eq,
                start_pos: 57,
            }]
        );
    }

    #[test]
    fn parse_just_count() {
        let goal = parse("foo. goal for year max=10");
        assert_eq!(goal.period, Period::Year);
        assert_eq!(goal.duration, None);
        assert_eq!(goal.count.unwrap().max, Some(10));
    }

    #[test]
    fn parse_aggregate() {
        let goal = parse("foo. goal for year income=1000 type=last");
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
}
