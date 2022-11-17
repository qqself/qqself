use std::fmt::Display;

use crate::binary_text::BinaryToText;

use super::rsa::Rsa;

#[derive(Debug)]
pub enum InputError {
    TooBig(String),
}

// Internal representation of both public and private key. Both public and private keys
// has it's own newtype to ensure those will be never confused between each other
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key<const S: usize>(BinaryToText);

impl<const S: usize> Key<S> {
    pub fn new(data: BinaryToText) -> Result<Self, InputError> {
        if data.len() > S {
            return Err(InputError::TooBig(format!(
                "Max length of the key is {}, got {}",
                S,
                data.len()
            )));
        }
        Ok(Self(data))
    }

    pub fn hash_string(&self) -> String {
        self.0.hash_string()
    }

    pub fn decoded(&self) -> Option<String> {
        let data = self.0.decoded().take()?;
        String::from_utf8(data).ok()
    }
}

impl<const S: usize> Display for Key<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.encoded())
    }
}

// Encoded RSA public key data
pub type PublicKey = Key<1024>;

// Encoded RSA private key data
pub type PrivateKey = Key<4096>;

/// Contains both public and private keys. We have custom [de]/serialization logic as key files are
/// rather simple structures and reading/writing it should work cross-platform in the reliable
/// way and preferably without any dependencies
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Keys {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl Keys {
    /// Generate new keys. It's CPU intensive operation and it's advised to run it outside of event loop if used
    pub fn generate_new() -> Self {
        let (public_key, private_key) = Rsa::generate_keys();
        Self {
            public_key,
            private_key,
        }
    }

    /// Serialize keys to string. Encoded public and private keys, separated by '\n'
    pub fn serialize(&self) -> String {
        let mut keys = self.public_key.to_string();
        keys.push('\n');
        keys.push_str(&self.private_key.to_string());
        keys
    }

    /// Deserialize string to keys. Returns `None` if data cannot be parsed
    // TODO Probably Option should be replaced with Result for better error reporting
    pub fn deserialize(data: String) -> Option<Keys> {
        let (line1, line2) = data.split_once('\n')?;
        let public_key = BinaryToText::new_from_encoded(line1.to_string())?;
        let public_key = PublicKey::new(public_key).ok()?;
        let private_key = BinaryToText::new_from_encoded(line2.to_string())?;
        let private_key = PrivateKey::new(private_key).ok()?;
        Some(Self {
            public_key,
            private_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let keys = Keys::generate_new();
        let serialized = keys.serialize();
        let deserialized = Keys::deserialize(serialized).unwrap();
        assert_eq!(keys, deserialized);
    }
}
