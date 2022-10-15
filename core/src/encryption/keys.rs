use std::fmt::Display;

use crate::binary_text::BinaryToText;

use super::rsa::RSA;

#[derive(Debug)]
pub enum InputError {
    TooBig(String),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key<const S: usize>(BinaryToText);

// Encoded RSA public key data
pub type PublicKey = Key<1024>;

// Encoded RSA private key data
pub type PrivateKey = Key<4096>;

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

pub fn generate_keys() -> (PublicKey, PrivateKey) {
    RSA::generate_keys()
}
