use std::collections::HashMap;
use std::io::ErrorKind;
use std::ops::Deref;
use std::rc::Rc;
use std::str::Utf8Error;
use std::sync::Arc;
use std::task::Poll;

use actix_web::body::Body;
use actix_web::dev::BodyEncoding;
use actix_web::http;
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::web::{Bytes, ServiceConfig};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::stream::poll_fn;
use futures::{future::ok, stream::once, Stream};
use futures_util::{StreamExt, TryStreamExt};
use serde::de::{Error, Unexpected};
use serde::{de, Serialize};
use serde::{Deserialize, Deserializer, Serializer};

use crate::storage::Storage;

type ClientId = String;

enum Base64String {
    PlainText(String),
    Encoded(String),
}

impl Serialize for Base64String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Base64String::PlainText(s) => {
                let data = base64::encode(&s);
                return serializer.serialize_str(&data);
            }
            _ => unreachable!("Only plain text data can be serialized"),
        };
    }
}

impl<'de> Deserialize<'de> for Base64String {
    fn deserialize<D>(deserializer: D) -> Result<Base64String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = String::deserialize(deserializer)?;
        let decoded = base64::decode(data).map_err(de::Error::custom)?;
        let text = String::from_utf8(decoded).map_err(de::Error::custom)?;
        Ok(Base64String::PlainText(text))
    }
}

#[derive(Deserialize, Serialize)]
struct SyncRequest {
    client_id: ClientId,
    operation_index: usize,
    sync_status: HashMap<ClientId, usize>,
    public_key: String,
    payload: Option<Base64String>,
}

#[derive(Deserialize, Serialize)]
struct SyncResponseLine {
    client_id: String,
    operation_index: usize,
    payload: Base64String,
}

async fn health(req: HttpRequest) -> impl Responder {
    "OK"
}

async fn sync(req: web::Json<SyncRequest>, data: web::Data<Box<dyn Storage>>) -> impl Responder {
    // TODO Validate request
    // TODO Save data if presented
    // TODO Return data
    // TODO Figure out reading operation_index
    // TODO Call read for every supplied client_id
    // TODO Should we change Storage trait to handle key/prefix creation instead of here?

    // HACK Couldn't figure out any other way. We return stream here which expects
    // data to be 'static. Performance wise I guess it's OK as storage lives all the time
    let data = Box::leak(Box::new(data));
    let stream =
        data.read("key_".to_string(), "key_".to_string())
            .map(move |(client_id, payload)| {
                let line = SyncResponseLine {
                    client_id,
                    operation_index: 0,
                    payload: Base64String::PlainText(String::from_utf8(payload).unwrap()),
                };
                let mut data = serde_json::to_string(&line).unwrap();
                data.push('\n');
                return Ok(Bytes::from(data)) as Result<web::Bytes, ()>;
            });
    HttpResponse::build(StatusCode::OK)
        .encoding(ContentEncoding::Gzip)
        .set_header(http::header::CONTENT_TYPE, "application/jsonlines")
        .streaming(stream)
}

pub fn http_config(storage: Box<dyn Storage>) -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.data(storage)
            .route("/health", web::get().to(health))
            .route("/sync", web::post().to(sync));
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use actix_web::http::Method;
    use actix_web::web::Buf;
    use actix_web::{test, web, App};
    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    use crate::storage::MemoryStorage;

    use super::*;

    #[actix_rt::test]
    async fn sync() {
        let mut storage = MemoryStorage { m: HashMap::new() };
        storage.save("key_1".to_string(), vec![0x00]);
        storage.save("key_2".to_string(), vec![0x01]);
        storage.save("key_3".to_string(), vec![0x02]);
        let app = App::new().configure(http_config(Box::new(storage)));
        let mut app = test::init_service(app).await;
        let data = SyncRequest {
            client_id: "".to_string(),
            operation_index: 0,
            sync_status: Default::default(),
            public_key: "".to_string(),
            payload: None,
        };
        let req = test::TestRequest::with_uri("/sync")
            .set_json(&data)
            .method(Method::POST)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let data = test::read_body(resp).await;
        let got_body = String::from_utf8_lossy(&data.bytes());
        let want = r#"{"client_id":"key_1","operation_index":0,"payload":"AA=="}
{"client_id":"key_2","operation_index":0,"payload":"AQ=="}
{"client_id":"key_3","operation_index":0,"payload":"Ag=="}
"#;
        assert_eq!(got_body, want);
    }

    #[actix_rt::test]
    async fn health() {
        let mut storage = MemoryStorage { m: HashMap::new() };
        let app = App::new().configure(http_config(Box::new(storage)));
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::with_uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[test]
    fn client_op_no_other_ops() {}

    #[test]
    fn client_op_other_ops() {}
}
