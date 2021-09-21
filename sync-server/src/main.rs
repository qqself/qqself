use actix_web::{middleware, HttpServer};

use crate::http::http_config;
use crate::storage::MemoryStorage;
use actix_web::http::ContentEncoding;
use std::collections::HashMap;

mod http;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let storage = Box::new(MemoryStorage { m: HashMap::new() });
        let app = actix_web::App::new().configure(http_config(storage));
        app.wrap(middleware::Compress::new(ContentEncoding::Gzip))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
