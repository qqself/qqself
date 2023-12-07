use qqself_core::encryption::cryptor::Cryptor as BaseCryptor;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Cryptor(BaseCryptor);

#[wasm_bindgen]
impl Cryptor {
    pub fn generate_new() -> Self {
        Self(BaseCryptor::generate_new())
    }

    pub fn from_deserialized_keys(data: String) -> Result<Cryptor, String> {
        Ok(Self(
            BaseCryptor::from_deserialized_keys(data).map_err(|err| err.to_string())?,
        ))
    }

    pub fn serialize_keys(&self) -> String {
        self.0.serialize_keys()
    }

    pub fn decrypt(&self, data: String) -> Result<String, String> {
        self.0.decrypt(data).map_err(|err| err.to_string())
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        self.0.encrypt(plaintext).map_err(|err| err.to_string())
    }

    pub fn sign_delete_token(&self) -> Result<String, String> {
        self.0.sign_delete_token().map_err(|err| err.to_string())
    }

    pub fn sign_find_token(&self, last_id: Option<String>) -> Result<String, String> {
        self.0
            .sign_find_token(last_id)
            .map_err(|err| err.to_string())
    }

    pub fn public_key_hash(&self) -> String {
        self.0.public_key_hash()
    }
}
