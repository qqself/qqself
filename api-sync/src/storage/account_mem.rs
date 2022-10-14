use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use qqself_core::encryption::keys::PublicKey;

use super::account::{Account, AccountStorage, AccountStorageErr};

pub struct MemoryAccountStorage {
    data: Mutex<HashMap<String, Account>>,
}

impl MemoryAccountStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AccountStorage for MemoryAccountStorage {
    async fn set(&self, acc: Account) -> Result<(), AccountStorageErr> {
        self.data
            .lock()
            .unwrap()
            .insert(acc.public_key.to_string(), acc);
        Ok(())
    }

    async fn find(&self, key: &PublicKey) -> Result<Account, AccountStorageErr> {
        match self.data.lock().unwrap().get(&key.to_string()) {
            Some(acc) => Ok(acc.clone()),
            None => Err(AccountStorageErr::NotFound),
        }
    }
}
