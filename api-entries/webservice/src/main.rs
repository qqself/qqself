use std::time::Duration;

use actix_web::{
    http::KeepAlive,
    middleware::{self, Logger},
    web::Data,
    HttpServer,
};
use log::info;
use qqself_api_entries_services::{entry::Entries, time::TimeOs};
use qqself_api_entries_webservice::routes::http_config;

const PORT: u16 = 8080;

async fn entry_service() -> Data<Entries> {
    #[cfg(feature = "storage-dynamodb")]
    {
        info!("Using DynamoDB as a storage");
        let dynamo =
            qqself_api_entries_services::entry_storage_dynamodb::DynamoDBEntryStorage::new(
                "qqself_entries",
            )
            .await;
        Data::new(Entries::new(Box::new(dynamo), Box::<TimeOs>::default()))
    }
    #[cfg(not(feature = "storage-dynamodb"))]
    {
        info!("Falling back to memory storage, no data will be persisted");
        Data::new(Entries::new(
            Box::new(qqself_api_entries_services::entry_storage::MemoryEntryStorage::new()),
            Box::<TimeOs>::default(),
        ))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info,aws_config=warn"));
    info!("Listening on {PORT}");
    let entries = entry_service().await;
    HttpServer::new(move || {
        actix_web::App::new()
            .configure(http_config(entries.clone()))
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
            .wrap(Logger::default())
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(300))) // To make it bigger than AppRunner max keep alive ELB
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
