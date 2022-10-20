use std::future::Future;

use qqself_core::encryption::payload::PayloadBytes;
use reqwest::{Error, Response};

pub struct Api {
    client: reqwest::Client,
}

impl Api {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    // TODO We should support text/plain as a content-type in API to avoid extra JSON serialization
    // TODO We should support streaming JSON lines not only in getting data, but also for sending
    pub fn set(&self, payload: PayloadBytes) -> impl Future<Output = Result<Response, Error>> {
        self.client
            .post("https://api.qqself.com/set")
            .body(format!("{{\"payload\":\"{}\"}}", payload.data()))
            .header("content-type", "application/json")
            .send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qqself_core::{datetime::Timestamp, encryption::keys::generate_keys};

    #[tokio::test]
    async fn test_set() {
        let (public_key, private_key) = generate_keys();
        let msg = "2022-10-10 00:00 01:00 test app=client_cli source=test_set";
        let payload =
            PayloadBytes::encrypt(&public_key, &private_key, Timestamp::now(), msg, None).unwrap();
        let api = Api::new();
        let resp = api.set(payload).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        assert_eq!(body, "");
    }
}
