use std::{fs, path::Path};

use qqself_core::encryption::keys::Keys;

pub struct KeyFile(Keys);

impl KeyFile {
    pub fn new(keys: Keys) -> Self {
        Self(keys)
    }

    pub fn load(path: &Path) -> Self {
        let data = fs::read_to_string(path).expect("key file should be available");
        let keys = Keys::deserialize(data).expect("key file should contain key information");
        Self(keys)
    }

    pub fn save(&self, config_path: &Path) {
        let data = self.0.serialize();
        fs::write(config_path, data).unwrap();
    }

    pub fn keys(&self) -> &Keys {
        &self.0
    }
}
