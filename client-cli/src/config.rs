use std::{fs, path::Path};

use qqself_core::encryption::keys::{PrivateKey, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    keys: Keys,
}

impl Config {
    pub fn new(public_key: PublicKey, private_key: PrivateKey) -> Self {
        Config {
            keys: Keys {
                public_key,
                private_key,
            },
        }
    }

    pub fn load(path: &Path) -> Self {
        let data = fs::read_to_string(path).unwrap();
        let config: Self = toml::from_str(&data).unwrap();
        config
    }

    pub fn save(&self, config_path: &Path) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(config_path, toml).unwrap();
    }

    pub fn keys(&self) -> (&PublicKey, &PrivateKey) {
        (&self.keys.public_key, &self.keys.private_key)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Keys {
    public_key: PublicKey,
    private_key: PrivateKey,
}
