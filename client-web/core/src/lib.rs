#![allow(non_snake_case)] // Use camelCase for everything exported as it's convention that TypeScript is using

use qqself_core::{
    api::{ApiRequest, RequestCreateErr},
    binary_text::BinaryToText,
    data_views::journal::JournalDay,
    date_time::datetime::DateDay,
    db::{Record, DB},
    encryption::{self, payload::PayloadBytes},
    parsing::parser::Parser,
};
use wasm_bindgen::prelude::wasm_bindgen;
mod util;

#[wasm_bindgen]
#[derive(Clone)] // TODO Keys shouldn't be Clone - it should be generated/got from cache and then moved to the App and never be used
pub struct Keys(encryption::keys::Keys);

#[wasm_bindgen]
impl Keys {
    pub fn createNewKeys() -> Keys {
        Self(encryption::keys::Keys::generate_new())
    }
    pub fn deserialize(data: String) -> Result<Keys, String> {
        match encryption::keys::Keys::deserialize(data) {
            Some(keys) => Ok(Self(keys)),
            None => Err("Failed to deserialize the key file".to_string()),
        }
    }
    pub fn serialize(&self) -> String {
        self.0.serialize()
    }
    pub fn decrypt(&self, data: String) -> Result<String, String> {
        let binary =
            BinaryToText::new_from_encoded(data).ok_or_else(|| "Bad data encoding".to_string())?;
        let payload = PayloadBytes::new_from_encrypted(binary).map_err(|v| v.to_string())?;
        let payload = payload.validated(None).map_err(|v| v.to_string())?;
        let decrypted = payload
            .decrypt(&self.0.private_key)
            .map_err(|v| v.to_string())?;
        Ok(decrypted.text().to_string())
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

#[wasm_bindgen(getter_with_clone)]
pub struct Request {
    pub url: String,
    pub payload: String,
    pub contentType: String,
}

// TODO Temporary here, after all we should move API IO to core as well
#[wasm_bindgen]
pub struct API {}

#[wasm_bindgen]
impl API {
    pub fn createApiFindRequest(keys: &Keys) -> Result<Request, ApiError> {
        let req = ApiRequest::new_find_request(&keys.0, None)?;
        Ok(Request {
            url: req.url.to_string(),
            payload: req.payload,
            contentType: req.content_type.to_string(),
        })
    }
    pub fn createApiSetRequest(keys: &Keys, msg: &str) -> Result<Request, ApiError> {
        let req = ApiRequest::new_set_request(&keys.0, msg.to_string())?;
        Ok(Request {
            url: req.url.to_string(),
            payload: req.payload,
            contentType: req.content_type.to_string(),
        })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct AppJournalDay {
    pub day: DateDay,
    // TODO For now simply join all Entry.to_string() with '\n'
    pub entries: String,
}

#[wasm_bindgen]
pub struct App {
    #[allow(unused)]
    keys: Keys,
    db: DB,
}

#[wasm_bindgen]
impl App {
    pub fn new(keys: &Keys) -> Self {
        Self {
            keys: keys.clone(),
            db: DB::default(),
        }
    }

    pub fn add_entry(&mut self, input: &str) {
        let entry = Parser::new(input)
            .parse_date_record()
            .expect("input should be parsable");
        let record = Record::from_entry(entry, 1);
        self.db.add(record);
    }

    pub fn journal_day(&self, day: DateDay) -> AppJournalDay {
        let journal_day = self
            .db
            .journal()
            .get(&day)
            .cloned()
            .unwrap_or_else(|| JournalDay::new(day));
        let mut entries = String::new();
        for entry in &journal_day.entries {
            entries.push_str(&format!("{}\n", entry.to_string_short()));
        }
        AppJournalDay { day, entries }
    }
}
