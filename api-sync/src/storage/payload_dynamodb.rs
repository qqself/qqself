use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest, DeleteRequest, Select};
use futures::{Stream, StreamExt, TryStreamExt};
use log::warn;
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

    async fn put_item(
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

    async fn batch_delete(&self, data: Vec<(String, String)>) -> Result<(), StorageErr> {
        self.client
            .batch_write_item()
            .request_items(
                self.table,
                data.into_iter()
                    .map(|(pk, id)| {
                        WriteRequest::builder()
                            .set_delete_request(Some(
                                DeleteRequest::builder()
                                    .key("pk", AttributeValue::S(pk))
                                    .key("id", AttributeValue::S(id))
                                    .build(),
                            ))
                            .build()
                    })
                    .collect(),
            )
            .send()
            .await
            .map_err(|err| {
                warn!("Error deleting the keys: {err}");
                StorageErr::IOError("Failed to delete the keys".to_string())
            })
            .map(|_| ())
    }
}

#[async_trait]
impl PayloadStorage for DynamoDBStorage {
    async fn set(&self, payload: Payload) -> Result<(), StorageErr> {
        self.put_item(
            payload.public_key(),
            payload.id(),
            Some(payload.data().data()),
        )
        .await?;
        if let Some(prev) = payload.previous_version() {
            self.put_item(payload.public_key(), prev, None).await?;
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

    async fn delete(&self, public_key: &PublicKey) -> Result<(), StorageErr> {
        // Surprisingly DynamoDB doesn't provide a simple way to delete records by partition key without knowing the sort key.
        // So first fetch all the sort keys by the given partition key and delete the records in batches
        // TODO It would be much better to return how many records we've deleted
        self.client
            .query()
            .table_name(self.table)
            .key_condition_expression("pk = :pk")
            .set_expression_attribute_values(Some(HashMap::from([(
                ":pk".to_string(),
                AttributeValue::S(public_key.to_string()),
            )])))
            .select(Select::SpecificAttributes)
            .projection_expression("pk,id")
            .into_paginator()
            .items()
            .send()
            .try_chunks(25)
            .map(|v| {
                let data = v.map_err(|err| {
                    warn!("Error fetching the keys for deletion: {err}");
                    StorageErr::IOError("Failed to fetch keys for deletion".to_string())
                })?;
                Ok(data
                    .into_iter()
                    .map(|v| {
                        let pk = v.get("pk").expect("pk should exists");
                        let pk = pk.as_s().expect("pk should be a string").to_owned();
                        let id = v.get("id").expect("id should exists");
                        let id = id.as_s().expect("id should be a string").to_owned();
                        (pk, id)
                    })
                    .collect::<Vec<(String, String)>>())
            })
            .try_for_each(|items| self.batch_delete(items))
            .await
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
        .ok_or_else(|| StorageErr::IOError("Payload missing".to_string()))?;
    let payload_string = attribute
        .as_s()
        .map_err(|_| StorageErr::IOError("Non string payload".to_string()))?;
    let encoded = BinaryToText::new_from_encoded(payload_string.to_owned())
        .ok_or_else(|| StorageErr::IOError("Payload cannot be decoded".to_string()))?;
    let payload = PayloadBytes::new_from_encrypted(encoded)
        .map_err(|_| StorageErr::IOError("Payload cannot be read as payload bytes".to_string()))?;
    Ok(payload)
}
