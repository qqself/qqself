#![allow(non_snake_case)] // Use camelCase for everything exported as it's convention that TypeScript is using

use qqself_core::{
    api::{ApiRequest, RequestCreateErr},
    binary_text::BinaryToText,
    encryption::keys::{generate_keys, PrivateKey, PublicKey},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
pub struct Keys {
    pub publicKey: String,
    pub privateKey: String,
}

#[wasm_bindgen]
pub fn createNewKeys() -> Keys {
    let (public_key, private_key) = generate_keys();
    Keys {
        publicKey: public_key.to_string(),
        privateKey: private_key.to_string(),
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum ApiErrorType {
    EncodingError,
    EncryptionFailed,
}

#[wasm_bindgen(getter_with_clone)]
pub struct ApiError {
    pub code: ApiErrorType,
    pub msg: String,
}

impl From<RequestCreateErr> for ApiError {
    fn from(err: RequestCreateErr) -> Self {
        match err {
            RequestCreateErr::EncodingError(err) => ApiError {
                code: ApiErrorType::EncodingError,
                msg: err.to_string(),
            },
            RequestCreateErr::EncryptingError(err) => ApiError {
                code: ApiErrorType::EncryptionFailed,
                msg: err.to_string(),
            },
        }
    }
}

fn parse_keys(public: &str, private: &str) -> Result<(PublicKey, PrivateKey), ApiError> {
    let err = |msg: &str| ApiError {
        code: ApiErrorType::EncodingError,
        msg: msg.to_string(),
    };
    let public_key = BinaryToText::new_from_encoded(public.to_string())
        .ok_or_else(|| err("Cannot encode public key"))?;
    let public_key = PublicKey::new(public_key).map_err(|_| err("Cannot create public key"))?;
    let private_key = BinaryToText::new_from_encoded(private.to_string())
        .ok_or_else(|| err("Cannot encode private key"))?;
    let private_key = PrivateKey::new(private_key).map_err(|_| err("Cannot create private key"))?;
    Ok((public_key, private_key))
}

#[wasm_bindgen(getter_with_clone)]
pub struct Request {
    pub url: String,
    pub payload: String,
    pub contentType: String,
}

// TODO Temporary here, after all we should move API IO to core as well
#[wasm_bindgen]
pub fn createApiFindRequest(keys: Keys) -> Result<Request, ApiError> {
    let (public_key, private_key) = parse_keys(&keys.publicKey, &keys.privateKey)?;
    let req = ApiRequest::new_find_request(&public_key, &private_key, None)?;
    Ok(Request {
        url: req.url.to_string(),
        payload: req.payload,
        contentType: req.content_type.to_string(),
    })
}
