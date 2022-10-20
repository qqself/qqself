use lazy_static::lazy_static;
use regex::Regex;

use crate::encryption::hash::StableHash;

// Encode bytes as text for storing and transferring bytes where raw bytes cannot be
// used and string representation is needed. Currently base58 is used for encoding
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryToText(String);

impl BinaryToText {
    pub fn new(bytes: &[u8]) -> Self {
        Self(bs58::encode(bytes).into_string())
    }

    pub fn new_from_encoded(encoded: String) -> Option<Self> {
        if BinaryToText::is_valid_base58(&encoded) {
            Some(Self(encoded))
        } else {
            None
        }
    }

    pub fn hash_string(&self) -> String {
        StableHash::hash_string(&self.0).to_string()
    }

    pub fn encoded(&self) -> String {
        self.0.clone()
    }

    pub fn decoded(&self) -> Option<Vec<u8>> {
        bs58::decode(&self.0).into_vec().ok()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Check that string contains only valid base58 characters, although string may be still not a valid base58
    // TODO: Actually it may be always valid as long as we check for valid characters, probably remove Option from decoded?
    fn is_valid_base58(s: &str) -> bool {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new("^[1-9A-HJ-NP-Za-km-z]*$").unwrap();
        }
        PATTERN.is_match(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_text_from_bytes() {
        let binary_text = BinaryToText::new(&[10, 20]);
        assert_eq!(binary_text.encoded(), "mV");
        assert_eq!(binary_text.hash_string(), "RBdiDbEHSoTSUPF1FdB7X3");
        assert_eq!(vec![10, 20], binary_text.decoded().unwrap());
    }

    #[test]
    fn binary_text_from_encoded() {
        let encoded = BinaryToText::new(&[10, 20]).encoded();
        let binary_text = BinaryToText::new_from_encoded(encoded).unwrap();
        assert_eq!(vec![10, 20], binary_text.decoded().unwrap());
    }

    #[test]
    fn binary_text_errors() {
        let bad_characters = r#"C:\\Windows\cmd.exe"#;
        assert!(BinaryToText::new_from_encoded(bad_characters.to_string()).is_none());
    }
}
