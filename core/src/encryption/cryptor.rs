use crate::{binary_text::BinaryToText, date_time::timestamp::Timestamp};

use super::{
    keys::Keys,
    payload::{PayloadBytes, PayloadId},
    tokens::{DeleteToken, SearchToken},
};

#[derive(Debug, thiserror::Error)]
pub enum CryptorError {
    #[error("CryptorError: {err}")]
    Error { err: String },
}

impl From<String> for CryptorError {
    fn from(err: String) -> Self {
        CryptorError::Error { err }
    }
}

/// Handles everything related to encryption, decryption, signing and keys generation
/// Every operation related to encryption is CPU heavy operation and it's better to run
/// those outside of the main event loop if exists
#[derive(Debug, Clone)]
pub struct Cryptor(Keys);

impl Cryptor {
    /// Creates a new Cryptor with new generated pair of keys
    pub fn generate_new() -> Self {
        Self(Keys::generate_new())
    }

    /// Creates Cryptor by deserializing `Keys` from the given string
    pub fn from_deserialized_keys(data: String) -> Result<Self, CryptorError> {
        match Keys::deserialize(data) {
            Some(keys) => Ok(Self(keys)),
            None => Err(CryptorError::Error {
                err: "Failed to deserialize the key file".to_string(),
            }),
        }
    }

    /// Serializes Cryptor's `Keys` to string
    pub fn serialize_keys(&self) -> String {
        self.0.serialize()
    }

    /// Decrypt the payload
    pub fn decrypt(&self, data: String) -> Result<String, CryptorError> {
        let binary =
            BinaryToText::new_from_encoded(data).ok_or_else(|| "Bad data encoding".to_string())?;
        let payload = PayloadBytes::new_from_encrypted(binary).map_err(|v| v.to_string())?;
        let payload = payload.validated(None).map_err(|v| v.to_string())?;
        let decrypted = payload
            .decrypt(&self.0.private_key)
            .map_err(|v| v.to_string())?;
        Ok(decrypted)
    }

    /// Encrypt the plaintext
    pub fn encrypt(&self, plaintext: &str) -> Result<String, CryptorError> {
        let payload = PayloadBytes::encrypt(
            &self.0.public_key,
            &self.0.private_key,
            Timestamp::now(),
            plaintext,
        )
        .map_err(|err| err.to_string())?;
        Ok(payload.data())
    }

    /// Creates and signs new `DeleteToken`
    pub fn sign_delete_token(&self) -> Result<String, CryptorError> {
        DeleteToken::encode(&self.0.public_key, &self.0.private_key, Timestamp::now()).map_err(
            |err| CryptorError::Error {
                err: err.to_string(),
            },
        )
    }

    /// Creates and signs new `SearchToken`
    pub fn sign_find_token(&self, last_id: Option<String>) -> Result<String, CryptorError> {
        let min_payload_id = last_id.map(PayloadId::new_encoded);
        SearchToken::encode(
            &self.0.public_key,
            &self.0.private_key,
            Timestamp::now(),
            min_payload_id,
        )
        .map_err(|err| CryptorError::Error {
            err: err.to_string(),
        })
    }

    /// Returns hash string for the Cryptor's public key
    pub fn public_key_hash(&self) -> String {
        self.0.public_key.hash_string()
    }
}
