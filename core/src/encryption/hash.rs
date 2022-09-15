use std::fmt::Display;

use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

use crate::binary_text::BinaryToText;

// Hash of fixed 16 bytes size. We use that instead of a standard one to ensure
// values are the same across different platforms and stable regardless of Rust version
// BLAKE2 currently used as a hasher
#[derive(Debug, PartialEq, Clone)]
pub struct StableHash([u8; StableHash::SIZE]);

impl StableHash {
    pub const SIZE: usize = 16;

    pub fn hash_string(s: &str) -> Self {
        StableHash::hash_bytes(s.as_bytes())
    }

    pub fn hash_bytes(data: &[u8]) -> Self {
        let mut hasher = Blake2bVar::new(StableHash::SIZE).unwrap();
        hasher.update(data);
        let mut buf = [0u8; StableHash::SIZE];
        hasher.finalize_variable(&mut buf).unwrap();
        StableHash(buf)
    }

    pub fn new_from_bytes(data: [u8; StableHash::SIZE]) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> [u8; StableHash::SIZE] {
        self.0
    }
}

impl Display for StableHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BinaryToText::new(&self.0).encoded())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        // Hash works
        let want = "WC8iEKfwZ7HKMn1APAEy1";
        let got = StableHash::hash_string("foo");
        assert_eq!(want, got.to_string());

        // Hash changes for new values
        let want = "CyU5c3zkD7Na8EkQktwRhT";
        let got = StableHash::hash_string("foo1");
        assert_eq!(want, got.to_string());

        // Hash is stable
        let got = StableHash::hash_string("foo1");
        assert_eq!(want, got.to_string());

        // Hash bytes
        let got = StableHash::hash_bytes(&vec![10, 20]);
        assert_eq!(got.to_string(), "Nr2ASJ1iffPXuHonq1Hddk");

        // Dummy test to ensure capacity doesn't effect the hash
        let mut data = Vec::with_capacity(10);
        data.push(10);
        data.push(20);
        let got = StableHash::hash_bytes(&data);
        assert_eq!(got.to_string(), "Nr2ASJ1iffPXuHonq1Hddk")
    }
}
