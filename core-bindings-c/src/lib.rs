use std::panic;
use std::sync::Arc;

use qqself_core::encryption::cryptor::CryptorError;
use qqself_core::encryption::hash::StableHash;

uniffi::include_scaffolding!("qqself");

pub use qqself_core::api::{ApiRequests as Api, Header, Request};
pub use qqself_core::build_info;
pub use qqself_core::encryption::cryptor::Cryptor;

pub mod panic_hook;
pub use panic_hook::{set_panic_hook, PanicHook};

pub fn string_hash(input: String) -> String {
    StableHash::hash_string(&input).to_string()
}

// uniffi doesn't support static functions, wrap those manually

pub fn cryptor_generate_new() -> Arc<Cryptor> {
    Arc::new(Cryptor::generate_new())
}

pub fn cryptor_from_deserialized_keys(data: String) -> Result<Arc<Cryptor>, CryptorError> {
    let cryptor = Cryptor::from_deserialized_keys(data)?;
    Ok(Arc::new(cryptor))
}
