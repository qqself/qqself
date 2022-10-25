use qqself_core::encryption::hash::StableHash;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn foo(msg: String) -> String {
    StableHash::hash_string(&msg).to_string()
}
