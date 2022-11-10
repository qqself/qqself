use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use qqself_core::{
    datetime::Timestamp,
    encryption::{
        keys::PublicKey,
        payload::{Payload, PayloadBytes},
    },
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum StorageErr {
    Err(&'static str),
    IOError(String),
}

#[async_trait]
pub trait PayloadStorage {
    async fn set(&self, payload: Payload) -> Result<(), StorageErr>;
    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<PayloadBytes, StorageErr>>>>;
}

#[cfg(test)]
mod tests {
    use crate::storage::{payload_fs::FSPayloadStorage, payload_mem::MemoryPayloadStorage};

    use super::*;
    use futures::StreamExt;
    use qqself_core::{
        binary_text::BinaryToText,
        encryption::{keys::PrivateKey, payload::PayloadId},
    };

    const PUBLIC_KEY_1: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqo47AoLLabVv7Ycj6VoEZj1Gz9YPPDhcUjbkzgzLb5n799MydJYdRLA17wDAuvNTcJ4m27F2jzg7Zv26r94eYbRRrYH6oauQGPr9a6XyvNKTzykLkU9m5C3vEnpTVai2NMdib9JiEeJUMUSaApNd4r3ZF9i46suP7qD9gimj2USuh1QHY3r9YKmcyurkZRGZhjyXAnbae98vuJtUxVyMMzV9QWkV1BodGMFc4gE77HhULKk1Z23igQWJZsDTUDhiZdLxs5pmW1699zEgNt42PtJGxQ4ouL5UZcNv42UpUrrXsnKpAKLkRKZTfpsdp4zmPYfSjMNqPQLqiyDLw1B1b5Vs23pAYNMNJoBJXp3wMsJFngqPtPDWZ9Bgm5361uAZa2yNBBfaJMoumTjAPY54MWzYbeqj7mB7ZvLm1351SVJn8rNqrHAE6fNxbruJVwjzbKzbLmD859ZBd2F1V4SKRQZSAymj9sfJYYCn3Z6KoKzBSgH2QYXoTb93dVGDGqegfwZ9EYq";
    const PRIVATE_KEY_1: &str = "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";
    const PUBLIC_KEY_2: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqEKPRmocEuhWJthsJJ44rdBUxeWKdcUfKFdThfJ5N9ZL41CYjJWQahFqkf6KcZcKMrzLwuvvNL3PQJ7Ly688RrUdYrY8NozzkiK1rUcBZCWAcXvavpDkxh46j7si82S1oZmpkfVT77nkwdHhUYNMmSxwnjQj3iTDqVYJqext2ZM11TMsr5FSfQrr1WUv4ZLyjMkE7uVHsJih3kB4dkfg5LYJfUfnGw9fDfRyCk6YFQ7t2HxTJRxigMxACSJpbCCgza2iA1LdEA8rYCKnLWsCeXMZC4fezu7Vhb2ivuYYRc16vkcJKq4xKxhp6mXWP5jrY9Y2dMWrMrBVeiE8ac1Z6D1EdvanJUzHisyRXg3K2DKmrvzKer7f67pXjHYUnLh9s3owzULxGsCZ88BLmMgrw5JrTvbYoRhACaUksFTUhJxAHJAysVDVHSr4PEhd6oqzPoM4TqFLYmaa5aYGBL8NgqJarvWvM59finsXpVvi2zcZ7hbFGHE2vLjqZpyjeiJNBc6DyEh";
    const PRIVATE_KEY_2: &str = "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";

    struct Keys {
        public: PublicKey,
        private: PrivateKey,
    }

    fn keys(public: &str, private: &str) -> Keys {
        let public =
            PublicKey::new(BinaryToText::new_from_encoded(public.to_string()).unwrap()).unwrap();
        let private =
            PrivateKey::new(BinaryToText::new_from_encoded(private.to_string()).unwrap()).unwrap();
        Keys { public, private }
    }

    async fn items_raw<S: PayloadStorage>(
        keys: &Keys,
        s: &S,
        after_timestamp: Option<u64>,
    ) -> Vec<Payload> {
        s.find(&keys.public, after_timestamp.map(Timestamp::new))
            .map(|v| v.unwrap())
            .map(|v| v.validated(None).unwrap())
            .collect::<Vec<_>>()
            .await
    }

    async fn items<S: PayloadStorage>(
        keys: &Keys,
        s: &S,
        after_timestamp: Option<u64>,
    ) -> Vec<String> {
        items_raw(keys, s, after_timestamp)
            .await
            .into_iter()
            .map(|v| v.decrypt(&keys.private).unwrap().text().to_string())
            .collect()
    }

    fn payload(
        keys: &Keys,
        timestamp: u64,
        plaintext: u64,
        previous: Option<PayloadId>,
    ) -> Payload {
        let plaintext = plaintext.to_string();
        let encrypted = PayloadBytes::encrypt(
            &keys.public,
            &keys.private,
            Timestamp::new(timestamp),
            &plaintext,
            previous,
        )
        .unwrap();
        encrypted.validated(None).unwrap()
    }

    async fn test_storage<S: PayloadStorage>(storage: S) {
        let keys1 = &keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let keys2 = &keys(PUBLIC_KEY_2, PRIVATE_KEY_2);

        // Default is empty
        assert!(items(keys1, &storage, None).await.is_empty());

        // Search all
        storage.set(payload(keys1, 1, 1, None)).await.unwrap();
        storage.set(payload(keys1, 2, 2, None)).await.unwrap();
        storage.set(payload(keys1, 3, 3, None)).await.unwrap();
        assert_eq!(items(keys1, &storage, None).await, vec!["1", "2", "3"]);
        assert!(items(keys2, &storage, None).await.is_empty());

        // Return items after timestamp
        assert_eq!(items(keys1, &storage, Some(2)).await, vec!["2", "3"]);

        // Add entires for other keys
        storage.set(payload(keys2, 1, 1, None)).await.unwrap();
        assert_eq!(items(keys1, &storage, None).await, vec!["1", "2", "3"]);
        assert_eq!(items(keys2, &storage, None).await, vec!["1"]);

        // Reset value
        let existing = items_raw(keys1, &storage, None).await;
        storage
            .set(payload(keys1, 4, 4, Some(existing[0].id().clone())))
            .await
            .unwrap();
        assert_eq!(items(keys1, &storage, None).await, vec!["2", "3", "4"]);

        // Additional entry with the same timestamp should be appended and not overwrite the existing entry
        storage.set(payload(keys1, 4, 5, None)).await.unwrap();
        let mut got = items(keys1, &storage, Some(4)).await;
        got.sort(); // Storages returns items sorted by timestamp but for enrties with the same timestamp order is not defined
        assert_eq!(got, vec!["4", "5"]);
    }

    #[tokio::test]
    async fn memory_storage() {
        test_storage(MemoryPayloadStorage::new()).await;
    }

    #[tokio::test]
    async fn fs_storage() {
        test_storage(FSPayloadStorage::new_temp()).await;
    }

    #[cfg(feature = "storage-dynamodb")]
    #[tokio::test]
    async fn dynamo_storage() {
        use crate::storage::payload_dynamodb::DynamoDBStorage;
        test_storage(DynamoDBStorage::new("qqself_entries").await).await;
    }
}
