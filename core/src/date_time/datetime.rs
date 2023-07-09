use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    ops::Sub,
    str::FromStr,
};

/// Date time range with start and end, format YYYY-MM-DD HH:MM - YYYY-MM-DD HH:MM
/// If day is the same then short notation format is supported: YYYY-MM-DD HH:MM HH:MM
#[derive(PartialEq, Clone, Copy, Eq)]
pub struct DateTimeRange {
    start: DateTime,
    end: DateTime,
}

impl DateTimeRange {
    pub const SIZE_LONG: usize = 35;
    pub const SIZE_SHORT: usize = 22;
    pub const SIZE_SEPARATOR: usize = 3;

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
        if self.start.date() == self.end.date() {
            f.write_fmt(format_args!("{} {}", self.start, self.end.time()))
        } else {
            f.write_fmt(format_args!("{} - {}", self.start, self.end))
        }
    }
}

impl Debug for DateTimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
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

impl FromStr for DateTimeRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s[0..DateTime::SIZE].parse::<DateTime>()?;
        if s.len() == Self::SIZE_LONG {
            let end = s[DateTime::SIZE + Self::SIZE_SEPARATOR..].parse::<DateTime>()?;
            Self::new(start, end).map_err(|v| v.to_string())
        } else if s.len() == Self::SIZE_SHORT {
            let time = s[DateTime::SIZE + 1..].parse::<Time>()?;
            let end = DateTime::new(start.date(), time);
            Self::new(start, end).map_err(|v| v.to_string())
        } else {
            Err("Not supported date time range length of the string".to_string())
        }
    }
}

// Date and time, format YYYY-MM-DD HH:MM
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct DateTime(time::PrimitiveDateTime);

impl DateTime {
    pub const SIZE: usize = 16;
    // There is no way to get local timezone using `time` on Unix/Mac https://github.com/time-rs/time/issues/325
    #[cfg(feature = "wasm")]
    pub fn now() -> Self {
        let now = time::OffsetDateTime::now_local().unwrap();
        DateTime::new(DateDay(now.date()), Time(now.time()))
    }
    pub fn new(date: DateDay, time: Time) -> Self {
        let datetime = time::PrimitiveDateTime::new(date.0, time.0);
        Self(datetime)
    }
    pub fn date(&self) -> DateDay {
        DateDay(self.0.date())
    }
    pub fn time(&self) -> Time {
        Time(self.0.time())
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let date = DateDay(self.0.date());
        let time = Time(self.0.time());
        f.write_fmt(format_args!("{} {}", date, time))
    }
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = s[0..10].parse::<DateDay>()?;
        let time = s[11..].parse::<Time>()?;
        Ok(DateTime::new(date, time))
    }
}

impl Sub<DateTime> for DateTime {
    type Output = Duration;

    fn sub(self, rhs: DateTime) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

/// Date, format YYYY-MM-DD
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
// HACK wasm_pack refuses to export `Date` because of name conflict with existing `Date` in JS
//      https://github.com/rustwasm/wasm-bindgen/issues/2798, so using slightly different name
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct DateDay(time::Date);

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl DateDay {
    #[cfg(feature = "wasm")]
    #[allow(non_snake_case)]
    pub fn fromDate(date: &js_sys::Date) -> Self {
        let month = (date.get_month() + 1).try_into().expect("invalid month");
        let year = date.get_full_year() as i32;
        let day = date.get_date().try_into().expect("invalid day");
        Self::new(year, month, day)
    }
    #[cfg(feature = "wasm")]
    #[allow(non_snake_case)]
    pub fn toString(&self) -> String {
        self.to_string()
    }

    #[allow(unused)]
    pub(crate) fn new(year: i32, month: u8, day: u8) -> Self {
        let month = time::Month::try_from(month).expect("invalid month number");
        let date = time::Date::from_calendar_date(year, month, day).expect("invalid date");
        DateDay(date)
    }
    pub fn remove_days(&self, days: usize) -> DateDay {
        let res = self.0.saturating_add(time::Duration::days(-(days as i64)));
        Self(res)
    }
    pub fn add_days(&self, days: usize) -> DateDay {
        let res = self.0.saturating_add(time::Duration::days(days as i64));
        Self(res)
    }
    pub fn year(&self) -> usize {
        self.0.year() as usize
    }
    pub fn month(&self) -> usize {
        self.0.month() as usize
    }
    pub fn day(&self) -> usize {
        self.0.day() as usize
    }
    pub fn days_from_monday(&self) -> u8 {
        self.0.weekday().number_days_from_monday()
    }
    pub fn as_start_of_week(&self) -> Self {
        self.remove_days(self.days_from_monday().into())
    }
    pub fn as_start_of_month(&self) -> Self {
        self.remove_days(self.day() - 1)
    }
    pub fn as_start_of_year(&self) -> Self {
        DateDay::new(
            self.year()
                .try_into()
                .expect("Cannot get a year from DateDay"),
            1,
            1,
        )
    }
}

impl Display for DateDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:04}-{:02}-{:02}",
            self.0.year(),
            self.0.month() as u8,
            self.0.day()
        ))
    }
}

impl FromStr for DateDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_format(s, vec!['d', 'd', 'd', 'd', '-', 'd', 'd', '-', 'd', 'd'])?;
        let year: i32 = parse_number(&s[0..4], 2000, 3000)?;
        let month: u8 = parse_number(&s[5..7], 1, 12)?;
        let month = time::Month::try_from(month).expect("invalid month number");
        let day: u8 = parse_number(&s[8..10], 1, 31)?;
        let date = time::Date::from_calendar_date(year, month, day)
            .map_err(|err| format!("invalid date: {}", err))?;
        Ok(DateDay(date))
    }
}

/// Time, format HH:MM
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Time(time::Time);

impl Time {
    pub const SIZE: usize = 5;
    #[allow(unused)]
    pub(crate) fn new(hours: u8, minutes: u8) -> Self {
        let time = time::Time::from_hms(hours, minutes, 0).expect("invalid time");
        Time(time)
    }
    pub fn day_start() -> Self {
        Time::new(0, 0)
    }
    pub fn day_end() -> Self {
        Time::new(23, 59)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.0.hour(), self.0.minute()))
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_format(s, vec!['d', 'd', ':', 'd', 'd'])?;
        let hours: u8 = parse_number(&s[0..2], 0, 23)?;
        let minutes: u8 = parse_number(&s[3..5], 0, 59)?;
        let time = time::Time::from_hms(hours, minutes, 0)
            .map_err(|err| format!("Invalid time: {}", err))?;
        Ok(Time(time))
    }
}

/// Duration measured in hours and minutes, format HH:MM
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Duration(time::Duration);

impl Duration {
    pub const fn new(hours: u64, minutes: u64) -> Self {
        let seconds = hours * 60 * 60 + minutes * 60;
        let duration = time::Duration::new(seconds as i64, 0);
        Duration(duration)
    }
    pub fn minutes(&self) -> u64 {
        self.0
            .whole_minutes()
            .try_into()
            .expect("duration minutes are invalid")
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hours = self.0.whole_hours();
        let minutes = self.0.whole_minutes() - hours * 60;
        f.write_fmt(format_args!("{:02}:{:02}", hours, minutes))
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = match s.find(':') {
            Some(sep) => sep,
            _ => return Err("Time duration has to be digits separated by :".to_string()),
        };
        let hours = parse_number(&s[0..sep], 0, 999)?;
        let minutes = parse_number(&s[sep + 1..s.len()], 0, 59)?;
        Ok(Duration(time::Duration::new(
            hours * 60 * 60 + minutes * 60,
            0,
        )))
    }
}

fn parse_number<T: FromStr + Ord + Display>(s: &str, min: T, max: T) -> Result<T, String> {
    let parsed = match s.parse::<T>() {
        Ok(parsed) => parsed,
        _ => return Err(format!("Cannot parse {}", s)),
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
        if expected == 'd' && !c.is_ascii_digit() {
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
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetime_format() {
        let datetime = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49));
        assert_eq!(datetime.to_string(), "2022-11-23 12:49")
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetime_parse() {
        assert_eq!(
            "2022-11-23 12:49".parse::<DateTime>().unwrap(),
            DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49))
        );
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetime_compare() {
        let datetime = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49));
        let datetime_time = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 50));
        assert!(datetime < datetime_time);
        let datetime_date = DateTime::new(DateDay::new(2022, 12, 23), Time::new(12, 49));
        assert!(datetime < datetime_date);
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetime_now() {
        let date = DateTime::now();
        assert_eq!(date.to_string().len(), 16);
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn dateday_format() {
        assert_eq!(DateDay::new(2022, 5, 9).to_string(), "2022-05-09")
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn dateday_day_of_week() {
        let day = DateDay::new(2023, 7, 6);
        assert_eq!(day.days_from_monday(), 3); // Thursday is 3 days from Monday
        assert_eq!(day.remove_days(3).days_from_monday(), 0); // Finding beginning of the week - Monday
        assert_eq!(day.add_days(3).days_from_monday(), 6); // Finding end of the week - Sunday
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn dateday_as_start() {
        let day = DateDay::new(2023, 7, 6);
        assert_eq!(day.as_start_of_week(), DateDay::new(2023, 7, 3));
        assert_eq!(day.as_start_of_month(), DateDay::new(2023, 7, 1));
        assert_eq!(day.as_start_of_year(), DateDay::new(2023, 1, 1));
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn date_parse() {
        assert_eq!(
            "2022-05-09".parse::<DateDay>().unwrap(),
            DateDay::new(2022, 5, 9)
        );
        assert_eq!(
            "2022-01-01".parse::<DateDay>().unwrap(),
            DateDay::new(2022, 1, 1)
        );
        assert_eq!(
            "2022-12-31".parse::<DateDay>().unwrap(),
            DateDay::new(2022, 12, 31)
        );
        assert!("2022-13-31".parse::<DateDay>().is_err());
        assert!("2022-12-32".parse::<DateDay>().is_err());
        assert!("2022-00-09".parse::<DateDay>().is_err());
        assert!("2022-13-09".parse::<DateDay>().is_err());
        assert!("2022-09-32".parse::<DateDay>().is_err());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn date_compare() {
        let date = DateDay::new(2022, 5, 5);
        assert!(date < DateDay::new(2022, 5, 6));
        assert!(date < DateDay::new(2022, 6, 1));
        assert!(date < DateDay::new(2023, 1, 1));
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn time_display() {
        assert_eq!(Time::new(1, 1).to_string(), "01:01");
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn time_parse() {
        assert_eq!("01:01".parse::<Time>().unwrap(), Time::new(1, 1));
        assert_eq!("23:59".parse::<Time>().unwrap(), Time::new(23, 59));
        assert!("24:00".parse::<Time>().is_err());
        assert!("00:60".parse::<Time>().is_err());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn time_compare() {
        let time = Time::new(10, 10);
        assert!(time < Time::new(10, 11));
        assert!(time < Time::new(20, 0));
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetimerange_parse_short() {
        let got = "2022-11-23 12:49 18:32".parse::<DateTimeRange>().unwrap();
        assert_eq!(
            got.start,
            DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49))
        );
        assert_eq!(
            got.end,
            DateTime::new(DateDay::new(2022, 11, 23), Time::new(18, 32))
        );
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetimerange_parse_long() {
        let got = "2022-11-23 12:49 - 2022-11-24 18:32"
            .parse::<DateTimeRange>()
            .unwrap();
        assert_eq!(
            got.start,
            DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49))
        );
        assert_eq!(
            got.end,
            DateTime::new(DateDay::new(2022, 11, 24), Time::new(18, 32))
        );
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetimerange_format_long() {
        // Long notation
        let start = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49));
        let end = DateTime::new(DateDay::new(2022, 11, 24), Time::new(12, 55));
        let range = DateTimeRange::new(start, end).unwrap();
        assert_eq!(range.to_string(), "2022-11-23 12:49 - 2022-11-24 12:55");
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetimerange_format_short() {
        let start = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49));
        let end = start;
        let range = DateTimeRange::new(start, end).unwrap();
        assert_eq!(range.to_string(), "2022-11-23 12:49 12:49");
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn datetimerange_duration() {
        let from = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 49));
        let to = DateTime::new(DateDay::new(2022, 11, 23), Time::new(12, 55));
        let range = DateTimeRange::new(from, to).unwrap();
        assert_eq!(range.duration(), Duration::new(0, 6));
    }
}
