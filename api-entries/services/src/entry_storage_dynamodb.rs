use std::{collections::HashMap, pin::Pin};

use async_stream::stream;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    types::{AttributeValue, DeleteRequest, Select, WriteRequest},
    Client,
};
use futures::Stream;
use log::warn;
use qqself_core::{
    binary_text::BinaryToText,
    date_time::timestamp::Timestamp,
    encryption::{
        hash::StableHash,
        keys::PublicKey,
        payload::{Payload, PayloadBytes, PayloadId},
    },
};

use crate::entry_storage::{EntryStorage, StorageErr};

pub struct DynamoDBEntryStorage {
    client: Client,
    table: &'static str,
}

impl DynamoDBEntryStorage {
    pub async fn new(table: &'static str) -> Self {
        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        Self { client, table }
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
                                    .build()
                                    .expect("DeleteRequest should be created"),
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
impl EntryStorage for DynamoDBEntryStorage {
    async fn set(&self, payload: Payload, payload_id: PayloadId) -> Result<PayloadId, StorageErr> {
        let res = self
            .client
            .put_item()
            .table_name(self.table)
            .item("pk", AttributeValue::S(payload.public_key().to_string()))
            .item("id", AttributeValue::S(payload_id.to_string()))
            .item("payload", AttributeValue::S(payload.data().data()))
            .send()
            .await;
        res.map(|_| payload_id)
            .map_err(|err| StorageErr::IOError(err.to_string()))
    }

    fn find(
        &self,
        public_key: &PublicKey,
        last_known_id: Option<(Timestamp, StableHash)>,
    ) -> Pin<Box<dyn Stream<Item = Result<(PayloadId, PayloadBytes), StorageErr>> + Send>> {
        let filter = if last_known_id.is_none() {
            "pk = :pk"
        } else {
            "pk = :pk AND id > :timestamp"
        };
        let mut attributes = HashMap::new();
        attributes.insert(":pk".to_string(), AttributeValue::S(public_key.to_string()));
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
            .set_expression_attribute_values(Some(attributes));
        let filter_id = last_known_id
            .map(|(timestamp, hash)| PayloadId::encode(timestamp, hash).to_string())
            .unwrap_or_default();
        let mut paginator = res.into_paginator().items().send();
        let stream = stream! {
            while let Some(v) = paginator.next().await {
                let data = v.map_err(|err| StorageErr::IOError(err.to_string()))?;

                // Get the payload
                let payload = data
                    .get("payload")
                    .ok_or_else(|| StorageErr::IOError("Payload missing".to_string()))?;
                let payload_string = payload
                    .as_s()
                    .map_err(|_| StorageErr::IOError("Non string payload".to_string()))?;
                let encoded = BinaryToText::new_from_encoded(payload_string.to_owned())
                    .ok_or_else(|| StorageErr::IOError("Payload cannot be decoded".to_string()))?;
                let payload = PayloadBytes::new_from_encrypted(encoded).map_err(|_| {
                    StorageErr::IOError("Payload cannot be read as payload bytes".to_string())
                })?;

                // Get payloadId which is encoded as id attribute
                let payload_id = data
                    .get("id")
                    .ok_or_else(|| StorageErr::IOError("Payload missing an id".to_string()))?;
                let payload_id_string = payload_id
                    .as_s()
                    .map_err(|_| StorageErr::IOError("Non string payload id".to_string()))?;

                // Ignore entry which is equal to the last filter_id
                if payload_id_string != &filter_id {
                  yield Ok((PayloadId::new_encoded(payload_id_string.clone()), payload));
                }
            }
        };
        Box::pin(stream)
    }

    async fn delete(&self, public_key: &PublicKey) -> Result<usize, StorageErr> {
        // DynamoDB doesn't provide a simple way to delete records by partition key without knowing the sort key.
        // We read the stream of all the sort keys by the given partition key and delete the records in batches
        let mut items = self
            .client
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
            .send();
        let mut deleted = 0;
        let mut delete_chunk = Vec::new();
        while let Some(v) = items.next().await {
            let data = v.map_err(|err| {
                warn!("Error fetching the keys for deletion: {err}");
                StorageErr::IOError("Failed to fetch keys for deletion".to_string())
            })?;

            if delete_chunk.len() == 25 {
                deleted += delete_chunk.len();
                self.batch_delete(delete_chunk).await?;
                delete_chunk = vec![];
            }

            let pk = data.get("pk").expect("pk should exists");
            let pk = pk.as_s().expect("pk should be a string").to_owned();
            let id = data.get("id").expect("id should exists");
            let id = id.as_s().expect("id should be a string").to_owned();
            delete_chunk.push((pk, id))
        }
        if !delete_chunk.is_empty() {
            deleted += delete_chunk.len();
            self.batch_delete(delete_chunk).await?;
        }
        Ok(deleted)
    }
}
