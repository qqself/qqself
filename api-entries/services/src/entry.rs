use std::pin::Pin;

use futures::{stream::Peekable, Stream, StreamExt, TryStreamExt};
use log::warn;
use qqself_core::{
    binary_text::BinaryToText,
    date_time::{datetime::Duration, timestamp::Timestamp},
    encryption::{
        payload::{Payload, PayloadBytes, PayloadError, PayloadId},
        tokens::{DeleteToken, SearchToken},
    },
};

use crate::{entry_storage::EntryStorage, service_error::ServiceErrorType, time::TimeProvider};

const MAX_PAYLOAD_AGE: Duration = Duration::new(1, 0);

pub struct Entries {
    storage: Box<dyn EntryStorage + Send + Sync>,
    time: Box<dyn TimeProvider + Send + Sync>,
}

type StreamItem = Result<(PayloadId, PayloadBytes), ServiceErrorType>;

impl Entries {
    pub fn new(
        storage: Box<dyn EntryStorage + Send + Sync>,
        time: Box<dyn TimeProvider + Send + Sync>,
    ) -> Self {
        Self { storage, time }
    }

    pub async fn save_payload(&self, payload_data: String) -> Result<PayloadId, ServiceErrorType> {
        let now = self.time.now().await;
        let payload = Entries::validate_payload(payload_data, now).await?;
        let payload_id = PayloadId::encode(now, payload.plaintext_hash().clone());
        self.storage
            .set(payload, payload_id)
            .await
            .map_err(|err| ServiceErrorType::IOError(format!("{:#?}", err)))
    }

    pub async fn delete(&self, token_data: String) -> Result<usize, ServiceErrorType> {
        let now = self.time.now().await;
        let delete_token = Entries::validate_delete_token(token_data, now)?;
        self.storage
            .delete(delete_token.public_key())
            .await
            .map_err(|err| ServiceErrorType::IOError(format!("{:#?}", err)))
    }

    pub fn time(&self) -> &dyn TimeProvider {
        &*self.time
    }

    pub fn storage(&self) -> &dyn EntryStorage {
        &*self.storage
    }

    pub async fn find(
        &self,
        token_data: String,
    ) -> Result<
        Peekable<Pin<Box<dyn Stream<Item = Result<(PayloadId, PayloadBytes), ServiceErrorType>>>>>,
        ServiceErrorType,
    > {
        let now = self.time.now().await;
        let search_token = Entries::validate_search_token(token_data, now)?;
        let stream: Pin<Box<dyn Stream<Item = StreamItem>>> = Box::pin(
            self.storage
                .find(
                    search_token.public_key(),
                    search_token
                        .last_known_id()
                        .to_owned()
                        .and_then(|v| v.decode()),
                )
                .map_err(|err| {
                    warn!("Storage find error {:?}", err);
                    ServiceErrorType::IOError("Streaming error".to_string())
                }),
        );

        Ok(stream.peekable())
    }

    async fn validate_payload(
        payload_data: String,
        now: Timestamp,
    ) -> Result<Payload, ServiceErrorType> {
        let encoded = BinaryToText::new_from_encoded(payload_data).ok_or_else(|| {
            ServiceErrorType::BadInput("Error validating encoded payload".to_string())
        })?;
        let payload_bytes = PayloadBytes::new_from_encrypted(encoded).map_err(|_| {
            ServiceErrorType::BadInput("Error validating encrypted payload".to_string())
        })?;

        // Validation is CPU heavy and may take about 2ms, use thread pool to avoid blocking event loop
        // TODO What about SearchToken and DeleteToken, those are implicitly validated and blocks the event loop
        let payload = tokio::task::spawn_blocking(move || {
            payload_bytes.validated(Some(now - MAX_PAYLOAD_AGE))
        })
        .await
        .map_err(|_| ServiceErrorType::IOError("Error calling payload verification".to_string()))?;
        payload.map_err(|err| match err {
            PayloadError::TimestampIsTooOld => ServiceErrorType::OutdatedPayload,
            _ => ServiceErrorType::BadInput(format!("Payload validation failure. {}", err)),
        })
    }

    fn validate_search_token(
        data: String,
        now: Timestamp,
    ) -> Result<SearchToken, ServiceErrorType> {
        SearchToken::decode(data, Some(now - MAX_PAYLOAD_AGE)).map_err(|err| {
            ServiceErrorType::BadInput(format!("Error encoding search token. {}", err))
        })
    }

    fn validate_delete_token(
        data: String,
        now: Timestamp,
    ) -> Result<DeleteToken, ServiceErrorType> {
        DeleteToken::decode(data, Some(now - MAX_PAYLOAD_AGE)).map_err(|err| {
            ServiceErrorType::BadInput(format!("Error encoding delete token. {}", err))
        })
    }
}
