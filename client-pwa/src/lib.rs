extern crate core;
extern crate wasm_bindgen;

mod utils;

use core::parser::{Entry, ParseError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse(s: String) -> String {
    return match Entry::from_string(&s) {
        Ok(entry) => entry.to_string(),
        Err(err) => err.to_string(),
    };
}
