use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::read_dir;
use std::path::Path;
use std::pin::Pin;

use futures::future;
use futures::{Future, Stream, StreamExt};

pub trait Storage {
    // TODO Should we remove mut from save? Should there be only one storage per thread or singleton?
    fn save(&mut self, key: String, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = ()>>>;
    fn read(
        &self,
        prefix: String,
        start_with: String,
    ) -> Box<dyn Stream<Item = (String, Vec<u8>)> + Unpin + '_>;
}

#[derive(Debug)]
pub struct MemoryStorage {
    pub m: HashMap<String, Vec<u8>>,
}

impl Storage for MemoryStorage {
    fn save(&mut self, key: String, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = ()>>> {
        self.m.insert(key, payload);
        Box::pin(async { () })
    }

    fn read(
        &self,
        prefix: String,
        start_with: String,
    ) -> Box<dyn Stream<Item = (String, Vec<u8>)> + Unpin + '_> {
        let stream = futures::stream::iter(self.m.iter())
            .filter(move |(k, v)| future::ready(k.starts_with(&prefix) && *k > &start_with))
            .map(|(k, v)| (k.clone(), v.clone()));
        Box::new(stream)
    }
}

#[derive(Debug)]
struct FileStorage {
    path: &'static str,
}

impl Storage for FileStorage {
    fn save(&mut self, key: String, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = ()>>> {
        let path = Path::new(self.path);
        std::fs::write(path.join(key), payload).unwrap();
        Box::pin(async { () })
    }

    fn read(
        &self,
        prefix: String,
        start_with: String,
    ) -> Box<dyn Stream<Item = (String, Vec<u8>)> + Unpin + '_> {
        let mut files = Vec::new();
        for file in read_dir(&self.path).unwrap() {
            let file = file.unwrap();
            let name = file.file_name().into_string().unwrap();
            if name.starts_with(&prefix) && name > start_with {
                // TODO It's a blocking call - move to tokio based one
                let data = std::fs::read(file.path()).unwrap();
                files.push((name, data));
            }
        }
        Box::new(futures::stream::iter(files))
    }
}

#[derive(Debug)]
struct S3Storage {
    bucket: &'static str,
}

impl Storage for S3Storage {
    fn save(&mut self, key: String, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = ()>>> {
        todo!()
    }

    fn read(
        &self,
        prefix: String,
        start_with: String,
    ) -> Box<dyn Stream<Item = (String, Vec<u8>)> + Unpin + '_> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_storage<S: Storage + Debug>(mut storage: S) {
        // Nothing in storage
        let data = storage.read("prefix".to_string(), "prefix_0".to_string());
        let items = data.collect::<Vec<_>>().await;
        assert_eq!(
            items.len(),
            0,
            "Expected no items in storage {:#?}",
            storage
        );

        // Add + Read
        storage.save("prefix_1".to_string(), vec![1]).await;
        storage.save("prefix_2".to_string(), vec![2]).await;
        storage.save("prefix_3".to_string(), vec![3]).await;
        let data = storage.read("prefix".to_string(), "prefix_0".to_string());
        let items = data.collect::<Vec<_>>().await;
        assert_eq!(items.len(), 3);
        assert!(items.contains(&("prefix_1".to_string(), vec![1u8])));
        assert!(items.contains(&("prefix_2".to_string(), vec![2u8])));
        assert!(items.contains(&("prefix_3".to_string(), vec![3u8])));

        // Read only after start
        let data = storage.read("prefix".to_string(), "prefix_2".to_string());
        let items = data.collect::<Vec<_>>().await;
        assert_eq!(items.len(), 1);
        assert!(items.contains(&("prefix_3".to_string(), vec![3u8])));

        // Read items matches prefix
        storage.save("another_1".to_string(), vec![1]).await;
        storage.save("another_2".to_string(), vec![2]).await;
        let data = storage.read("another".to_string(), "another_0".to_string());
        let items = data.collect::<Vec<_>>().await;
        assert_eq!(items.len(), 2);
        assert!(items.contains(&("another_1".to_string(), vec![1u8])));
        assert!(items.contains(&("another_2".to_string(), vec![2u8])));
    }

    fn test_dir() -> &'static str {
        let dir = "/tmp/test_dir";
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        dir
    }

    #[test]
    fn memory_storage() {
        tokio_test::block_on(test_storage(MemoryStorage { m: HashMap::new() }));
    }

    #[test]
    fn file_storage() {
        tokio_test::block_on(test_storage(FileStorage { path: test_dir() }));
    }

    #[test]
    fn s3_storage() {
        tokio_test::block_on(test_storage(S3Storage {
            bucket: "qqself.test",
        }));
    }
}
