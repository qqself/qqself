use crate::{binary_text::BinaryToText, datetime::Timestamp};

use super::{
    aes::Aes,
    hash::StableHash,
    keys::{PrivateKey, PublicKey},
    rsa::Rsa,
};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PayloadError {
    #[error("{0}")]
    TooBig(String),
    #[error("{0}")]
    EncryptionError(&'static str),
    #[error("{0}")]
    DecryptionError(&'static str),
    #[error("{0}")]
    ValidationError(&'static str),
    #[error("Payload timestamp is too old")]
    TimestampIsTooOld,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PayloadId {
    timestamp: Timestamp,
    hash: StableHash,
}

impl PayloadId {
    pub fn new(timestamp: Timestamp, hash: StableHash) -> Self {
        Self { timestamp, hash }
    }
    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    pub fn hash(&self) -> &StableHash {
        &self.hash
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Plaintext {
    id: PayloadId,
    text: String,
}

impl Plaintext {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn id(&self) -> &PayloadId {
        &self.id
    }
}

#[derive(Debug)]
pub struct Payload {
    id: PayloadId,
    data: PayloadBytes,
    public_key: PublicKey,
    previous_version: Option<PayloadId>,
}

impl Payload {
    // Decrypt the payload and get clear text payload
    pub fn decrypt(&self, private_key: &PrivateKey) -> Result<Plaintext, PayloadError> {
        let encoded = self
            .data
            .0
            .decoded()
            .ok_or(PayloadError::ValidationError("Cannot decode the payload"))?;
        let bytes = PayloadBinary::from_bytes(&encoded)
            .ok_or(PayloadError::ValidationError("Cannot read binary data"))?;
        // Decrypt AES key using RSA private key
        let aes_key = Rsa::decrypt(private_key, bytes.aes_key)
            .ok_or(PayloadError::DecryptionError("Failed to decrypt AES key"))?;
        // Now decrypt the payload with AES key
        let data = Aes::decrypt(&aes_key, bytes.aes_payload).ok_or(
            PayloadError::DecryptionError("Failed to decrypt the payload"),
        )?;
        let text = String::from_utf8(data).map_err(|_| {
            PayloadError::DecryptionError("Decrypted payload is not valid UTF8 string")
        })?;
        Ok(Plaintext {
            text,
            id: PayloadId {
                timestamp: bytes.timestamp,
                hash: bytes.hash,
            },
        })
    }

    pub fn id(&self) -> &PayloadId {
        &self.id
    }

    pub fn data(&self) -> PayloadBytes {
        self.data.clone()
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn previous_version(&self) -> &Option<PayloadId> {
        &self.previous_version
    }
}

// Raw payload bytes
#[derive(Debug, Clone)]
pub struct PayloadBytes(BinaryToText);

impl PayloadBytes {
    const MAX_SIZE: usize = 4096;
    const VERSION: u64 = 1;

    // Create new payload from already encrypted data. Fails in case payload it too big
    pub fn new_from_encrypted(data: BinaryToText) -> Result<Self, PayloadError> {
        if data.len() > PayloadBytes::MAX_SIZE {
            return Err(PayloadError::TooBig(format!(
                "Encoded payload of size {} is bigger than max {}",
                data.len(),
                PayloadBytes::MAX_SIZE
            )));
        }
        Ok(Self(data))
    }

    // Creates new encrypted payload
    pub fn encrypt(
        public_key: &PublicKey,
        private_key: &PrivateKey,
        timestamp: Timestamp,
        plaintext: &str,
        previous_version: Option<PayloadId>,
    ) -> Result<Self, PayloadError> {
        // Generate new ephemeral AES key and encrypt the payload with it
        let aes_payload = Aes::encrypt(plaintext.as_bytes()).ok_or(
            PayloadError::EncryptionError("AES encryption of payload failed"),
        )?;
        // Encrypt AES key using RSA public key
        let encrypted_aes_key = Rsa::encrypt(public_key, aes_payload.key()).ok_or(
            PayloadError::EncryptionError("RSA encryption of AES key failed"),
        )?;
        let bytes = PayloadBinary::to_bytes(
            private_key,
            public_key,
            PayloadBytes::VERSION,
            timestamp.as_u64(),
            &encrypted_aes_key,
            aes_payload.payload(),
            previous_version,
        )
        .ok_or(PayloadError::EncryptionError("Failed to sign the data"))?;
        PayloadBytes::new_from_encrypted(BinaryToText::new(&bytes))
    }

    // Do full validation of the payload, including RSA signature validation
    pub fn validated(self, min_timestamp: Option<Timestamp>) -> Result<Payload, PayloadError> {
        // Encoding and data reading
        let encoded = self
            .0
            .decoded()
            .ok_or(PayloadError::ValidationError("Cannot decode the payload"))?;
        let bytes = PayloadBinary::from_bytes(&encoded)
            .ok_or(PayloadError::ValidationError("Cannot read binary data"))?;
        // Version check
        if bytes.version != PayloadBytes::VERSION {
            return Err(PayloadError::ValidationError("Cannot handle such version"));
        }
        // Timestamp if we check for it
        if let Some(min_timestamp) = min_timestamp {
            if bytes.timestamp < min_timestamp {
                return Err(PayloadError::TimestampIsTooOld);
            }
        }
        // Validate public key
        let public_key_str = std::str::from_utf8(bytes.public_key)
            .map_err(|_| PayloadError::ValidationError("Public key string error"))?;
        let public_key_data = BinaryToText::new_from_encoded(public_key_str.to_string())
            .ok_or(PayloadError::ValidationError("Public key encoding error"))?;
        let public_key = PublicKey::new(public_key_data)
            .map_err(|_| PayloadError::ValidationError("Public key validation error"))?;
        // Hash and RSA verify
        if Rsa::verify_signature(&public_key, bytes.signature, &bytes.hash).is_none() {
            return Err(PayloadError::ValidationError(
                "Payload signature validation failed",
            ));
        }
        Ok(Payload {
            id: PayloadId::new(bytes.timestamp, bytes.hash),
            data: self,
            public_key,
            previous_version: bytes.previous,
        })
    }

    pub fn data(self) -> String {
        self.0.encoded()
    }
}

/*  Internal helper struct for binary payload reading/creating. Format:
[VERSION]                         8 bytes
[CONTAINS_PREVIOUS]               1 byte
  [PREVIOUS_TIMESTAMP]               8 bytes if [CONTAINS_PREVIOUS]
  [PREVIOUS_HASH]                   16 bytes if [CONTAINS_PREVIOUS]
[TIMESTAMP]                       8 bytes
[PUBLIC_KEY_LENGTH]               8 bytes
[ENCRYPTED_AES_KEY_LENGTH]        8 bytes
[AES_ENCRYPTED_PAYLOAD_LENGTH]    8 bytes
[PUBLIC_KEY]                      Dynamic size
[ENCRYPTED_AES_KEY]               Dynamic size
[AES_ENCRYPTED_PAYLOAD]           Dynamic size
[SIGNATURE]                       Dynamic size, rest of bytes */
struct PayloadBinary<'a> {
    version: u64,
    previous: Option<PayloadId>,
    timestamp: Timestamp,
    hash: StableHash,
    public_key: &'a [u8],
    aes_key: &'a [u8],
    aes_payload: &'a [u8],
    signature: &'a [u8],
}

impl<'a> PayloadBinary<'a> {
    fn from_bytes(data: &'a [u8]) -> Option<Self> {
        // Read fixed size lengths first
        let (version, idx) = PayloadBinary::read_u64(data, 0)?;
        let (contains_previous, idx) = PayloadBinary::read_byte(data, idx)?;
        let (previous, idx) = match contains_previous {
            0 => (None, idx),
            _ => {
                // contains previous flag is set, read previous PayloadId information
                let (timestamp, idx) = PayloadBinary::read_u64(data, idx)?;
                let (hash_bytes, idx) = PayloadBinary::read_bytes(data, idx, 16)?;
                let hash = StableHash::new_from_bytes(hash_bytes.try_into().ok()?);
                (Some(PayloadId::new(Timestamp::new(timestamp), hash)), idx)
            }
        };
        let (timestamp, idx) = PayloadBinary::read_u64(data, idx)?;
        let timestamp = Timestamp::new(timestamp);
        let (public_key_len, idx) = PayloadBinary::read_u64(data, idx)?;
        let (aes_key_len, idx) = PayloadBinary::read_u64(data, idx)?;
        let (aes_payload_len, idx) = PayloadBinary::read_u64(data, idx)?;
        // Rest of the payload with dynamic sizes
        let (public_key, idx) = PayloadBinary::read_bytes(data, idx, public_key_len)?;
        let (aes_key, idx) = PayloadBinary::read_bytes(data, idx, aes_key_len)?;
        let (aes_payload, idx) = PayloadBinary::read_bytes(data, idx, aes_payload_len)?;
        let all_but_signature = &data[..idx];
        let hash = StableHash::hash_bytes(all_but_signature);
        let signature = &data[idx..];
        if signature.is_empty() {
            return None;
        }
        Some(PayloadBinary {
            version,
            previous,
            timestamp,
            hash,
            public_key,
            aes_key,
            aes_payload,
            signature,
        })
    }

    fn to_bytes(
        private_key: &PrivateKey,
        public_key: &PublicKey,
        version: u64,
        timestamp: u64,
        aes_key: &'a [u8],
        aes_payload: &'a [u8],
        previous: Option<PayloadId>,
    ) -> Option<Vec<u8>> {
        let public_key_s = public_key.to_string();
        let public_key = public_key_s.as_bytes();
        let capacity =
            16 + 8 * 5 + public_key.len() + aes_key.len() + aes_payload.len() + Rsa::SIGNATURE_SIZE;
        let mut data = Vec::with_capacity(capacity);
        // Fixed sizes length
        data.extend_from_slice(&version.to_le_bytes());
        // If previous version is set
        match previous {
            Some(id) => {
                data.push(0x01);
                data.extend_from_slice(&id.timestamp.as_u64().to_le_bytes());
                data.extend_from_slice(&id.hash.as_bytes());
            }
            None => data.push(0x00),
        }
        data.extend_from_slice(&timestamp.to_le_bytes());
        data.extend_from_slice(&usize_bytes(public_key.len()));
        data.extend_from_slice(&usize_bytes(aes_key.len()));
        data.extend_from_slice(&usize_bytes(aes_payload.len()));
        // Dynamic sizes
        data.extend_from_slice(public_key);
        data.extend_from_slice(aes_key);
        data.extend_from_slice(aes_payload);
        // Hash the payload, sign it and append the signature
        let digest = StableHash::hash_bytes(&data);
        let signature = Rsa::sign(private_key, &digest)?;
        data.extend_from_slice(&signature);
        Some(data)
    }

    fn read_u64(data: &'a [u8], idx: usize) -> Option<(u64, usize)> {
        let (data, idx) = PayloadBinary::read_bytes(data, idx, 8)?;
        let u64_bytes: [u8; 8] = data.try_into().expect("read 8 bytes");
        Some((u64::from_le_bytes(u64_bytes), idx))
    }

    fn read_bytes(data: &'a [u8], idx: usize, len: u64) -> Option<(&'a [u8], usize)> {
        let len = len as usize; // Safe conversion as len will never reach very big values
        if data.len() < idx {
            return None;
        }
        let data = &data[idx..];
        if data.len() < len {
            return None;
        }
        Some((&data[..len], idx + len))
    }

    fn read_byte(data: &'a [u8], idx: usize) -> Option<(u8, usize)> {
        let (read, idx) = PayloadBinary::read_bytes(data, idx, 1)?;
        Some((read[0], idx))
    }
}

// usize size depends on a target, most are 64 bit, but WebAssembly is 32 bit
// We have to be extra careful with usize bytes to ensure encoded data can be read everywhere
pub(crate) fn usize_bytes(v: usize) -> [u8; 8] {
    let v: u64 = v
        .try_into()
        .expect("only 32 and 64 bit systems are supported");
    v.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    const PUBLIC_KEY_1: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqo47AoLLabVv7Ycj6VoEZj1Gz9YPPDhcUjbkzgzLb5n799MydJYdRLA17wDAuvNTcJ4m27F2jzg7Zv26r94eYbRRrYH6oauQGPr9a6XyvNKTzykLkU9m5C3vEnpTVai2NMdib9JiEeJUMUSaApNd4r3ZF9i46suP7qD9gimj2USuh1QHY3r9YKmcyurkZRGZhjyXAnbae98vuJtUxVyMMzV9QWkV1BodGMFc4gE77HhULKk1Z23igQWJZsDTUDhiZdLxs5pmW1699zEgNt42PtJGxQ4ouL5UZcNv42UpUrrXsnKpAKLkRKZTfpsdp4zmPYfSjMNqPQLqiyDLw1B1b5Vs23pAYNMNJoBJXp3wMsJFngqPtPDWZ9Bgm5361uAZa2yNBBfaJMoumTjAPY54MWzYbeqj7mB7ZvLm1351SVJn8rNqrHAE6fNxbruJVwjzbKzbLmD859ZBd2F1V4SKRQZSAymj9sfJYYCn3Z6KoKzBSgH2QYXoTb93dVGDGqegfwZ9EYq";
    const PRIVATE_KEY_1: &str = "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";
    const PUBLIC_KEY_2: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqEKPRmocEuhWJthsJJ44rdBUxeWKdcUfKFdThfJ5N9ZL41CYjJWQahFqkf6KcZcKMrzLwuvvNL3PQJ7Ly688RrUdYrY8NozzkiK1rUcBZCWAcXvavpDkxh46j7si82S1oZmpkfVT77nkwdHhUYNMmSxwnjQj3iTDqVYJqext2ZM11TMsr5FSfQrr1WUv4ZLyjMkE7uVHsJih3kB4dkfg5LYJfUfnGw9fDfRyCk6YFQ7t2HxTJRxigMxACSJpbCCgza2iA1LdEA8rYCKnLWsCeXMZC4fezu7Vhb2ivuYYRc16vkcJKq4xKxhp6mXWP5jrY9Y2dMWrMrBVeiE8ac1Z6D1EdvanJUzHisyRXg3K2DKmrvzKer7f67pXjHYUnLh9s3owzULxGsCZ88BLmMgrw5JrTvbYoRhACaUksFTUhJxAHJAysVDVHSr4PEhd6oqzPoM4TqFLYmaa5aYGBL8NgqJarvWvM59finsXpVvi2zcZ7hbFGHE2vLjqZpyjeiJNBc6DyEh";
    const PRIVATE_KEY_2: &str = "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";
    const TIMESTAMP: u64 = 1662750865;

    fn keys(public: &str, private: &str) -> (PublicKey, PrivateKey) {
        let public_key =
            PublicKey::new(BinaryToText::new_from_encoded(public.to_string()).unwrap()).unwrap();
        let private_key =
            PrivateKey::new(BinaryToText::new_from_encoded(private.to_string()).unwrap()).unwrap();
        (public_key, private_key)
    }

    #[test]
    #[wasm_bindgen_test]
    fn binary_data() {
        // No previous
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let previous_id =
            PayloadId::new(Timestamp::new(TIMESTAMP), StableHash::hash_string("entry"));

        let bytes = PayloadBinary::to_bytes(
            &private_key,
            &public_key,
            PayloadBytes::VERSION,
            TIMESTAMP,
            &[3; 30],
            &[4; 40],
            Some(previous_id.clone()),
        )
        .unwrap();
        let data = PayloadBinary::from_bytes(&bytes).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.timestamp, Timestamp::new(TIMESTAMP));
        assert_eq!(data.public_key, public_key.to_string().as_bytes());
        assert_eq!(data.aes_key, &vec![3; 30]);
        assert_eq!(data.aes_payload, &vec![4; 40]);
        assert_eq!(data.signature.len(), Rsa::SIGNATURE_SIZE);
        assert_eq!(data.previous, Some(previous_id));
        assert!(PayloadBinary::from_bytes(&[1; 100]).is_none())
    }

    #[test]
    #[wasm_bindgen_test]
    fn encrypt_decrypt() {
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let data = "payload";
        let encrypted = PayloadBytes::encrypt(
            &public_key,
            &private_key,
            Timestamp::new(TIMESTAMP),
            data,
            None,
        )
        .unwrap();
        let payload = encrypted.validated(None).unwrap();
        assert_eq!(payload.public_key(), &public_key);
        let decrypted = payload.decrypt(&private_key).unwrap();
        assert_eq!(decrypted.text, data.to_string());
        assert_eq!(decrypted.id().timestamp(), &Timestamp::new(TIMESTAMP));

        // Can't decrypt with another key
        let (_, private_key2) = keys(PUBLIC_KEY_2, PRIVATE_KEY_2);
        assert_eq!(
            payload.decrypt(&private_key2),
            Err(PayloadError::DecryptionError("Failed to decrypt AES key"))
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn validation() {
        let (public_key1, private_key1) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let (_, _) = keys(PUBLIC_KEY_2, PRIVATE_KEY_2);
        // Decoding errors
        let payload = PayloadBytes::new_from_encrypted(BinaryToText::new(&[10, 20])).unwrap();
        assert_eq!(
            payload.validated(None).unwrap_err(),
            PayloadError::ValidationError("Cannot read binary data")
        );
        // Timestamp check
        let data = "payload";
        let payload = PayloadBytes::encrypt(
            &public_key1,
            &private_key1,
            Timestamp::new(TIMESTAMP),
            data,
            None,
        )
        .unwrap();
        assert_eq!(
            payload
                .validated(Some(Timestamp::new(u64::MAX)))
                .unwrap_err(),
            PayloadError::TimestampIsTooOld
        );
        // Signature check. Modify the encrypted payload
        let payload = PayloadBytes::encrypt(
            &public_key1,
            &private_key1,
            Timestamp::new(TIMESTAMP),
            data,
            None,
        )
        .unwrap();
        let mut bad_payload = payload.0.encoded();
        let bad_symbol = if bad_payload.ends_with('1') { "2" } else { "1" };
        bad_payload.replace_range(bad_payload.len() - 1..bad_payload.len(), bad_symbol);
        let encrypted_bad =
            PayloadBytes::new_from_encrypted(BinaryToText::new_from_encoded(bad_payload).unwrap())
                .unwrap();
        assert_eq!(
            encrypted_bad.validated(None).unwrap_err(),
            PayloadError::ValidationError("Payload signature validation failed")
        );
    }
}
