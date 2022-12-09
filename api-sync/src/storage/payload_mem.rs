use std::pin::Pin;
use std::sync::Mutex;

use async_trait::async_trait;
use futures::stream::{self};
use futures::Stream;
use qqself_core::date_time::timestamp::Timestamp;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes, PayloadId};

use super::payload::{PayloadStorage, StorageErr};

pub struct MemoryPayloadStorage {
    data: Mutex<Vec<(PublicKey, PayloadId, Option<Payload>)>>,
}

impl MemoryPayloadStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::from(Vec::new()),
        }
    }
}

impl Default for MemoryPayloadStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PayloadStorage for MemoryPayloadStorage {
    async fn set(&self, payload: Payload) -> Result<(), StorageErr> {
        let mut data = self.data.lock().unwrap();
        if let Some(prev) = payload.previous_version() {
            for v in data.iter_mut() {
                if &v.1 == prev {
                    v.2.take();
                }
            }
        }
        data.push((
            payload.public_key().clone(),
            payload.id().clone(),
            Some(payload),
        ));
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<PayloadBytes, StorageErr>>>> {
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
                found.push(Ok(data.data()));
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
