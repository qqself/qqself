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
        entry::Entries, entry_storage::MemoryEntryStorage, time::TimeOs,
    };

    use crate::handler;

    fn entries() -> Entries {
        Entries::new(
            Box::<MemoryEntryStorage>::default(),
            Box::<TimeOs>::default(),
        )
    }

    fn req(body: &str) -> Request<Body> {
        let fixture = r#"{"requestContext":{"http":{"method":"GET"}},"body":"[BODY]"}"#;
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
}
