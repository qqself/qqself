use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
    pin::Pin,
};

use async_trait::async_trait;
use futures::{stream, Stream, StreamExt};
use qqself_core::{
    binary_text::BinaryToText,
    date_time::timestamp::Timestamp,
    encryption::{
        keys::PublicKey,
        payload::{Payload, PayloadBytes, PayloadId},
    },
};

use super::payload::{PayloadStorage, StorageErr};

pub struct FSPayloadStorage {
    path: PathBuf,
}

impl FSPayloadStorage {
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn new_temp() -> Self {
        // OK to unwrap here as temp_dir used mostly for testing/debugging
        let temp = tempdir::TempDir::new("").expect("temp dir").into_path();
        Self { path: temp }
    }

    fn save_file(
        &self,
        public_key: &PublicKey,
        id: &PayloadId,
        data: Option<String>,
    ) -> Result<(), StorageErr> {
        let name = format!(
            "{}|{}|{}",
            public_key.hash_string(),
            id.timestamp(),
            id.hash()
        );
        match data {
            Some(data) => std::fs::write(self.path.join(name), data)
                .map_err(|err| StorageErr::IOError(format!("Failed to write payload: {err}"))),
            None => std::fs::remove_file(self.path.join(name)).map_err(|err| {
                StorageErr::IOError(format!("Failed to delete previous payload: {}", err))
            }),
        }
    }

    fn find_files(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Result<Vec<PayloadBytes>, StorageErr> {
        let mut found = Vec::new();
        let listing = read_dir(&self.path)
            .map_err(|_| StorageErr::IOError("Failed to read_dir".to_string()))?;
        let timestamp = after_timestamp.unwrap_or_default();
        for file in listing {
            let file =
                file.map_err(|_| StorageErr::IOError("Failed to read folder file".to_string()))?;
            let name = file
                .file_name()
                .into_string()
                .map_err(|_| StorageErr::IOError("Failed to get file name".to_string()))?;
            let prefix = format!("{}|", public_key.hash_string());
            if !name.starts_with(&prefix) {
                continue; // File for other public_key
            }
            let prefix_time = format!("{}|{}|", public_key.hash_string(), timestamp);
            if name.len() < prefix_time.len() {
                continue; // Some invalid file
            }
            let file_time_prefix = name[..prefix_time.len()].to_string();
            if file_time_prefix < prefix_time {
                continue; // File older than what we find
            }
            let data = std::fs::read(file.path())
                .map_err(|_| StorageErr::IOError("Failed to read file data".to_string()))?;
            let data_string = String::from_utf8(data)
                .map_err(|_| StorageErr::IOError("Failed to convert file data to string".to_string()))?;
            let encoded = BinaryToText::new_from_encoded(data_string).ok_or_else(|| {
                StorageErr::IOError("Failed to read file as encoded data".to_string())
            })?;
            let payload = PayloadBytes::new_from_encrypted(encoded).map_err(|_| {
                StorageErr::IOError("Failed to read file as payload bytes".to_string())
            })?;
            found.push((file_time_prefix, payload))
        }
        // File system may return files in random order, sort it here
        found.sort_by(|a, b| a.0.cmp(&b.0));
        let found: Vec<_> = found.into_iter().map(|v| v.1).collect();
        Ok(found)
    }

    fn delete_files(&self, public_key: &PublicKey) -> Result<(), StorageErr> {
        let listing = read_dir(&self.path)
            .map_err(|_| StorageErr::IOError("Failed to read_dir".to_string()))?;
        for file in listing {
            let file =
                file.map_err(|_| StorageErr::IOError("Failed to read folder file".to_string()))?;
            let name = file
                .file_name()
                .into_string()
                .map_err(|_| StorageErr::IOError("Failed to get file name".to_string()))?;
            let prefix = format!("{}|", public_key.hash_string());
            if name.starts_with(&prefix) {
                fs::remove_file(file.path()).map_err(|err| {
                    StorageErr::IOError(format!("Failed to delete the file: {}", err))
                })?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl PayloadStorage for FSPayloadStorage {
    async fn set(&self, payload: Payload) -> Result<(), StorageErr> {
        // We can't use public_key as a file name as it can be much bigger than file names limits
        // FSStorage is mostly used for local deployments, so using hash instead is fine
        self.save_file(
            payload.public_key(),
            payload.id(),
            Some(payload.data().data()),
        )?;
        if let Some(prev) = payload.previous_version() {
            self.save_file(payload.public_key(), prev, None)?;
        };
        Ok(())
    }

    fn find(
        &self,
        public_key: &PublicKey,
        after_timestamp: Option<Timestamp>,
    ) -> Pin<Box<dyn Stream<Item = Result<PayloadBytes, StorageErr>>>> {
        let files = match self.find_files(public_key, after_timestamp) {
            Ok(v) => v,
            Err(err) => return Box::pin(stream::iter(vec![Err(err)])),
        };
        Box::pin(stream::iter(files).map(Ok))
    }

    async fn delete(&self, public_key: &PublicKey) -> Result<(), StorageErr> {
        self.delete_files(public_key)
    }
}
