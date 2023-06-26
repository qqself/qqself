use std::pin::Pin;
use std::sync::Mutex;

use async_trait::async_trait;
use futures::stream;
use futures::Stream;

use qqself_core::date_time::timestamp::Timestamp;
use qqself_core::encryption::hash::StableHash;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes, PayloadId};

use super::payload_storage::{PayloadStorage, StorageErr};

pub struct PayloadStorageMemory {
    data: Mutex<Vec<(PublicKey, String, Option<Payload>)>>,
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
            if let Some(item) = data.iter_mut().find(|(public_key, payload_id, _)| {
                public_key == payload.public_key() && payload_id == &prev.to_string()
            }) {
                item.2.take();
            }
        }
        data.push((
            payload.public_key().clone(),
            payload_id.to_string(),
            Some(payload),
        ));
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        last_known_id: Option<(Timestamp, StableHash)>,
    ) -> Pin<Box<dyn Stream<Item = Result<(PayloadId, PayloadBytes), StorageErr>>>> {
        let data = self.data.lock().unwrap();
        let mut found = Vec::new();
        for (key, id, val) in data.iter() {
            if key != public_key {
                continue;
            }
            if last_known_id.as_ref().map_or(false, |(timestamp, hash)| {
                id < &timestamp.to_string()
                    || id == &PayloadId::encode(*timestamp, hash.clone()).to_string()
            }) {
                continue;
            }
            if let Some(val) = val {
                found.push(Ok((PayloadId::new_encoded(id.clone()), val.data())));
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
