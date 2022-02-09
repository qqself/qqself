use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};
use std::str::FromStr;

use datetime::{DatePeriod, DateTime, DateTimeRange, Time};
use db::Query;
use parser::{ParseError, Parser};

// Record represent parsed text line
#[derive(PartialEq)]
pub enum Record {
    // Entry record - new data input
    Entry(Entry),

    // Goal record - new goal set or canceled
    Goal(Goal),
}

impl Record {
    pub fn from_string(input: &str, relevant_to: DateTime) -> Result<Record, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse_record(relevant_to)
    }
}

#[derive(PartialEq, Clone)]
pub struct Entry {
    pub tags: Vec<Tag>,
    pub comment: Option<String>,
    pub date_range: DateTimeRange,
}

impl Entry {
    pub(crate) fn new(date_range: DateTimeRange, comment: Option<String>, tags: Vec<Tag>) -> Self {
        Entry {
            date_range,
            comment,
            tags,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tags: Vec<String> = self.tags.iter().map(|t| format!("{}", t)).collect();
        let mut s = String::new();
        s.push_str(&self.date_range.start.to_string());
        s.push_str(" - ");
        s.push_str(&self.date_range.end.to_string());
        s.push(' ');
        s.push_str(&tags.join(". "));
        if self.comment.is_some() {
            s.push_str(". ");
            // Escape line breaks in the comment
            let comment = self.comment.as_ref().unwrap();
            let escaped = comment.replace('\n', "\\n");
            s.push_str(&escaped);
        }
        write!(f, "{}", s)
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub val: Vec<Prop>,
    pub start_pos: usize,
}

impl Tag {
    pub fn new(name: String, val: Vec<Prop>, start_pos: usize) -> Self {
        Tag {
            name,
            val,
            start_pos,
        }
    }
    pub fn matches(&self, query: &Tag) -> bool {
        if self.name != query.name {
            return false;
        }
        if query.val.is_empty() {
            return true;
        }
        for query_prop in &query.val {
            for tag_prop in &self.val {
                if tag_prop.matches(query_prop) {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        for prop in &self.val {
            f.write_char(' ')?;
            f.write_str(&prop.to_string())?;
        }
        std::fmt::Result::Ok(())
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(PartialEq, Clone)]
pub struct Prop {
    pub name: String,
    pub val: PropVal,
    pub operator: PropOperator,
    pub start_pos: usize,
}

impl Prop {
    pub fn matches(&self, query: &Prop) -> bool {
        if self.name != query.name {
            return false;
        }
        match query.operator {
            PropOperator::Eq => self.val == query.val,
            PropOperator::Less => self.val < query.val,
            PropOperator::More => self.val > query.val,
        }
    }
}

impl Display for Prop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        if self.val != PropVal::None {
            f.write_str(&self.operator.to_string())?;
            f.write_str(&self.val.to_string())?;
        }
        std::fmt::Result::Ok(())
    }
}

impl Debug for Prop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropOperator {
    Eq,
    Less,
    More,
}

impl Display for PropOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PropOperator::Eq => '=',
            PropOperator::Less => '<',
            PropOperator::More => '>',
        })
    }
}

#[derive(PartialEq, Clone)]
pub enum PropVal {
    None,           // No value for property
    Number(f32),    // For simplicity we use f32 for both floats and integers
    Time(u8, u8),   // Time in \d\d:\d\d format. Hours or minutes scale is not known
    String(String), // Anything else
}

impl PropVal {
    pub(crate) fn parse(s: &str) -> PropVal {
        if s.is_empty() {
            return PropVal::None;
        }
        if let Ok(v) = s.parse::<f32>() {
            return PropVal::Number(v);
        }
        if let Ok(time) = s.parse::<Time>() {
            return PropVal::Time(time.hours, time.minutes);
        }
        PropVal::String(s.to_string())
    }
}

impl Display for PropVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropVal::None => std::fmt::Result::Ok(()),
            PropVal::Number(n) => f.write_str(&n.to_string()),
            PropVal::Time(n, m) => f.write_fmt(format_args!("{}:{}", n.to_string(), m.to_string())),
            PropVal::String(s) => f.write_str(s),
        }
    }
}

impl Debug for PropVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialOrd for PropVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PropVal::None, PropVal::None) => Some(Ordering::Equal),
            (PropVal::String(s1), PropVal::String(s2)) => s1.partial_cmp(s2),
            (PropVal::Number(n1), PropVal::Number(n2)) => n1.partial_cmp(n2),
            (PropVal::Time(n1, m1), PropVal::Time(n2, m2)) => {
                let cmp1 = n1.cmp(n2);
                if cmp1 != Ordering::Equal {
                    return Some(cmp1);
                }
                m1.partial_cmp(m2)
            }
            (_, _) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Aggregate {
    Average,
    Sum,
    Last,
}

impl FromStr for Aggregate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_lowercase().as_str() {
            "average" => Ok(Aggregate::Average),
            "sum" => Ok(Aggregate::Sum),
            "last" => Ok(Aggregate::Last),
            s => Err(format!("Unexpected aggregate value {}", s)),
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Count {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(PartialEq)]
pub struct Goal {
    pub aggregate: Aggregate,
    pub canceled: bool,
    pub comment: Option<String>,
    pub count: Option<Count>,
    pub duration: Option<Time>,
    pub period: DatePeriod,
    pub properties: Vec<Prop>,
    pub query: Query,
    pub(crate) str: String, // TODO There has to be a simple way to get string representation in runtime
}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.str);
        if self.comment.is_some() {
            s.push(' ');
            // Escape line breaks in the comment
            let comment = self.comment.as_ref().unwrap();
            let escaped = comment.replace('\n', "\\n");
            s.push_str(&escaped);
        }
        f.write_str(&s)
    }
}
