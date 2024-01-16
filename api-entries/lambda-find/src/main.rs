use lambda_http::{run, service_fn, Error, Request, Response};
use qqself_api_entries_services::{
    entry::Entries,
    entry_storage_dynamodb::DynamoDBEntryStorage,
    service_error::{HttpCodeForError, ServiceError, ServiceErrorType},
    time::TimeOs,
};

async fn find_entries(
    entries: &Entries,
    req: Request,
) -> Result<Response<String>, ServiceErrorType> {
    let req_body = match req.into_body() {
        lambda_http::Body::Text(s) => s,
        _ => return Err(ServiceErrorType::BadInput("Not a text body".to_string())),
    };
    // TODO We need to support streaming, but for now batch everything
    let found = entries.find_batched(req_body).await?;
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(found)
        .map_err(|err| ServiceErrorType::ResponseError(err.to_string()))
}

async fn handler(entries: &Entries, req: Request) -> Result<Response<String>, Error> {
    match find_entries(entries, req).await {
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
    run(service_fn(|req: Request| async {
        handler(&entries, req).await
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
        test_helpers::{test_payload, test_timepoint, TEST_KEYS_1, TEST_KEYS_2},
    };
    use qqself_core::{
        binary_text::BinaryToText,
        date_time::timestamp::Timestamp,
        encryption::{
            hash::StableHash,
            payload::{PayloadBytes, PayloadId},
            tokens::SearchToken,
        },
    };

    use crate::handler;

    fn entries() -> Entries {
        Entries::new(Box::<MemoryEntryStorage>::default(), test_timepoint())
    }

    fn req(body: &str) -> Request<Body> {
        let fixture = r#"{"requestContext":{"http":{"method":"POST"}},"body":"[BODY]"}"#;
        let req = fixture.replace("[BODY]", body);
        lambda_http::request::from_str(&req).unwrap()
    }

    #[tokio::test]
    async fn test_bad_input() {
        let resp = handler(&entries(), req("")).await.unwrap();
        assert_eq!(resp.status(), 400);
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/json".parse().unwrap());
        assert_eq!(resp.headers(), &headers);
        assert_eq!(
            resp.body().to_string(),
            r#"{"error_code":400,"error":"BadInput. Error encoding search token. Token validation error. Failed to read binary data"}"#
        );
    }

    #[tokio::test]
    async fn test_find() {
        let entries = entries();
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
            entries.save_payload(encrypted.data()).await.unwrap();
        }
        let (public_key, private_key) = keys;
        let extract_plaintext = |data: String| {
            let mut output = Vec::new();
            for s in data.lines() {
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
        let resp = handler(&entries, req(&body)).await.unwrap();
        assert_eq!(
            extract_plaintext(resp.body().to_string()),
            vec!["1", "2", "3"]
        );

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
        let resp = handler(&entries, req(&body)).await.unwrap();
        assert_eq!(extract_plaintext(resp.body().to_string()), vec!["2", "3"]);

        // Another key
        let (public_key, private_key) = &*TEST_KEYS_2;
        let body = SearchToken::encode(public_key, private_key, time_start, None).unwrap();
        let resp = handler(&entries, req(&body)).await.unwrap();
        assert!(extract_plaintext(resp.body().to_string()).is_empty());
    }
}
