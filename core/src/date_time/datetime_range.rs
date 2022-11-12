use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use super::datetime::{DateTime, Duration};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct DateTimeRange {
    start: DateTime,
    end: DateTime,
}

impl DateTimeRange {
    /// Creates new DateTimeRange, returns error when end is less than start
    pub fn new(start: DateTime, end: DateTime) -> Result<Self, &'static str> {
        if end < start {
            return Err("end time cannot be before the start");
        }
        Ok(Self { start, end })
    }
    pub fn start(&self) -> DateTime {
        self.start
    }
    pub fn end(&self) -> DateTime {
        self.end
    }
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

impl Display for DateTimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.start, self.end))
    }
}

impl Ord for DateTimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for DateTimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::date_time::datetime::{Date, Time};

    use super::*;

    #[test]
    fn datetimerange_format() {
        let start = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 49));
        let end = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 55));
        let range = DateTimeRange::new(start, end).unwrap();
        assert_eq!(range.to_string(), "2022-11-23 12:49 2022-11-23 12:55");
    }

    #[test]
    fn datetimerange_duration() {
        let from = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 49));
        let to = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 55));
        let range = DateTimeRange::new(from, to).unwrap();
        assert_eq!(range.duration(), Duration::new(0, 6));
    }
}
