use std::future::Future;

use qqself_core::api::Request;
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

    pub fn send(&self, req: Request) -> impl Future<Output = Result<Response, Error>> {
        self.client.post(req.url).body(req.payload).send()
    }
}

#[cfg(test)]
mod tests {
    use qqself_core::{api::ApiRequests, encryption::cryptor::Cryptor};

    use super::*;

    #[tokio::test]
    async fn test_api() {
        let cryptor = Cryptor::generate_new();
        let http = Http::new();
        let api = ApiRequests::default();

        // Find by default returns nothing
        let req = api.create_find_request(cryptor.sign_find_token(None).unwrap());
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        assert_eq!(body.lines().count(), 0);

        // Set
        let msg = "2022-10-10 00:00 01:00 test app=client_cli source=test_set";
        let req = api.create_set_request(cryptor.encrypt(msg).unwrap());
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        // Find what we've just added
        let req = api.create_find_request(cryptor.sign_find_token(None).unwrap());
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().await.unwrap();
        assert_eq!(body.lines().count(), 1);

        // Delete it all
        let req = api.create_delete_request(cryptor.sign_delete_token().unwrap());
        let resp = http.send(req).await.unwrap();
        assert_eq!(resp.status(), 200);
    }
}
