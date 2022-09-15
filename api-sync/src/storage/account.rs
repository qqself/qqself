use std::fmt::Display;

use async_trait::async_trait;
use qqself_core::{datetime::Timestamp, encryption::keys::PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub public_key: PublicKey,
    pub created_at: Timestamp,
    pub last_payment: Option<Timestamp>,
}

impl Account {
    pub fn new(public_key: PublicKey) -> Self {
        Account {
            created_at: Timestamp::now(),
            last_payment: None,
            public_key,
        }
    }
}

#[derive(Debug)]
pub enum AccountStorageErr {
    NotFound,
    IOError(String),
}

impl Display for AccountStorageErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AccountStorageErr::NotFound => "Account with such public_key not found".to_string(),
            AccountStorageErr::IOError(s) => format!("Error while loading the account {s}"),
        };
        f.write_str(&s)
    }
}

#[async_trait]
pub trait AccountStorage {
    async fn set(&self, acc: Account) -> Result<(), AccountStorageErr>;
    async fn find(&self, key: &PublicKey) -> Result<Account, AccountStorageErr>;
}
