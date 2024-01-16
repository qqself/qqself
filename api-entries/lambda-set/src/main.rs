use lambda_http::{run, service_fn, Error, Request, Response};
use qqself_api_entries_services::{
    entry::Entries,
    entry_storage_dynamodb::DynamoDBEntryStorage,
    service_error::{HttpCodeForError, ServiceError, ServiceErrorType},
    time::TimeOs,
};

async fn set_entry(entries: &Entries, req: Request) -> Result<Response<String>, ServiceErrorType> {
    let req_body = match req.into_body() {
        lambda_http::Body::Text(s) => s,
        _ => return Err(ServiceErrorType::BadInput("Not a text body".to_string())),
    };
    let payload_id = entries
        .save_payload(req_body)
        .await
        .map(|v| v.to_string())?;
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(payload_id)
        .map_err(|err| ServiceErrorType::ResponseError(err.to_string()))
}

async fn handler(entries: &Entries, req: Request) -> Result<Response<String>, Error> {
    match set_entry(entries, req).await {
        Ok(v) => Ok(v),
        Err(err) => Ok::<Response<String>, Error>(
            Response::builder()
                .status(err.http_status_code())
                .header("content-type", "text/json")
                .body(
                    serde_json::to_string(&ServiceError::new(err)).expect("Should serialize error"),
                )
                .expect("Should create error value"),
        ),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let dynamo = DynamoDBEntryStorage::new("qqself_entries").await;
    let entries = Entries::new(Box::new(dynamo), Box::<TimeOs>::default());
    run(service_fn(|event: Request| async {
        handler(&entries, event).await
    }))
    .await
}

#[cfg(test)]
mod tests {
    use lambda_http::{
        http::{HeaderMap, Request},
        Body,
    };
    use qqself_api_entries_services::{
        entry::Entries,
        entry_storage::MemoryEntryStorage,
        test_helpers::{items_plaintext, test_payload, test_timepoint, TEST_KEYS_1},
    };
    use qqself_core::date_time::timestamp::Timestamp;

    use crate::handler;

    fn entries() -> Entries {
        Entries::new(Box::<MemoryEntryStorage>::default(), test_timepoint())
    }

    fn req(body: &str) -> Request<Body> {
        let fixture = r#"{"requestContext":{"http":{"method":"GET"}},"body":"[BODY]"}"#;
        let req = fixture.replace("[BODY]", body);
        lambda_http::request::from_str(&req).unwrap()
    }

    #[tokio::test]
    async fn test_empty_body() {
        let resp = handler(&entries(), req("")).await.unwrap();
        assert_eq!(resp.status(), 400);
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/json".parse().unwrap());
        assert_eq!(resp.headers(), &headers);
        assert_eq!(
            resp.body().to_string(),
            r#"{"error_code":400,"error":"BadInput. Payload validation failure. Cannot read binary data"}"#
        );
    }

    #[tokio::test]
    async fn test_bad_body() {
        let encrypted = test_payload("entry", Timestamp::now(), &TEST_KEYS_1);
        let resp = handler(&entries(), req(&(encrypted.data() + "h")))
            .await
            .unwrap();
        assert_eq!(resp.status(), 400);
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/json".parse().unwrap());
        assert_eq!(resp.headers(), &headers);
        assert_eq!(
            resp.body().to_string(),
            r#"{"error_code":400,"error":"BadInput. Payload validation failure. Cannot read binary data"}"#
        );
    }

    #[tokio::test]
    async fn test_too_old_payload() {
        let encrypted = test_payload("entry", Timestamp::default(), &TEST_KEYS_1);
        let resp = handler(&entries(), req(&encrypted.data())).await.unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/json".parse().unwrap());
        assert_eq!(resp.headers(), &headers);
        assert_eq!(
            resp.body().to_string(),
            r#"{"error_code":400,"error":"OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp"}"#
        );
    }

    #[tokio::test]
    async fn test_add() {
        let entries = entries();
        let keys = &*TEST_KEYS_1;
        for (ts, expected) in [
            (1, "00000000001662750866|SWxyLukYqS63bYMvfwoj8f"),
            (3, "00000000001662750869|R85af9xML6WR7fNUXNgi5V"),
            (2, "00000000001662750871|93i31rxkhgVVzHahAA2LBF"),
        ] {
            entries
                .time()
                .sleep(std::time::Duration::from_millis(ts))
                .await;
            let encrypted = test_payload(&ts.to_string(), entries.time().now().await, keys);
            let resp = handler(&entries, req(&encrypted.data())).await.unwrap();
            assert_eq!(resp.status(), 200);
            assert_eq!(resp.body().to_string(), expected);
        }
        let got = items_plaintext(entries.storage(), keys).await;
        assert_eq!(got, vec!["1", "3", "2"])
    }
}
