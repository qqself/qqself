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
        entry_storage::MemoryEntryStorage,
        test_helpers::{items_plaintext, test_payload, test_timepoint, TEST_KEYS_1, TEST_KEYS_2},
    };
    use qqself_core::{
        binary_text::BinaryToText,
        date_time::timestamp::Timestamp,
        encryption::{
            hash::StableHash,
            payload::{PayloadBytes, PayloadId},
            tokens::{DeleteToken, SearchToken},
        },
    };

    fn test_app() -> (Data<Entries>, impl FnOnce(&mut ServiceConfig)) {
        let entries = Data::new(Entries::new(
            Box::new(MemoryEntryStorage::new()),
            test_timepoint(),
        ));
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
        assert_eq!(resp.error_code, 400);
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
        let encrypted = test_payload("entry", Timestamp::default(), &TEST_KEYS_1);
        let resp: ServiceError =
            test::call_and_read_body_json(&app, req_set(encrypted.data()).to_request()).await;
        assert_eq!(resp.error, "OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp".to_string());
        assert_eq!(resp.error_code, 400);
    }

    #[actix_web::test]
    async fn test_set_bad_input() {
        let (_, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let encrypted = test_payload("entry", Timestamp::now(), &TEST_KEYS_1);
        let resp: ServiceError =
            test::call_and_read_body_json(&app, req_set(encrypted.data() + "h").to_request()).await;
        assert_eq!(
            resp.error,
            "BadInput. Payload validation failure. Cannot read binary data".to_string()
        );
        assert_eq!(resp.error_code, 400);
    }

    #[actix_web::test]
    async fn test_set_add() {
        let (entries, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let keys = &*TEST_KEYS_1;
        for (ts, expected) in [
            (1, b"00000000001662750866|SWxyLukYqS63bYMvfwoj8f"),
            (3, b"00000000001662750869|R85af9xML6WR7fNUXNgi5V"),
            (2, b"00000000001662750871|93i31rxkhgVVzHahAA2LBF"),
        ] {
            entries
                .time()
                .sleep(std::time::Duration::from_millis(ts))
                .await;
            let encrypted = test_payload(&ts.to_string(), entries.time().now().await, keys);
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            assert_eq!(test::read_body(resp).await.to_vec(), expected);
        }
        let got = items_plaintext(entries.storage(), keys).await;
        assert_eq!(got, vec!["1", "3", "2"])
    }

    #[actix_web::test]
    async fn test_find() {
        let (entries, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let keys = &*TEST_KEYS_1;
        let time_start = entries.time().now().await;
        for ts in [1, 2, 3] {
            entries
                .time()
                .sleep(std::time::Duration::from_millis(ts))
                .await;
            let encrypted = test_payload(
                &ts.to_string(),
                Timestamp::from_u64(time_start.as_u64() + ts),
                keys,
            );
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200)
        }
        let (public_key, private_key) = keys;
        let extract_plaintext = |data: Bytes| {
            let s = String::from_utf8(data.to_vec()).unwrap();
            let mut output = Vec::new();
            for s in s.lines() {
                let payload_start_pos = s.find(':').unwrap() + 1;
                let binary_text =
                    BinaryToText::new_from_encoded(s[payload_start_pos..].to_string()).unwrap();
                let bytes = PayloadBytes::new_from_encrypted(binary_text).unwrap();
                let valid = bytes.validated(None).unwrap();
                let plaintext = valid.decrypt(private_key).unwrap();
                output.push(plaintext);
            }
            output
        };
        // Return all
        let body = SearchToken::encode(public_key, private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert_eq!(extract_plaintext(resp), vec!["1", "2", "3"]);

        // Return after
        let body = SearchToken::encode(
            public_key,
            private_key,
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
        let (public_key, private_key) = &*TEST_KEYS_2;
        let body = SearchToken::encode(public_key, private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert!(extract_plaintext(resp).is_empty());
    }

    #[actix_web::test]
    async fn test_delete() {
        for keys in [&*TEST_KEYS_1, &*TEST_KEYS_2] {
            let (entries, configure) = test_app();
            let app = test::init_service(App::new().configure(configure)).await;
            let time_start = Timestamp::now();
            let encrypted = test_payload("foo", Timestamp::from_u64(time_start.as_u64()), keys);
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            let body = DeleteToken::encode(&keys.0, &keys.1, time_start).unwrap();
            test::call_and_read_body(&app, req_delete(body).to_request()).await;
            let got = items_plaintext(entries.storage(), keys).await;
            assert!(got.is_empty());
        }
    }
}
