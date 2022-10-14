use actix_web::{middleware::Logger, web::Data, HttpServer};
use qqself_api_sync::{
    http::routes::http_config,
    storage::{
        account::AccountStorage, account_mem::MemoryAccountStorage, payload::PayloadStorage,
        payload_mem::MemoryPayloadStorage,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let account_storage =
        Data::new(Box::new(MemoryAccountStorage::new()) as Box<dyn AccountStorage + Sync + Send>);
    let entry_storage =
        Data::new(Box::new(MemoryPayloadStorage::new()) as Box<dyn PayloadStorage + Sync + Send>);
    HttpServer::new(move || {
        let app = actix_web::App::new()
            .configure(http_config(entry_storage.clone(), account_storage.clone()))
            .wrap(Logger::default());
        app
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
