use std::{fs, path::Path};

use qqself_core::encryption::cryptor::Cryptor;

pub struct KeyFile(Cryptor);

impl KeyFile {
    pub fn generate_new() -> Self {
        let cryptor = Cryptor::generate_new();
        Self(cryptor)
    }

    pub fn load_from_file(path: &Path) -> KeyFile {
        let data = fs::read_to_string(path).expect("key file should be available");
        Self(
            Cryptor::from_deserialized_keys(data).expect("key file should contain key information"),
        )
    }

    pub fn save_to_file(&self, path: &Path) {
        let data = self.0.serialize_keys();
        fs::write(path, data).unwrap();
    }

    pub fn cryptor(self) -> Cryptor {
        self.0
    }
}
