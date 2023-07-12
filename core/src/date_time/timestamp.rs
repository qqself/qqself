use std::{fmt::Display, ops::Sub};

use super::datetime::Duration;

/// Milliseconds precision timestamp that supports sorting in lexicographic order when converted to string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp(u64);

impl Timestamp {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> Self {
        Timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .expect("time cannot be before the UNIX_EPOCH")
                .as_millis()
                .try_into()
                .expect("Timestamp should fit into u64"), // Number of milliseconds between year 1970 and 3000 should fit into 46 bits
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> Self {
        let now = js_sys::Date::now();
        Timestamp(now as u64)
    }

    // Returns number of milliseconds elapsed between timestamp value and now
    pub fn elapsed(&self) -> u64 {
        let now = Timestamp::now();
        now.0 - self.0
    }

    pub fn from_u64(milliseconds: u64) -> Self {
        Self(milliseconds)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn from_string(s: &str) -> Option<Self> {
        s.parse::<u64>().map(Timestamp).ok()
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:0>20}", &self.0.to_string()))
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, other: Duration) -> Self::Output {
        // To avoid panic we can fallback to Timestamp::default() in case of overflow
        // but not sure if hiding this error is worth it
        Self(self.as_u64() - other.minutes() * 60 * 60)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[test]
    #[wasm_bindgen_test]
    fn timestamp_string_lexicographic_order() {
        let max = Timestamp::from_string(&u64::MAX.to_string())
            .unwrap()
            .to_string();
        assert_eq!(max, "18446744073709551615");
        let min = Timestamp::default().to_string();
        assert_eq!(min, "00000000000000000000");
        let value = Timestamp::now().to_string();
        assert_eq!(value.len(), max.len());
        assert!(value < max);
        assert!(value > min);
    }

    #[test]
    fn timestamp_to_from_string() {
        let v = Timestamp::now();
        let s = v.to_string();
        let parsed = Timestamp::from_string(&s).unwrap();
        assert_eq!(v, parsed);
    }

    #[test]
    fn timestamp_sub() {
        let duration = Duration::new(1, 1);
        let milliseconds = duration.minutes() * 60 * 60;
        let got = Timestamp::from_u64(milliseconds) - duration;
        assert_eq!(got.as_u64(), 0);

        let timestamp = Timestamp::from_u64(40_000_000);
        let duration = Duration::new(1, 1);
        assert_eq!(duration.minutes() * 60 * 60, 219_600);
        assert_eq!((timestamp - duration).as_u64(), 39_780_400);
    }

    #[test]
    #[cfg(feature = "cargo")]
    fn timestamp_serde() {
        // Just check that Timestamp can be serialized
        #[derive(serde::Serialize)]
        struct Foo {
            t: Timestamp,
        }
    }
}
