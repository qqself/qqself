use std::collections::HashMap;
use std::future::ready;
use std::pin::Pin;

use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, Select, WriteRequest};
use aws_sdk_dynamodb::Client;
use futures::{Stream, StreamExt, TryStreamExt};
use log::warn;
use qqself_core::binary_text::BinaryToText;
use qqself_core::date_time::timestamp::Timestamp;
use qqself_core::encryption::hash::StableHash;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes, PayloadId};

use super::payload_storage::{PayloadStorage, StorageErr};

pub struct PayloadStorageDynamoDB {
    client: Client,
    table: &'static str,
}

impl PayloadStorageDynamoDB {
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
impl PayloadStorage for PayloadStorageDynamoDB {
    async fn set(&self, payload: Payload, payload_id: PayloadId) -> Result<(), StorageErr> {
        self.put_item(
            payload.public_key(),
            &payload_id,
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
        last_known_id: Option<(Timestamp, StableHash)>,
    ) -> Pin<Box<dyn Stream<Item = Result<(PayloadId, PayloadBytes), StorageErr>>>> {
        let filter = if last_known_id.is_none() {
            "pk = :pk"
        } else {
            "pk = :pk AND id > :timestamp"
        };
        let mut attributes = HashMap::new();
        attributes.insert(":pk".to_string(), AttributeValue::S(public_key.to_string()));
        // Filter out all the entries with no values in payload, those were replaced by new
        // values later on, so safe to ignore
        attributes.insert(
            ":payloadType".to_string(),
            AttributeValue::S("S".to_string()),
        );
        if let Some((timestamp, _)) = &last_known_id {
            // We want all the entries after last known id timestamp
            attributes.insert(
                ":timestamp".to_string(),
                AttributeValue::S(timestamp.to_string()),
            );
        }
        let res = self
            .client
            .query()
            .table_name(self.table)
            .key_condition_expression(filter)
            .filter_expression("attribute_type(payload,:payloadType)")
            .set_expression_attribute_values(Some(attributes));
        let filter_id = last_known_id
            .map(|(timestamp, hash)| PayloadId::encode(timestamp, hash).to_string())
            .unwrap_or_default();
        let res = res
            .into_paginator()
            .items()
            .send()
            .map(process_stream_item)
            .filter(move |v| {
                // Filter out entry with id equal to last known id
                if let Ok(v) = v {
                    if v.0.to_string() == filter_id {
                        return ready(false);
                    }
                }
                ready(true)
            });
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
) -> Result<(PayloadId, PayloadBytes), StorageErr> {
    let data = match item {
        Err(err) => return Err(StorageErr::IOError(err.to_string())),
        Ok(v) => v,
    };

    // Get the payload
    let payload = data
        .get("payload")
        .ok_or_else(|| StorageErr::IOError("Payload missing".to_string()))?;
    let payload_string = payload
        .as_s()
        .map_err(|_| StorageErr::IOError("Non string payload".to_string()))?;
    let encoded = BinaryToText::new_from_encoded(payload_string.to_owned())
        .ok_or_else(|| StorageErr::IOError("Payload cannot be decoded".to_string()))?;
    let payload = PayloadBytes::new_from_encrypted(encoded)
        .map_err(|_| StorageErr::IOError("Payload cannot be read as payload bytes".to_string()))?;

    // Get payloadId which is encoded as id attribute
    let payload_id = data
        .get("id")
        .ok_or_else(|| StorageErr::IOError("Payload missing an id".to_string()))?;
    let payload_id_string = payload_id
        .as_s()
        .map_err(|_| StorageErr::IOError("Non string payload id".to_string()))?;
    Ok((PayloadId::new_encoded(payload_id_string.clone()), payload))
}
