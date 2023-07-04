#![allow(non_snake_case)] // Use camelCase for everything exported as it's convention that TypeScript is using

/*

Few hard learned rules to follow when writing bridge functions:

- panics/unreachable/todo should not be used as it breaks WebAssembly context and bridge stops. Return Result<T, String> instead
- Never pass structs by value as then WebAssembly will nullify this object on JS side
- Use crate::util::log for debugging
- Careful with recursion - if Rust calls passed JS function (e.g. callback) which in turn calls Rust again it may create a
  situation where struct is borrowed as `&mut self` and `&self` which causes a crash with cryptic error message and bad stacktrace.
  To break recursion use `setTimeout(logic, 0)` on JS side
  To make things worse it's not crashing in NodeJS. Stable and simple way is to use interior mutability and avoid `&mut self` in the bridge

*/

use std::{cell::RefCell, panic};

use qqself_core::{
    api::{ApiRequest, RequestCreateErr},
    binary_text::BinaryToText,
    data_views::{
        journal::{JournalDay, JournalUpdate},
        skills::SkillsUpdate,
    },
    date_time::{datetime::DateDay, timestamp::Timestamp},
    db::{Record, ViewUpdate, DB},
    encryption::{
        self,
        hash::StableHash,
        payload::PayloadBytes,
        payload::PayloadId,
        tokens::{DeleteToken, SearchToken},
    },
    record::Entry,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::util::error;

mod util;

/// Initialize the library, for now only sets panic hooks and returns build info
#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

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
        Ok(decrypted)
    }

    pub fn encrypt(&self, plaintext: String) -> Result<String, String> {
        let payload = PayloadBytes::encrypt(
            &self.0.public_key,
            &self.0.private_key,
            Timestamp::now(),
            &plaintext,
            None,
        )
        .map_err(|err| err.to_string())?;
        Ok(payload.data())
    }

    pub fn sign_delete_token(&self) -> Result<String, String> {
        DeleteToken::encode(&self.0.public_key, &self.0.private_key, Timestamp::now())
            .map_err(|err| err.to_string())
    }

    pub fn sign_find_token(&self, lastId: Option<String>) -> Result<String, String> {
        let min_payload_id = lastId.map(PayloadId::new_encoded);
        SearchToken::encode(
            &self.0.public_key,
            &self.0.private_key,
            Timestamp::now(),
            min_payload_id,
        )
        .map_err(|err| err.to_string())
    }

    pub fn public_key_hash(&self) -> String {
        self.0.public_key.hash_string()
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
// TODO Whole API needs rethinking: We have one in Core, one in PWA, one in bridge,
//      some handles encryption, some handles already encrypted payload
// TODO Also error handling is OK, but we need to distinguish between re-triable/invisible and
//      something that user should see
#[wasm_bindgen]
pub struct API {}

#[wasm_bindgen]
impl API {
    pub fn createApiFindRequest(encryptedPayload: &str) -> Result<Request, ApiError> {
        let req = ApiRequest::new_find_request_encrypted(encryptedPayload.to_string());
        Ok(Request {
            url: req.url.to_string(),
            payload: req.payload,
            contentType: req.content_type.to_string(),
        })
    }

    /// Creates API set request, accepts already encrypted payload
    pub fn createApiSetRequest(encryptedPayload: &str) -> Result<Request, ApiError> {
        let req = ApiRequest::new_set_request_encrypted(encryptedPayload.to_string());
        Ok(Request {
            url: req.url.to_string(),
            payload: req.payload,
            contentType: req.content_type.to_string(),
        })
    }
    pub fn createApiDeleteRequest(keys: &Keys) -> Result<Request, ApiError> {
        let req = ApiRequest::new_delete_request(&keys.0)?;
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
pub struct Views {
    #[allow(unused)]
    keys: Keys,
    db: RefCell<DB>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct SkillsView {
    pub skills: String,
}

#[wasm_bindgen]
impl Views {
    pub fn new(keys: &Keys, onUpdate: js_sys::Function) -> Self {
        let mut db = DB::default();
        db.subscribe_view_updates(Box::new(move |update| {
            let obj = js_sys::Map::new();
            match update {
                ViewUpdate::Journal(JournalUpdate::DayUpdated(update)) => {
                    obj.set(&"view".into(), &"Journal".into());
                    obj.set(&"type".into(), &"DayUpdated".into());
                    obj.set(&"day".into(), &update.to_string().into());
                }
                ViewUpdate::Skills(SkillsUpdate::HourProgress(update)) => {
                    obj.set(&"view".into(), &"Skills".into());
                    obj.set(&"type".into(), &"HourProgress".into());
                    obj.set(&"message".into(), &update.into());
                }
                ViewUpdate::Skills(SkillsUpdate::LevelUp(update)) => {
                    obj.set(&"view".into(), &"Skills".into());
                    obj.set(&"type".into(), &"LevelUp".into());
                    obj.set(&"message".into(), &update.into());
                }
            };
            if let Err(err) = onUpdate.call1(&JsValue::NULL, &obj) {
                error(&err);
            }
        }));
        Self {
            keys: keys.clone(),
            db: RefCell::new(db),
        }
    }

    pub fn add_entry(&self, input: String) -> Result<(), String> {
        let entry = Entry::parse(&input).map_err(|err| err.to_string())?;
        let record = Record::from_entry(entry, 1);
        let mut db = self.db.borrow_mut();
        db.add(record);
        Ok(())
    }

    pub fn journal_day(&self, day: &DateDay) -> AppJournalDay {
        let journal_day = self
            .db
            .borrow()
            .journal()
            .get(day)
            .cloned()
            .unwrap_or_else(|| JournalDay::new(*day));
        let mut entries = String::new();
        for entry in &journal_day.entries {
            entries.push_str(&format!("{}\n", entry.to_string_short()));
        }
        AppJournalDay { day: *day, entries }
    }

    pub fn entry_count(&self) -> usize {
        self.db.borrow().count()
    }

    pub fn view_skills(&self) -> SkillsView {
        let skills = self
            .db
            .borrow()
            .skills()
            .iter()
            .map(|v| format!("{} {} {}", v.kind(), v.title(), v.progress().level))
            .fold(String::new(), |a, b| a + &b + "\n");
        SkillsView { skills }
    }
}

#[wasm_bindgen]
pub fn validateEntry(input: String) -> Option<String> {
    Entry::parse(&input).map_err(|e| e.to_string()).err()
}

#[wasm_bindgen]
pub fn stringHash(input: String) -> String {
    StableHash::hash_string(&input).to_string()
}
