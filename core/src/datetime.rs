use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::AddAssign;
use std::str::FromStr;

// I'm still not sure about adding chrono as core has to be as light as possible to
// be able to use anywhere. Let's have our own Date and Time wrappers and see how
// far we would go. Actually it maybe makes no sense as chrono supports even no_std
#[derive(PartialEq, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:0>4}-{:0>2}-{:0>2}",
            self.year, self.month, self.day
        ))
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Err(err) = check_format(s, vec!['d', 'd', 'd', 'd', '-', 'd', 'd', '-', 'd', 'd']) {
            return Err(err);
        }
        let year = parse_number(&s[0..4], 2000u16, 3000u16)?;
        let month = parse_number(&s[5..7], 1, 12)?;
        let day = parse_number(&s[8..10], 1, 31)?;
        Ok(Date { year, month, day })
    }
}

impl Eq for Date {}

impl PartialOrd<Self> for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        let year = self.year.cmp(&other.year);
        if year != Ordering::Equal {
            return year;
        }
        let month = self.month.cmp(&other.month);
        if month != Ordering::Equal {
            return month;
        }
        let day = self.day.cmp(&other.day);
        if day != Ordering::Equal {
            return day;
        }
        Ordering::Equal
    }
}

#[derive(PartialEq, Clone)]
pub struct DayTime {
    pub hours: u8,
    pub minutes: u8,
}

impl DayTime {
    pub const fn new(hours: u8, minutes: u8) -> Self {
        DayTime { hours, minutes }
    }
    pub fn as_minutes(&self) -> u64 {
        (self.hours as u64) * 60 + self.minutes as u64
    }
    pub fn as_seconds(&self) -> u64 {
        self.as_minutes() * 60
    }
}

impl Display for DayTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:0>2}:{:0>2}", self.hours, self.minutes))
    }
}

impl Debug for DayTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for DayTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Err(err) = check_format(s, vec!['d', 'd', ':', 'd', 'd']) {
            return Err(err);
        }
        let hours = parse_number(&s[0..2], 0, 23)?;
        let minutes = parse_number(&s[3..5], 0, 59)?;
        Ok(DayTime { hours, minutes })
    }
}

impl Eq for DayTime {}

impl PartialOrd<Self> for DayTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DayTime {
    fn cmp(&self, other: &Self) -> Ordering {
        let hours = self.hours.cmp(&other.hours);
        if hours != Ordering::Equal {
            return hours;
        }
        let minutes = self.minutes.cmp(&other.minutes);
        if minutes != Ordering::Equal {
            return minutes;
        }
        Ordering::Equal
    }
}

#[derive(PartialEq, Clone)]
pub struct TimeDuration {
    pub measure1: usize,
    pub measure2: usize,
}

impl TimeDuration {
    pub const fn new(measure1: usize, measure2: usize) -> Self {
        TimeDuration { measure1, measure2 }
    }
}

impl Default for TimeDuration {
    fn default() -> Self {
        TimeDuration::new(0, 0)
    }
}

impl Display for TimeDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:0>2}:{:0>2}", self.measure1, self.measure2))
    }
}

impl Debug for TimeDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for TimeDuration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We assume that time duration is anything that looks time: From 0:01 to 99:99
        let sep = match s.find(':') {
            Some(idx) => idx,
            None => return Err("Time duration has to be digits separated by :".to_string()),
        };
        let measure1 = parse_number(&s[0..sep], 0, 99)?;
        let measure2 = parse_number(&s[sep + 1..s.len()], 0, 99)?;
        Ok(TimeDuration { measure1, measure2 })
    }
}

impl Eq for TimeDuration {}

impl PartialOrd<Self> for TimeDuration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeDuration {
    fn cmp(&self, other: &Self) -> Ordering {
        let measure1 = self.measure1.cmp(&other.measure1);
        if measure1 != Ordering::Equal {
            return measure1;
        }
        let measure2 = self.measure2.cmp(&other.measure2);
        if measure2 != Ordering::Equal {
            return measure2;
        }
        Ordering::Equal
    }
}

impl AddAssign for TimeDuration {
    fn add_assign(&mut self, rhs: Self) {
        let total = self.measure1 * 60 + self.measure2 + rhs.measure1 * 60 + rhs.measure2;
        self.measure1 = total / 60;
        self.measure2 = total - self.measure1 * 60;
    }
}

#[derive(PartialEq, Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: DayTime,
}

impl DateTime {
    pub(crate) const fn new(date: Date, time: DayTime) -> Self {
        DateTime { date, time }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.date, self.time))
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Eq for DateTime {}

impl PartialOrd<Self> for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        let date = self.date.cmp(&other.date);
        if date != Ordering::Equal {
            return date;
        }
        let time = self.time.cmp(&other.time);
        if time != Ordering::Equal {
            return time;
        }
        Ordering::Equal
    }
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err(format!(
                "Expected datetime string of length {}, got {} for value {}",
                16,
                s.len(),
                s
            ));
        }
        let date = (&s[0..10]).parse::<Date>()?;
        let time = (&s[11..16]).parse::<DayTime>()?;
        Ok(DateTime { date, time })
    }
}

#[derive(PartialEq, Clone)]
pub struct DateTimeRange {
    pub start: DateTime,
    pub end: DateTime,
}

impl DateTimeRange {
    fn new(start: DateTime, end: DateTime) -> Self {
        DateTimeRange { start, end }
    }
    pub fn duration(&self) -> TimeDuration {
        if self.start.date != self.end.date {
            unimplemented!("Duration can only be calculated for same days ranges");
        }
        let minutes = (self.end.time.as_minutes() - self.start.time.as_minutes()) as usize;
        let hours = minutes / 60;
        TimeDuration::new(hours, minutes - hours * 60)
    }
}

impl Display for DateTimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.start, self.end))
    }
}

impl Debug for DateTimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(PartialEq)]
pub enum DatePeriod {
    Day,
    Week,
    Month,
    Year,
}

impl Display for DatePeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DatePeriod::Day => "day",
            DatePeriod::Week => "week",
            DatePeriod::Month => "month",
            DatePeriod::Year => "year",
        })
    }
}

impl Debug for DatePeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for DatePeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "day" => Ok(DatePeriod::Day),
            "week" => Ok(DatePeriod::Week),
            "month" => Ok(DatePeriod::Month),
            "year" => Ok(DatePeriod::Year),
            s => Err(format!("Date period cannot be parsed from {}", s)),
        }
    }
}

fn parse_number<T: FromStr + Ord + Display>(s: &str, min: T, max: T) -> Result<T, String> {
    let parsed = match s.parse::<T>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(format!("Cannot parse {}", s)),
    };
    if parsed < min || parsed > max {
        return Err(format!(
            "Value is out of range of {}..{}, got {}",
            min, max, parsed
        ));
    };
    Ok(parsed)
}

fn check_format(s: &str, format: Vec<char>) -> Result<(), String> {
    let mut idx = 0;
    for c in s.chars() {
        if idx == format.len() {
            return Err(format!(
                "String is longer than expected length of {}",
                format.len()
            ));
        }
        let expected = format[idx];
        if expected == 'd' && !c.is_digit(10) {
            return Err(format!("Expected digit at index {}, got {}", idx, c));
        }
        if expected != 'd' && expected != c {
            return Err(format!("Expected {} at index {}, got {}", expected, idx, c));
        }
        idx += 1;
    }
    if idx != format.len() {
        return Err(format!(
            "Expected string length of {}, got {}",
            format.len(),
            idx
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {
        assert_eq!(Date::new(2022, 5, 9).to_string(), "2022-05-09")
    }

    #[test]
    fn date_parse() {
        assert_eq!("2022-05-09".parse::<Date>().unwrap(), Date::new(2022, 5, 9));
        assert_eq!("2022-01-01".parse::<Date>().unwrap(), Date::new(2022, 1, 1));
        assert_eq!(
            "2022-12-31".parse::<Date>().unwrap(),
            Date::new(2022, 12, 31)
        );
        assert!("2022-13-31".parse::<Date>().is_err());
        assert!("2022-12-32".parse::<Date>().is_err());
        assert!("2022-00-09".parse::<Date>().is_err());
        assert!("2022-13-09".parse::<Date>().is_err());
        assert!("2022-09-32".parse::<Date>().is_err());
    }

    #[test]
    fn date_compare() {
        let date = Date::new(2022, 5, 5);
        assert!(date < Date::new(2022, 5, 6));
        assert!(date < Date::new(2022, 6, 1));
        assert!(date < Date::new(2023, 1, 1));
    }

    #[test]
    fn time_conversion() {
        assert_eq!(DayTime::new(1, 12).as_minutes(), 72);
        assert_eq!(DayTime::new(1, 12).as_seconds(), 72 * 60);
    }

    #[test]
    fn time_display() {
        assert_eq!(DayTime::new(1, 1).to_string(), "01:01");
    }

    #[test]
    fn time_parse() {
        assert_eq!("01:01".parse::<DayTime>().unwrap(), DayTime::new(1, 1));
        assert_eq!("23:59".parse::<DayTime>().unwrap(), DayTime::new(23, 59));
        assert!("24:00".parse::<DayTime>().is_err());
        assert!("00:60".parse::<DayTime>().is_err());
    }

    #[test]
    fn time_compare() {
        let time = DayTime::new(10, 10);
        assert!(time < DayTime::new(10, 11));
        assert!(time < DayTime::new(20, 0));
    }

    #[test]
    fn datetime_format() {
        let datetime = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 49));
        assert_eq!(datetime.to_string(), "2022-11-23 12:49")
    }

    #[test]
    fn datetime_parse() {
        let want = DateTime::new(Date::new(2020, 1, 30), DayTime::new(00, 00));
        let got = "2020-01-30 00:00".parse::<DateTime>().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn datetime_compare() {
        let datetime = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 49));
        let datetime_time = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 50));
        assert!(datetime < datetime_time);
        let datetime_date = DateTime::new(Date::new(2022, 12, 23), DayTime::new(12, 49));
        assert!(datetime < datetime_date);
    }

    #[test]
    fn datetimerange_format() {
        let start = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 49));
        let end = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 55));
        let range = DateTimeRange::new(start, end);
        assert_eq!(range.to_string(), "2022-11-23 12:49 2022-11-23 12:55");
    }

    #[test]
    fn datetimerange_duration() {
        let from = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 49));
        let to = DateTime::new(Date::new(2022, 11, 23), DayTime::new(12, 55));
        let range = DateTimeRange::new(from, to);
        assert_eq!(range.duration(), TimeDuration::new(0, 6));
    }

    #[test]
    fn timeduration_add() {
        let mut v = TimeDuration::new(9, 49);
        v += TimeDuration::new(2, 15);
        assert_eq!(v, TimeDuration::new(12, 4));
    }

    #[test]
    fn dateperiod_format() {
        assert_eq!(DatePeriod::Day.to_string(), "day");
        assert_eq!(DatePeriod::Week.to_string(), "week");
    }

    #[test]
    fn dateperiod_parse() {
        assert_eq!("day".parse::<DatePeriod>().unwrap(), DatePeriod::Day);
        assert_eq!("Year".parse::<DatePeriod>().unwrap(), DatePeriod::Year);
    }
}
