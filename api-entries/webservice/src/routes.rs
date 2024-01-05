use std::{fmt::Display, pin::Pin};

use actix_web::{
    http::header::ContentType,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder,
};
use futures::StreamExt;
use log::info;
use qqself_api_entries_services::{
    build_info,
    entry::Entries,
    service_error::{HttpCodeForError, ServiceError, ServiceErrorType},
};

#[derive(Debug)]
struct ResponseError(ServiceErrorType);

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.0.http_status_code())
            .expect("Status code should always be valid")
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let body = ServiceError {
            error_code: self.status_code().as_u16(),
            error: self.0.to_string(),
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(body)
    }
}

async fn not_found() -> Result<String, ResponseError> {
    Err(ResponseError(ServiceErrorType::NotFound))
}

pub fn http_config(entries: Data<Entries>) -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::JsonConfig::default().error_handler(|err, _| {
            ResponseError(ServiceErrorType::BadInput(err.to_string())).into()
        }))
        .app_data(entries)
        .route("/health", web::get().to(health))
        .route("/set", web::post().to(set))
        .route("/find", web::post().to(find))
        .route("/delete", web::post().to(delete))
        .default_service(web::route().to(not_found));
    }
}

async fn health() -> impl Responder {
    build_info()
}

async fn set(req_body: String, entries: Data<Entries>) -> Result<impl Responder, ResponseError> {
    entries
        .save_payload(req_body)
        .await
        .map(|v| v.to_string())
        .map_err(ResponseError)
}

async fn find(req_body: String, entries: Data<Entries>) -> Result<HttpResponse, ResponseError> {
    let mut items = entries.find(req_body).await.map_err(ResponseError)?;

    // HACK For some reason when we are behind AWS ELB and client makes multiple HTTP requests where
    //      one of the responses is streamed (like with /find) then it may be truncated with only headers
    //      transferred. No idea what causes that, but following (accidentally discovered) fix helps.
    //      We are using ELB via AWS AppRunner, so no logs are available, maybe it's just AppRunner thing?
    if (Pin::new(&mut items).peek().await).is_none() {
        info!("No entries found for /find");
        return Ok(HttpResponse::Ok().body(""));
    }
    let items = items.map(|v| {
        v.map(|(payload_id, payload_bytes)| {
            web::Bytes::from(format!("{}:{}\n", payload_id, payload_bytes.data()))
        })
    });
    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(items))
}

async fn delete(req_body: String, entries: Data<Entries>) -> Result<impl Responder, ResponseError> {
    entries
        .delete(req_body)
        .await
        .map(|v| v.to_string())
        .map_err(ResponseError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::header,
        test,
        web::{self, Bytes, Data, ServiceConfig},
        App,
    };
    use qqself_api_entries_services::{
        entry_storage::{EntryStorage, MemoryEntryStorage},
        time::TimeStatic,
    };
    use qqself_core::{
        binary_text::BinaryToText,
        date_time::timestamp::Timestamp,
        encryption::{
            hash::StableHash,
            keys::{PrivateKey, PublicKey},
            payload::{PayloadBytes, PayloadId},
            tokens::{DeleteToken, SearchToken},
        },
    };

    const PUBLIC_KEY_1: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqo47AoLLabVv7Ycj6VoEZj1Gz9YPPDhcUjbkzgzLb5n799MydJYdRLA17wDAuvNTcJ4m27F2jzg7Zv26r94eYbRRrYH6oauQGPr9a6XyvNKTzykLkU9m5C3vEnpTVai2NMdib9JiEeJUMUSaApNd4r3ZF9i46suP7qD9gimj2USuh1QHY3r9YKmcyurkZRGZhjyXAnbae98vuJtUxVyMMzV9QWkV1BodGMFc4gE77HhULKk1Z23igQWJZsDTUDhiZdLxs5pmW1699zEgNt42PtJGxQ4ouL5UZcNv42UpUrrXsnKpAKLkRKZTfpsdp4zmPYfSjMNqPQLqiyDLw1B1b5Vs23pAYNMNJoBJXp3wMsJFngqPtPDWZ9Bgm5361uAZa2yNBBfaJMoumTjAPY54MWzYbeqj7mB7ZvLm1351SVJn8rNqrHAE6fNxbruJVwjzbKzbLmD859ZBd2F1V4SKRQZSAymj9sfJYYCn3Z6KoKzBSgH2QYXoTb93dVGDGqegfwZ9EYq";
    const PRIVATE_KEY_1: &str = "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";
    const PUBLIC_KEY_2: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqEKPRmocEuhWJthsJJ44rdBUxeWKdcUfKFdThfJ5N9ZL41CYjJWQahFqkf6KcZcKMrzLwuvvNL3PQJ7Ly688RrUdYrY8NozzkiK1rUcBZCWAcXvavpDkxh46j7si82S1oZmpkfVT77nkwdHhUYNMmSxwnjQj3iTDqVYJqext2ZM11TMsr5FSfQrr1WUv4ZLyjMkE7uVHsJih3kB4dkfg5LYJfUfnGw9fDfRyCk6YFQ7t2HxTJRxigMxACSJpbCCgza2iA1LdEA8rYCKnLWsCeXMZC4fezu7Vhb2ivuYYRc16vkcJKq4xKxhp6mXWP5jrY9Y2dMWrMrBVeiE8ac1Z6D1EdvanJUzHisyRXg3K2DKmrvzKer7f67pXjHYUnLh9s3owzULxGsCZ88BLmMgrw5JrTvbYoRhACaUksFTUhJxAHJAysVDVHSr4PEhd6oqzPoM4TqFLYmaa5aYGBL8NgqJarvWvM59finsXpVvi2zcZ7hbFGHE2vLjqZpyjeiJNBc6DyEh";
    const PRIVATE_KEY_2: &str = "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";

    fn keys(public: &str, private: &str) -> (PublicKey, PrivateKey) {
        let public_key =
            PublicKey::new(BinaryToText::new_from_encoded(public.to_string()).unwrap()).unwrap();
        let private_key =
            PrivateKey::new(BinaryToText::new_from_encoded(private.to_string()).unwrap()).unwrap();
        (public_key, private_key)
    }

    fn test_app() -> (Data<Entries>, impl FnOnce(&mut ServiceConfig)) {
        let time = Box::new(TimeStatic::new(1662750865));
        let entries = Data::new(Entries::new(Box::new(MemoryEntryStorage::new()), time));
        (entries.clone(), |cfg: &mut web::ServiceConfig| {
            http_config(entries)(cfg)
        })
    }

    fn req_set(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/set")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    fn req_delete(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/delete")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    fn req_find(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/find")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    async fn items_plaintext(
        storage: &dyn EntryStorage,
        public_key: &PublicKey,
        private_key: &PrivateKey,
    ) -> Vec<String> {
        storage
            .find(public_key, None)
            .map(|v| v.unwrap())
            .map(|(_, data)| data.validated(None).unwrap())
            .map(|v| v.decrypt(private_key).unwrap())
            .collect::<Vec<_>>()
            .await
    }

    #[actix_web::test]
    async fn test_health() {
        let (_, configure) = test_app();
        let init_service = test::init_service(App::new().configure(configure)).await;
        let app = init_service;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200)
    }

    #[actix_web::test]
    async fn test_set_bad_json() {
        let (_, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post()
            .uri("/set")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload("{}")
            .to_request();
        let resp: ServiceError = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.error_code, 422);
        assert_eq!(resp.error, "BadInput. Error validating encoded payload");
    }

    #[actix_web::test]
    async fn test_not_found() {
        let (_, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post().uri("/invalid").to_request();
        let resp: ServiceError = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.error_code, 404);
        assert_eq!(resp.error, "Requested endpoint not found")
    }

    #[actix_web::test]
    async fn test_too_old_payload() {
        let (_, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encrypted =
            PayloadBytes::encrypt(&public_key, &private_key, Timestamp::default(), "entry")
                .unwrap();
        let resp: ServiceError =
            test::call_and_read_body_json(&app, req_set(encrypted.data()).to_request()).await;
        assert_eq!(resp.error, "OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp".to_string());
        assert_eq!(resp.error_code, 408);
    }

    #[actix_web::test]
    async fn test_set_bad_input() {
        let (_, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encrypted =
            PayloadBytes::encrypt(&public_key, &private_key, Timestamp::now(), "entry").unwrap();
        let resp: ServiceError =
            test::call_and_read_body_json(&app, req_set(encrypted.data() + "h").to_request()).await;
        assert_eq!(
            resp.error,
            "BadInput. Payload validation failure. Cannot read binary data".to_string()
        );
        assert_eq!(resp.error_code, 422);
    }

    #[actix_web::test]
    async fn test_set_add() {
        let (entries, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        for (ts, expected) in [
            (1, b"00000000001662750866|SWxyLukYqS63bYMvfwoj8f"),
            (3, b"00000000001662750869|R85af9xML6WR7fNUXNgi5V"),
            (2, b"00000000001662750871|93i31rxkhgVVzHahAA2LBF"),
        ] {
            entries
                .time()
                .sleep(std::time::Duration::from_millis(ts))
                .await;
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                entries.time().now().await,
                &ts.to_string(),
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            assert_eq!(test::read_body(resp).await.to_vec(), expected);
        }
        let got = items_plaintext(entries.storage(), &public_key, &private_key).await;
        assert_eq!(got, vec!["1", "3", "2"])
    }

    #[actix_web::test]
    async fn test_set_replace() {
        let (entries, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let mut replace_id = None;
        for ts in [1, 2, 3, 4] {
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                Timestamp::from_u64(Timestamp::now().as_u64() + ts),
                &ts.to_string(),
            )
            .unwrap();
            let payload = encrypted.validated(None).unwrap();
            let resp = test::call_service(&app, req_set(payload.data().data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            if ts == 2 {
                let body = test::read_body(resp).await.to_vec();
                let body = String::from_utf8(body).unwrap();
                let payload_id = PayloadId::new_encoded(body);
                replace_id.replace(payload_id);
            }
        }
        let got = items_plaintext(entries.storage(), &public_key, &private_key).await;
        assert_eq!(got, vec!["1", "2", "3", "4"])
    }

    #[actix_web::test]
    async fn test_find() {
        let (entries, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let time_start = entries.time().now().await;
        for ts in [1, 2, 3] {
            entries
                .time()
                .sleep(std::time::Duration::from_millis(ts))
                .await;
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                Timestamp::from_u64(time_start.as_u64() + ts),
                &ts.to_string(),
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200)
        }
        let extract_plaintext = |data: Bytes| {
            let s = String::from_utf8(data.to_vec()).unwrap();
            let mut output = Vec::new();
            for s in s.lines() {
                let payload_start_pos = s.find(':').unwrap() + 1;
                let binary_text =
                    BinaryToText::new_from_encoded(s[payload_start_pos..].to_string()).unwrap();
                let bytes = PayloadBytes::new_from_encrypted(binary_text).unwrap();
                let valid = bytes.validated(None).unwrap();
                let plaintext = valid.decrypt(&private_key).unwrap();
                output.push(plaintext);
            }
            output
        };
        // Return all
        let body = SearchToken::encode(&public_key, &private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert_eq!(extract_plaintext(resp), vec!["1", "2", "3"]);

        // Return after
        let body = SearchToken::encode(
            &public_key,
            &private_key,
            entries.time().now().await,
            Some(PayloadId::encode(
                Timestamp::from_u64(time_start.as_u64() + 2),
                StableHash::hash_string("s"),
            )),
        )
        .unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert_eq!(extract_plaintext(resp), vec!["2", "3"]);

        // Another key
        let (public_key, private_key) = keys(PUBLIC_KEY_2, PRIVATE_KEY_2);
        let body = SearchToken::encode(&public_key, &private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert!(extract_plaintext(resp).is_empty());
    }

    #[actix_web::test]
    async fn test_delete() {
        for (public_key, private_key) in &[
            keys(PUBLIC_KEY_1, PRIVATE_KEY_1),
            keys(PUBLIC_KEY_2, PRIVATE_KEY_2),
        ] {
            let (entries, configure) = test_app();
            let app = test::init_service(App::new().configure(configure)).await;
            let time_start = Timestamp::now();
            let encrypted = PayloadBytes::encrypt(
                public_key,
                private_key,
                Timestamp::from_u64(time_start.as_u64()),
                "foo",
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            let body = DeleteToken::encode(public_key, private_key, time_start).unwrap();
            test::call_and_read_body(&app, req_delete(body).to_request()).await;
            let got = items_plaintext(entries.storage(), public_key, private_key).await;
            assert!(got.is_empty());
        }
    }
}
