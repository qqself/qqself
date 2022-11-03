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
    use super::*;
    use qqself_core::encryption::keys::generate_keys;

    #[tokio::test]
    async fn test_api() {
        let (public_key, private_key) = generate_keys();
        let msg = "2022-10-10 00:00 01:00 test app=client_cli source=test_set";
        let req = ApiRequest::new_set_request(&public_key, &private_key, msg.to_string()).unwrap();
        let payload = req.payload.clone();
        let http = Http::new();

        // Set
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        assert_eq!(body, "");

        // Find what we've just added
        let req = ApiRequest::new_find_request(&public_key, &private_key, None).unwrap();
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        let lines: Vec<_> = body.lines().collect();
        assert_eq!(lines.len(), 1);
        assert_eq!(payload, lines[0]);
    }
}
