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
    data_views::skills::SkillsNotification,
    date_time::{datetime::DateDay, timestamp::Timestamp},
    db::{Notification, Query, Record, ViewUpdate, DB},
    encryption::{
        self,
        hash::StableHash,
        payload::PayloadBytes,
        payload::PayloadId,
        tokens::{DeleteToken, SearchToken},
    },
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::util::error;

mod util;

/// Initialize the library, for now only sets panic hooks and returns build info
#[wasm_bindgen]
pub fn initialize() -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    [
        ("Build", env!("VERGEN_BUILD_TIMESTAMP")),
        ("Commit", env!("VERGEN_GIT_COMMIT_MESSAGE")),
        ("Hash", env!("VERGEN_GIT_SHA")),
        ("Host", env!("VERGEN_RUSTC_HOST_TRIPLE")),
        ("Profile", env!("VERGEN_CARGO_OPT_LEVEL")),
        ("Rust", env!("VERGEN_RUSTC_SEMVER")),
        ("Target", env!("VERGEN_CARGO_TARGET_TRIPLE")),
    ]
    .map(|(k, v)| format!("{k}: {v}"))
    .join("\n")
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

#[wasm_bindgen]
pub struct Views {
    #[allow(unused)]
    keys: Keys,
    db: RefCell<DB>,
}

#[wasm_bindgen]
pub struct UiRecord {
    record: Record,
}

#[wasm_bindgen]
impl UiRecord {
    pub fn to_string(&self, include_date: bool, include_entry_tag: bool) -> String {
        self.record.to_string(include_date, include_entry_tag)
    }

    pub fn created_deleted_record(&self) -> UiRecord {
        let input = self.record.to_deleted_string();
        UiRecord::parse(input, None).expect("deleted string should always be parsable")
    }

    pub fn day(&self) -> String {
        self.record.date_range().start().date().to_string()
    }

    pub fn revision(&self) -> usize {
        self.record.revision()
    }

    pub fn parse(input: String, override_revision: Option<usize>) -> Result<UiRecord, String> {
        let record = Record::parse(&input)?;
        let record = match override_revision {
            Some(revision) => record.with_updated_revision(revision),
            None => record,
        };
        Ok(UiRecord { record })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct SkillData {
    pub title: String,
    pub kind: String,
    pub level: usize,
}

#[wasm_bindgen(getter_with_clone)]
pub struct SkillWeek {
    pub name: String,
    pub progress: usize,
    pub target: usize,
}

#[wasm_bindgen]
impl Views {
    pub fn new(keys: &Keys, onUpdate: js_sys::Function, onNotification: js_sys::Function) -> Self {
        let mut db = DB::default();
        db.on_view_update(Box::new(move |update| {
            let data = js_sys::Map::new();
            match update {
                ViewUpdate::QueryResults => {
                    data.set(&"view".into(), &"QueryResults".into());
                }
                ViewUpdate::Skills(update) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &update.skill.into());
                }
                ViewUpdate::Week => {
                    data.set(&"view".into(), &"Week".into());
                }
            };
            if let Err(err) = onUpdate.call1(&JsValue::NULL, &data) {
                error(&err);
            }
        }));
        db.on_notification(Box::new(move |notification| {
            let data = js_sys::Map::new();
            match notification {
                Notification::Skills(SkillsNotification::HourProgress(msg)) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &msg.into())
                }
                Notification::Skills(SkillsNotification::LevelUp(msg)) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &msg.into())
                }
            };
            if let Err(err) = onNotification.call1(&JsValue::NULL, &data) {
                error(&err);
            }
        }));
        Self {
            keys: keys.clone(),
            db: RefCell::new(db),
        }
    }

    pub fn add_record(&self, record: &UiRecord, interactive: bool, now: Option<DateDay>) {
        let mut db = self.db.borrow_mut();
        db.add(record.record.clone(), interactive, now);
    }

    pub fn update_query(&self, query: String) -> Result<(), String> {
        let query = Query::new(&query).map_err(|v| v.to_string())?;
        let mut db = self.db.borrow_mut();
        db.update_query(query);
        Ok(())
    }

    pub fn query_results(&self) -> Vec<UiRecord> {
        let mut records = Vec::new();
        for record in self.db.borrow().query_results().iter() {
            let record = UiRecord {
                record: record.clone(),
            };
            records.push(record);
        }
        records
    }

    pub fn entry_count(&self) -> usize {
        self.db.borrow().count()
    }

    pub fn view_skills(&self) -> Vec<SkillData> {
        let db = self.db.borrow();
        let mut skills = db.skills().iter().map(|(_, v)| v).collect::<Vec<_>>();
        skills.sort();

        let mut output = Vec::new();
        for skill in skills {
            let skill_data = SkillData {
                title: skill.title().to_string(),
                kind: skill.kind().to_string(),
                level: skill.progress().level,
            };
            output.push(skill_data);
        }
        output
    }

    pub fn view_week(&self) -> Vec<SkillWeek> {
        let db = self.db.borrow();
        let mut output = Vec::new();
        for data in db.week().values() {
            let data = SkillWeek {
                name: data.skill().to_string(),
                progress: data.progress() as usize,
                target: data.target() as usize,
            };
            output.push(data);
        }
        output
    }
}

#[wasm_bindgen]
pub fn validateQuery(query: String) -> Option<String> {
    Query::new(&query).map_err(|v| v.to_string()).err()
}

#[wasm_bindgen]
pub fn stringHash(input: String) -> String {
    StableHash::hash_string(&input).to_string()
}
