#[derive(Debug)]
pub struct ApiRequests {
    base_path: String,
}

impl ApiRequests {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    /// Create new Find request for sync API given already encrypted payload
    pub fn create_find_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/find", self.base_path),
            payload,
            content_type: "text/plain",
        }
    }

    /// Creates API set request, accepts already encrypted payload
    pub fn create_set_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/set", self.base_path),
            payload,
            content_type: "text/plain",
        }
    }

    /// Create new Delete request for sync API
    pub fn create_delete_request(&self, payload: String) -> Request {
        Request {
            url: format!("{}/delete", self.base_path),
            payload,
            content_type: "text/plain",
        }
    }
}

impl Default for ApiRequests {
    fn default() -> Self {
        ApiRequests::new("https://api.qqself.com".to_string())
    }
}

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub payload: String,
    pub content_type: &'static str,
}
