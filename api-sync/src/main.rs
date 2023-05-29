use actix_web::{
    middleware::{self, Logger},
    web::Data,
    HttpServer,
};
use log::info;
use qqself_api_sync::{
    http::routes::http_config,
    storage::{
        account::AccountStorage, account_mem::MemoryAccountStorage, payload::PayloadStorage,
        payload_mem::MemoryPayloadStorage,
    },
};

#[allow(unreachable_code)] // Fallback memory storage may be not always used
async fn memory_storage() -> Data<Box<dyn PayloadStorage + Send + Sync>> {
    #[cfg(feature = "storage-dynamodb")]
    {
        info!("Using DynamoDB as a storage");
        let dynamo =
            qqself_api_sync::storage::payload_dynamodb::DynamoDBStorage::new("qqself_entries")
                .await;
        return Data::new(Box::new(dynamo) as Box<dyn PayloadStorage + Sync + Send>);
    }
    info!("Falling back to memory storage, no data will be persisted");
    Data::new(Box::new(MemoryPayloadStorage::new()) as Box<dyn PayloadStorage + Sync + Send>)
}

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info,aws_config=warn"));
    let account_storage =
        Data::new(Box::new(MemoryAccountStorage::new()) as Box<dyn AccountStorage + Sync + Send>);
    let entry_storage = memory_storage().await;
    info!("Listening on {PORT}");
    HttpServer::new(move || {
        actix_web::App::new()
            .configure(http_config(entry_storage.clone(), account_storage.clone()))
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
