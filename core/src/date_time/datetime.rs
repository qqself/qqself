use std::{
    fmt::{Display, Formatter},
    ops::Sub,
    str::FromStr,
};

// Date and time, format YYYY-MM-DD HH:MM
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct DateTime(time::PrimitiveDateTime);

impl DateTime {
    // There is no way to get local timezone using `time` on Unix/Mac https://github.com/time-rs/time/issues/325
    #[cfg(feature = "wasm")]
    pub fn now() -> Self {
        let now = time::OffsetDateTime::now_local().unwrap();
        DateTime::new(Date(now.date()), Time(now.time()))
    }
    pub fn new(date: Date, time: Time) -> Self {
        let datetime = time::PrimitiveDateTime::new(date.0, time.0);
        Self(datetime)
    }
    pub fn date(&self) -> Date {
        Date(self.0.date())
    }
    pub fn time(&self) -> Time {
        Time(self.0.time())
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let date = Date(self.0.date());
        let time = Time(self.0.time());
        f.write_fmt(format_args!("{} {}", date, time))
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
// TODO wasm_pack refuses to export it: "cannot shadow already defined class `Date`". Should we rename it? Wrap it?
// #[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Date(time::Date);

impl Date {
    #[allow(unused)]
    pub(crate) fn new(year: i32, month: u8, day: u8) -> Self {
        let month = time::Month::try_from(month).expect("invalid month number");
        let date = time::Date::from_calendar_date(year, month, day).expect("invalid date");
        Date(date)
    }
    pub fn remove_days(&self, days: usize) -> Date {
        let res = self.0.saturating_add(time::Duration::days(-(days as i64)));
        Self(res)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:04}-{:02}-{:02}",
            self.0.year(),
            self.0.month() as u8,
            self.0.day()
        ))
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_format(s, vec!['d', 'd', 'd', 'd', '-', 'd', 'd', '-', 'd', 'd'])?;
        let year: i32 = parse_number(&s[0..4], 2000, 3000)?;
        let month: u8 = parse_number(&s[5..7], 1, 12)?;
        let month = time::Month::try_from(month).expect("invalid month number");
        let day: u8 = parse_number(&s[8..10], 1, 31)?;
        let date = time::Date::from_calendar_date(year, month, day)
            .map_err(|err| format!("invalid date: {}", err))?;
        Ok(Date(date))
    }
}

/// Time, format HH:MM
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Time(time::Time);

impl Time {
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
    fn datetime_format() {
        let datetime = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 49));
        assert_eq!(datetime.to_string(), "2022-11-23 12:49")
    }

    #[test]
    fn datetime_compare() {
        let datetime = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 49));
        let datetime_time = DateTime::new(Date::new(2022, 11, 23), Time::new(12, 50));
        assert!(datetime < datetime_time);
        let datetime_date = DateTime::new(Date::new(2022, 12, 23), Time::new(12, 49));
        assert!(datetime < datetime_date);
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn date_now() {
        let date = DateTime::now();
        assert_eq!(date.to_string().len(), 16);
    }

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
    fn time_display() {
        assert_eq!(Time::new(1, 1).to_string(), "01:01");
    }

    #[test]
    fn time_parse() {
        assert_eq!("01:01".parse::<Time>().unwrap(), Time::new(1, 1));
        assert_eq!("23:59".parse::<Time>().unwrap(), Time::new(23, 59));
        assert!("24:00".parse::<Time>().is_err());
        assert!("00:60".parse::<Time>().is_err());
    }

    #[test]
    fn time_compare() {
        let time = Time::new(10, 10);
        assert!(time < Time::new(10, 11));
        assert!(time < Time::new(20, 0));
    }
}
