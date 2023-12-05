#[derive(Debug)]
pub struct ApiRequests {
    base_path: String,
}

impl ApiRequests {
    pub fn new(base_path: Option<String>) -> Self {
        Self {
            base_path: base_path.unwrap_or("https://api.qqself.com".to_string()),
        }
    }

    /// Create new Find request for sync API given already encrypted payload
    pub fn create_find_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/find", self.base_path),
            payload,
        }
    }

    /// Creates API set request, accepts already encrypted payload
    pub fn create_set_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/set", self.base_path),
            payload,
        }
    }

    /// Create new Delete request for sync API
    pub fn create_delete_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/delete", self.base_path),
            payload,
        }
    }
}

impl Default for ApiRequests {
    fn default() -> Self {
        ApiRequests::new(None)
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "wasm",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct Request {
    pub url: String,
    pub payload: String,
}
