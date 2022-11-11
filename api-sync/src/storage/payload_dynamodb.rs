use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use aws_sdk_dynamodb::error::QueryError;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use futures::{Stream, StreamExt};
use qqself_core::binary_text::BinaryToText;
use qqself_core::date_time::timestamp::Timestamp;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes, PayloadId};

use super::payload::{PayloadStorage, StorageErr};

pub struct DynamoDBStorage {
    client: Client,
    table: &'static str,
}

impl DynamoDBStorage {
    pub async fn new(table: &'static str) -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        Self { client, table }
    }

    async fn dynamo_put_item(
        &self,
        public_key: &PublicKey,
        payload_id: &PayloadId,
        payload: Option<String>,
    ) -> Result<(), StorageErr> {
        let payload = match payload {
            Some(payload) => AttributeValue::S(payload),
            None => AttributeValue::Null(true),
        };
        let res = self
            .client
            .put_item()
            .table_name(self.table)
            .item("pk", AttributeValue::S(public_key.to_string()))
            .item("id", AttributeValue::S(payload_id.to_string()))
            .item("payload", payload)
            .send()
            .await;
        res.map(|_| ())
            .map_err(|err| StorageErr::IOError(err.to_string()))
    }
}

#[async_trait]
impl PayloadStorage for DynamoDBStorage {
    async fn set(&self, payload: Payload) -> Result<(), StorageErr> {
        self.dynamo_put_item(
            payload.public_key(),
            payload.id(),
            Some(payload.data().data()),
        )
        .await?;
        if let Some(prev) = payload.previous_version() {
            self.dynamo_put_item(payload.public_key(), prev, None)
                .await?;
        };
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<PayloadBytes, StorageErr>>>> {
        let filter = if after_timestamp.is_none() {
            "pk = :pk"
        } else {
            "pk = :pk AND id > :id"
        };
        let mut attributes = HashMap::new();
        attributes.insert(":pk".to_string(), AttributeValue::S(public_key.to_string()));
        // Filter out all the entries with no values in payload, those were replaced by new
        // values later on, so safe to ignore
        attributes.insert(
            ":payloadType".to_string(),
            AttributeValue::S("S".to_string()),
        );
        if let Some(timestamp) = after_timestamp {
            attributes.insert(":id".to_string(), AttributeValue::S(timestamp.to_string()));
        }
        let res = self
            .client
            .query()
            .table_name(self.table)
            .key_condition_expression(filter)
            .filter_expression("attribute_type(payload,:payloadType)")
            .set_expression_attribute_values(Some(attributes));
        let res = res.into_paginator().items().send().map(process_stream_item);
        Box::pin(res)
    }
}

fn process_stream_item(
    item: Result<HashMap<String, AttributeValue>, SdkError<QueryError>>,
) -> Result<PayloadBytes, StorageErr> {
    let data = match item {
        Err(err) => return Err(StorageErr::IOError(err.to_string())),
        Ok(v) => v,
    };
    let attribute = data
        .get("payload")
        .ok_or(StorageErr::Err("Payload missing"))?;
    let payload_string = attribute
        .as_s()
        .map_err(|_| StorageErr::Err("Non string payload"))?;
    let encoded = BinaryToText::new_from_encoded(payload_string.to_owned())
        .ok_or(StorageErr::Err("Payload cannot be decoded"))?;
    let payload = PayloadBytes::new_from_encrypted(encoded)
        .map_err(|_| StorageErr::Err("Payload cannot be read as payload bytes"))?;
    Ok(payload)
}
