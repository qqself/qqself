#![allow(non_snake_case)] // Use camelCase for everything exported as it's convention that TypeScript is using

mod api;
mod cryptor;
mod util;
mod views;

use std::panic;

use qqself_core::{build_info, db::Query, encryption::hash::StableHash};
use wasm_bindgen::prelude::wasm_bindgen;

/// Initialize the library, for now only sets panic hooks and returns build info
#[wasm_bindgen]
pub fn initialize() -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    build_info()
}

#[wasm_bindgen]
pub fn validateQuery(query: String) -> Option<String> {
    Query::new(&query).map_err(|v| v.to_string()).err()
}

#[wasm_bindgen]
pub fn stringHash(input: String) -> String {
    StableHash::hash_string(&input).to_string()
}
