use std::pin::Pin;
use std::sync::Mutex;

use async_trait::async_trait;
use futures::stream::{self};
use futures::Stream;
use qqself_core::datetime::Timestamp;
use qqself_core::encryption::keys::PublicKey;
use qqself_core::encryption::payload::{Payload, PayloadBytes};

use super::payload::{PayloadStorage, StorageErr};

pub struct MemoryPayloadStorage {
    data: Mutex<Vec<Option<Payload>>>,
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
                if v.as_mut().map_or(false, |v| v.id() == prev) {
                    v.take();
                }
            }
        }
        data.push(Some(payload));
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<PayloadBytes, StorageErr>>>> {
        let mut found = Vec::new();
        let data = self.data.lock().unwrap();
        let timestamp = after_timestamp.unwrap_or_else(Timestamp::zero);
        for v in data.iter().flatten() {
            if v.public_key() != public_key {
                continue;
            }
            if v.id().timestamp() <= &timestamp {
                continue;
            }
            found.push(Ok(v.data()));
        }
        Box::pin(stream::iter(found))
    }
}
