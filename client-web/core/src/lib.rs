#![allow(non_snake_case)] // Use camelCase for everything exported as it's convention that TypeScript is using

use qqself_core::encryption::keys::generate_keys;
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
