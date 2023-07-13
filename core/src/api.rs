use crate::{
    date_time::timestamp::Timestamp,
    encryption::{
        keys::Keys,
        payload::{PayloadBytes, PayloadError, PayloadId},
        tokens::{DeleteToken, SearchToken, TokenErr},
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestCreateErr {
    #[error("encoding error during request creation {0}")]
    EncodingError(#[from] TokenErr),
    #[error("encrypting error during request creation {0}")]
    EncryptingError(#[from] PayloadError),
}

/// Helper struct for creating new requests for sync API. It only creates requests, no IO performed
#[derive(Debug)]
pub struct ApiRequest {
    pub url: &'static str,
    pub payload: String,
    pub content_type: &'static str,
}

// TODO Should we use keys here at all and provide options with embedded encryption?
impl ApiRequest {
    /// Encrypt payload and create new Find request for sync API
    pub fn new_find_request(
        keys: &Keys,
        min_payload_id: Option<PayloadId>,
    ) -> Result<Self, RequestCreateErr> {
        let payload = SearchToken::encode(
            &keys.public_key,
            &keys.private_key,
            Timestamp::now(),
            min_payload_id,
        )?;
        Ok(ApiRequest::new_find_request_encrypted(payload))
    }

    /// Create new Find request for sync API given already encrypted payload
    pub fn new_find_request_encrypted(payload: String) -> Self {
        Self {
            url: "https://api.qqself.com/find",
            payload,
            content_type: "text/plain",
        }
    }

    /// Encrypt payload and create new Set request for sync API
    pub fn new_set_request(keys: &Keys, plaintext: String) -> Result<Self, RequestCreateErr> {
        let payload = PayloadBytes::encrypt(
            &keys.public_key,
            &keys.private_key,
            Timestamp::now(),
            &plaintext,
            None,
        )?;
        Ok(ApiRequest::new_set_request_encrypted(payload.data()))
    }

    /// Create new Set request for sync API
    pub fn new_set_request_encrypted(payload: String) -> Self {
        Self {
            url: "https://api.qqself.com/set",
            payload,
            content_type: "text/plain",
        }
    }

    /// Encrypt the payload and create new Delete request for sync API
    pub fn new_delete_request(keys: &Keys) -> Result<Self, RequestCreateErr> {
        let payload = DeleteToken::encode(&keys.public_key, &keys.private_key, Timestamp::now())?;
        Ok(ApiRequest::new_delete_request_encrypted(payload))
    }

    /// Create new Delete request for sync API
    pub fn new_delete_request_encrypted(payload: String) -> Self {
        Self {
            url: "https://api.qqself.com/delete",
            payload,
            content_type: "text/plain",
        }
    }
}
