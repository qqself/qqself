use lazy_static::lazy_static;

#[derive(Debug)]
pub struct ApiRequests {
    base_path: String,
}

lazy_static! {
    static ref DEFAULT_HEADERS: Vec<Header> = {
        vec![
            Header {
                name: "Content-Type".to_string(),
                value: "text/plain".to_string(),
            },
            Header {
                name: "X-Client-Version".to_string(),
                value: "0".to_string(),
            },
        ]
    };
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
            headers: DEFAULT_HEADERS.clone(),
        }
    }

    /// Creates API set request, accepts already encrypted payload
    pub fn create_set_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/set", self.base_path),
            payload,
            headers: DEFAULT_HEADERS.clone(),
        }
    }

    /// Create new Delete request for sync API
    pub fn create_delete_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/delete", self.base_path),
            payload,
            headers: DEFAULT_HEADERS.clone(),
        }
    }
}

impl Default for ApiRequests {
    fn default() -> Self {
        ApiRequests::new(None)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "wasm",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct Header {
    // Simple static str would be a nicer here, but it has bad support in bindings generation
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "wasm",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct Request {
    pub url: String,
    pub payload: String,
    pub headers: Vec<Header>,
}

// TODO We should move ServerErrors from api-entries/services in here as right now every client has their own implementation of it
