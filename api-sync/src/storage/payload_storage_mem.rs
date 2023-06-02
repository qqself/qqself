use std::pin::Pin;
use std::sync::Mutex;

use async_trait::async_trait;
use futures::stream::{self};
use futures::Stream;
use qqself_core::date_time::timestamp::Timestamp;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes, PayloadId};

use super::payload_storage::{PayloadIdString, PayloadStorage, StorageErr};

pub struct PayloadStorageMemory {
    data: Mutex<Vec<(PublicKey, PayloadId, Option<Payload>)>>,
}

impl PayloadStorageMemory {
    pub fn new() -> Self {
        Self {
            data: Mutex::from(Vec::new()),
        }
    }
}

impl Default for PayloadStorageMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PayloadStorage for PayloadStorageMemory {
    async fn set(&self, payload: Payload, payload_id: PayloadId) -> Result<(), StorageErr> {
        let mut data = self.data.lock().unwrap();
        if let Some(prev) = payload.previous_version() {
            for v in data.iter_mut() {
                if &v.1 == prev {
                    v.2.take();
                }
            }
        }
        data.push((payload.public_key().clone(), payload_id, Some(payload)));
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<(PayloadIdString, PayloadBytes), StorageErr>>>> {
        let mut found = Vec::new();
        let data = self.data.lock().unwrap();
        let timestamp = after_timestamp.unwrap_or_default();
        for v in data.iter() {
            if &v.0 != public_key {
                continue;
            }
            if v.1.timestamp() < &timestamp {
                continue;
            }
            if let Some(data) = &v.2 {
                found.push(Ok((v.1.to_string(), data.data())));
            }
        }
        Box::pin(stream::iter(found))
    }

    async fn delete(&self, public_key: &PublicKey) -> Result<(), StorageErr> {
        let mut data = self.data.lock().unwrap();
        // TODO: drain_filter fits better here, use it once available in stable
        data.retain(|(key, _, _)| key != public_key);
        Ok(())
    }
}
