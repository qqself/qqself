use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};

use crate::date_time::datetime::Duration;
use crate::date_time::datetime_range::DateTimeRange;
use crate::parser::{ParseError, Parser};

#[derive(Clone, Eq, PartialEq)]
pub struct Entry {
    // TODO Remove public
    pub(crate) tags: Vec<Tag>,
    pub(crate) comment: Option<String>,
    pub(crate) date_range: DateTimeRange,
}

impl Entry {
    #[allow(dead_code)]
    pub(crate) fn new(date_range: DateTimeRange, comment: Option<String>, tags: Vec<Tag>) -> Self {
        Entry {
            date_range,
            comment,
            tags,
        }
    }
    // TODO Not sure if we need it
    #[allow(dead_code)]
    pub(crate) fn parse(input: &str) -> Result<Entry, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse_date_record()
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn date_range(&self) -> &DateTimeRange {
        &self.date_range
    }
    /// Returns string representation of an entry but used only time for datetime ranges
    /// Useful for rendering data when date is visible via other means e.g. journal view
    pub fn to_string_short(&self) -> String {
        self.serialize(true)
    }
    fn serialize(&self, omit_dates: bool) -> String {
        let tags: Vec<String> = self.tags.iter().map(|t| format!("{}", t)).collect();
        let mut s = String::new();
        let start = if omit_dates {
            self.date_range.start().time().to_string()
        } else {
            self.date_range.start().to_string()
        };
        s.push_str(&start);
        s.push(' ');
        s.push('-');
        s.push(' ');
        let end = if omit_dates {
            self.date_range.end().time().to_string()
        } else {
            self.date_range.end().to_string()
        };
        s.push_str(&end);
        s.push(' ');
        s.push_str(&tags.join(". "));
        if let Some(comment) = &self.comment {
            s.push_str(". ");
            // Escape line breaks in the comment
            let escaped = comment.replace('\n', "\\n");
            s.push_str(&escaped);
        }
        s
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.serialize(false))
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date_range
            .cmp(&other.date_range)
            // TODO Hack for now, implement proper ordering for all intermediate objects
            .then_with(|| self.to_string().cmp(&other.to_string()))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq)]
pub struct Tag {
    pub name: String,
    pub props: Vec<Prop>,
    pub start_pos: usize,
}

impl Tag {
    pub fn new(name: String, props: Vec<Prop>, start_pos: usize) -> Self {
        Tag {
            name,
            props,
            start_pos,
        }
    }
    pub fn matches(&self, query: &Tag) -> bool {
        if self.name != query.name {
            return false;
        }
        if query.props.is_empty() {
            return true;
        }
        for query_prop in &query.props {
            for tag_prop in &self.props {
                if tag_prop.matches(query_prop) {
                    return true;
                }
            }
        }
        false
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.props == other.props
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        for prop in &self.props {
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

#[derive(Clone, Eq)]
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

impl PartialEq for Prop {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.val == other.val && self.operator == other.operator
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

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
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
    Time(Duration), // Time duration
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
        if let Ok(time) = s.parse::<Duration>() {
            return PropVal::Time(time);
        }
        PropVal::String(s.to_string())
    }
}

impl Display for PropVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropVal::None => std::fmt::Result::Ok(()),
            PropVal::Number(n) => f.write_fmt(format_args!("{:.1}", n)),
            PropVal::Time(time) => f.write_str(&time.to_string()),
            PropVal::String(s) => f.write_str(s),
        }
    }
}

impl PartialOrd for PropVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PropVal::None, PropVal::None) => Some(Ordering::Equal),
            (PropVal::String(s1), PropVal::String(s2)) => s1.partial_cmp(s2),
            (PropVal::Number(n1), PropVal::Number(n2)) => n1.partial_cmp(n2),
            (PropVal::Time(time1), PropVal::Time(time2)) => time1.partial_cmp(time2),
            (_, _) => None,
        }
    }
}

impl Eq for PropVal {}
