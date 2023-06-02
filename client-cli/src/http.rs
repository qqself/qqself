use std::future::Future;

use qqself_core::api::ApiRequest;
use reqwest::{Error, Response};

pub struct Http {
    client: reqwest::Client,
}

impl Http {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn send(&self, req: ApiRequest) -> impl Future<Output = Result<Response, Error>> {
        self.client
            .post(req.url)
            .body(req.payload)
            .header("Content-Type", req.content_type)
            .send()
    }
}

#[cfg(test)]
mod tests {
    use qqself_core::encryption::keys::Keys;

    use super::*;

    #[tokio::test]
    async fn test_api() {
        let keys = Keys::generate_new();
        let http = Http::new();

        // Set
        let msg = "2022-10-10 00:00 01:00 test app=client_cli source=test_set";
        let req = ApiRequest::new_set_request(&keys, msg.to_string()).unwrap();
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        // Find what we've just added
        let req = ApiRequest::new_find_request(&keys, None).unwrap();
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        assert_eq!(body.lines().count(), 1);

        // Delete it all
        let req = ApiRequest::new_delete_request(&keys).unwrap();
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
    }
}
