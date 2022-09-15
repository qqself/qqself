use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};

// According to https://stackoverflow.com/a/64481383 nonce can be reused as long as we
// never reuse the same key. We always generate new AES key for every payload, so it's fine
// TODO On the other hand it's not a big deal to generate a new one every time. Do it eventually
const NONCE: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// AES encryption/decryption with ephemeral keys that are generated for each payload
pub(crate) struct AES {
    key: Vec<u8>,
    payload: Vec<u8>,
}

impl AES {
    pub(crate) fn encrypt<B: AsRef<[u8]>>(bytes: B) -> Option<Self> {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(&NONCE);
        let payload = cipher.encrypt(nonce, bytes.as_ref()).ok()?;
        Some(Self {
            key: Vec::from(key.as_slice()),
            payload,
        })
    }

    pub(crate) fn decrypt(key: &[u8], payload: &[u8]) -> Option<Vec<u8>> {
        let key = Aes256Gcm::new_from_slice(key).ok()?;
        let nonce = Nonce::from_slice(&NONCE);
        key.decrypt(nonce, payload).ok()
    }

    pub(crate) fn key(&self) -> &[u8] {
        &self.key
    }

    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let encrypted = AES::encrypt(vec![10, 20]).unwrap();
        let decrypted = AES::decrypt(encrypted.key(), encrypted.payload()).unwrap();
        assert_eq!(decrypted, vec![10, 20]);
    }
}
