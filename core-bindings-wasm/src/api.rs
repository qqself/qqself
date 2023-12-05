use qqself_core::api::{ApiRequests, Request};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Api(ApiRequests);

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub fn new(base_path: Option<String>) -> Self {
        Self(ApiRequests::new(base_path))
    }

    pub fn create_set_request(&self, payload: String) -> Request {
        self.0.create_set_request(payload)
    }

    pub fn create_find_request(&self, payload: String) -> Request {
        self.0.create_find_request(payload)
    }

    pub fn create_delete_request(&self, payload: String) -> Request {
        self.0.create_delete_request(payload)
    }
}
