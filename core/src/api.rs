use crate::{
    date_time::timestamp::Timestamp,
    encryption::{
        keys::Keys,
        payload::{PayloadBytes, PayloadError},
        search_token::{SearchToken, SearchTokenErr},
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestCreateErr {
    #[error("encoding error during request creation {0}")]
    EncodingError(#[from] SearchTokenErr),
    #[error("encrypting error during request creation {0}")]
    EncryptingError(#[from] PayloadError),
}

// Helper struct for creating new requests for sync API. It only creates requests, no IO is performed
#[derive(Debug)]
pub struct ApiRequest {
    pub url: &'static str,
    pub payload: String,
    pub content_type: &'static str,
}

impl ApiRequest {
    // Create new Find request for sync API
    pub fn new_find_request(
        keys: &Keys,
        timestamp_search: Option<Timestamp>,
    ) -> Result<Self, RequestCreateErr> {
        let payload = SearchToken::encode(
            &keys.public_key,
            &keys.private_key,
            Timestamp::now(),
            timestamp_search,
        )?;
        Ok(Self {
            url: "https://api.qqself.com/find",
            payload,
            content_type: "text/plain",
        })
    }

    // Create new Set request for sync API
    pub fn new_set_request(keys: &Keys, plaintext: String) -> Result<Self, RequestCreateErr> {
        let payload = PayloadBytes::encrypt(
            &keys.public_key,
            &keys.private_key,
            Timestamp::now(),
            &plaintext,
            None,
        )?;
        Ok(Self {
            url: "https://api.qqself.com/set",
            payload: payload.data(),
            content_type: "text/plain",
        })
    }
}
