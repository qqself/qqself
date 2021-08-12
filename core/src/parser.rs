pub struct Entry {
    original: String,
}

#[derive(Debug)]
pub enum ParseError {
    NoTags,
}

impl Entry {
    pub fn from_string(s: &str) -> Result<Entry, ParseError> {
        if !s.contains('#') {
            return Err(ParseError::NoTags);
        }
        Ok(Entry {
            original: s.to_string(),
        })
    }

    pub fn to_string(&self, time: String) -> String {
        format!("{} {}", time, self.original)
    }
}
