use crate::{binary_text::BinaryToText, date_time::timestamp::Timestamp};

use super::{
    hash::StableHash,
    keys::{PrivateKey, PublicKey},
    payload::usize_bytes,
    rsa::Rsa,
};
use thiserror::Error;

/// Signed search token for retrieving entries from backend services
#[derive(Debug)]
pub struct SearchToken {
    public_key: PublicKey,
    search_timestamp: Option<Timestamp>,
}

impl SearchToken {
    pub fn new_from_encoded(
        data: String,
        min_timestamp: Option<Timestamp>,
    ) -> Result<Self, TokenErr> {
        let token = Token::new_from_encoded(data, min_timestamp)?;
        Ok(Self {
            public_key: token.key,
            search_timestamp: token.payload,
        })
    }
    pub fn encode(
        public_key: &PublicKey,
        private_key: &PrivateKey,
        timestamp_created: Timestamp,
        timestamp_search: Option<Timestamp>,
    ) -> Result<String, TokenErr> {
        Token::encode(public_key, private_key, timestamp_created, timestamp_search)
    }
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
    pub fn search_timestamp(&self) -> &Option<Timestamp> {
        &self.search_timestamp
    }
}

/// Signed delete token for deleting all entires from backend services
#[derive(Debug)]
pub struct DeleteToken {
    public_key: PublicKey,
}

impl DeleteToken {
    pub fn new_from_encoded(data: String, min_timestamp: Option<Timestamp>) -> Result<Self, TokenErr> {
        // TODO It's not exactly clean to use Token here as DeleteToken doesn't have any payload
        let token = Token::new_from_encoded(data, min_timestamp)?;
        Ok(Self {
            public_key: token.key,
        })
    }
    pub fn encode(
        public_key: &PublicKey,
        private_key: &PrivateKey,
        timestamp_created: Timestamp,
    ) -> Result<String, TokenErr> {
        Token::encode(public_key, private_key, timestamp_created, None)
    }
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TokenErr {
    #[error("Search token validation error. {0}")]
    ValidationError(&'static str),
    #[error("Search token timestamp is too old")]
    TimestampIsTooOld,
}

/// Common Token functionality
struct Token {
    key: PublicKey,
    payload: Option<Timestamp>,
}

impl Token {
    const VERSION: u64 = 1;

    fn new_from_encoded(data: String, min_timestamp: Option<Timestamp>) -> Result<Self, TokenErr> {
        let encoded = BinaryToText::new_from_encoded(data)
            .ok_or(TokenErr::ValidationError("Failed to validate encoded data"))?;
        let decoded = encoded
            .decoded()
            .ok_or(TokenErr::ValidationError("Failed to decode data"))?;
        let bytes = TokenBinary::from_bytes(&decoded)
            .ok_or(TokenErr::ValidationError("Failed to read binary data"))?;
        if bytes.version != Token::VERSION {
            return Err(TokenErr::ValidationError("Cannot handle such version"));
        }
        // Timestamp if we check for it
        if let Some(min_timestamp) = min_timestamp {
            if bytes.timestamp_created < min_timestamp.as_u64() {
                return Err(TokenErr::TimestampIsTooOld);
            }
        }
        // Validate public key
        let public_key_str = std::str::from_utf8(bytes.public_key)
            .map_err(|_| TokenErr::ValidationError("Public key string error"))?;
        let public_key_data = BinaryToText::new_from_encoded(public_key_str.to_string())
            .ok_or(TokenErr::ValidationError("Public key encoding error"))?;
        let public_key = PublicKey::new(public_key_data)
            .map_err(|_| TokenErr::ValidationError("Public key validation error"))?;
        // Hash and RSA verify
        if Rsa::verify_signature(&public_key, bytes.signature, &bytes.hash).is_none() {
            return Err(TokenErr::ValidationError(
                "Payload signature validation failed",
            ));
        }
        let search_timestamp = if bytes.timestamp_search == 0 {
            None
        } else {
            Some(Timestamp::from_u64(bytes.timestamp_search))
        };
        Ok(Token {
            key: public_key,
            payload: search_timestamp,
        })
    }

    fn encode(
        public_key: &PublicKey,
        private_key: &PrivateKey,
        timestamp_created: Timestamp,
        timestamp_search: Option<Timestamp>,
    ) -> Result<String, TokenErr> {
        let data = TokenBinary::to_bytes(
            private_key,
            public_key,
            Token::VERSION,
            timestamp_created.as_u64(),
            timestamp_search.unwrap_or_default().as_u64(),
        )
        .ok_or(TokenErr::ValidationError("Failed encoding a search token"))?;
        Ok(BinaryToText::new(&data).encoded())
    }
}

/* Internal helper struct for binary payload reading/creating. Format:
[VERSION]            8 bytes
[TIMESTAMP_CREATED]  8 bytes
[TIMESTAMP_SEARCH]   8 bytes
[PUBLIC_KEY_LENGTH]  8 bytes
[PUBLIC_KEY]         Dynamic size
[SIGNATURE]          Dynamic size, rest of bytes
*/
struct TokenBinary<'a> {
    version: u64,
    timestamp_created: u64,
    timestamp_search: u64,
    hash: StableHash,
    public_key: &'a [u8],
    signature: &'a [u8],
}

// TODO There is a bit of repetition with PayloadBinary. Although formats are
//      different maybe we can extract some common functions from both of those?
impl<'a> TokenBinary<'a> {
    fn from_bytes(data: &'a [u8]) -> Option<Self> {
        // Read fixed size lengths first
        let (version, idx) = TokenBinary::read_u64(data, 0)?;
        let (timestamp_created, idx) = TokenBinary::read_u64(data, idx)?;
        let (timestamp_search, idx) = TokenBinary::read_u64(data, idx)?;
        let (public_key_len, idx) = TokenBinary::read_u64(data, idx)?;
        // Rest of the payload with dynamic sizes
        let (public_key, idx) = TokenBinary::read_bytes(data, idx, public_key_len)?;
        let all_but_signature = &data[..idx];
        let hash = StableHash::hash_bytes(all_but_signature);
        let signature = &data[idx..];
        if signature.is_empty() {
            return None;
        }
        Some(TokenBinary {
            version,
            timestamp_created,
            timestamp_search,
            public_key,
            signature,
            hash,
        })
    }

    fn to_bytes(
        private_key: &PrivateKey,
        public_key: &PublicKey,
        version: u64,
        timestamp_created: u64,
        timestamp_search: u64,
    ) -> Option<Vec<u8>> {
        let public_key_s = public_key.to_string();
        let public_key = public_key_s.as_bytes();
        let capacity = 8 * 4 + public_key.len() + Rsa::SIGNATURE_SIZE;
        let mut data = Vec::with_capacity(capacity);
        // Fixed sizes length
        data.extend_from_slice(&version.to_le_bytes());
        data.extend_from_slice(&timestamp_created.to_le_bytes());
        data.extend_from_slice(&timestamp_search.to_le_bytes());
        data.extend_from_slice(&usize_bytes(public_key.len()));
        // Dynamic sizes
        data.extend_from_slice(public_key);
        // Hash the payload, sign it and append the signature
        let digest = StableHash::hash_bytes(&data);
        let signature = Rsa::sign(private_key, &digest)?;
        data.extend_from_slice(&signature);
        Some(data)
    }

    fn read_u64(data: &'a [u8], idx: usize) -> Option<(u64, usize)> {
        let (data, idx) = TokenBinary::read_bytes(data, idx, 8)?;
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
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    const PUBLIC_KEY_1: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqo47AoLLabVv7Ycj6VoEZj1Gz9YPPDhcUjbkzgzLb5n799MydJYdRLA17wDAuvNTcJ4m27F2jzg7Zv26r94eYbRRrYH6oauQGPr9a6XyvNKTzykLkU9m5C3vEnpTVai2NMdib9JiEeJUMUSaApNd4r3ZF9i46suP7qD9gimj2USuh1QHY3r9YKmcyurkZRGZhjyXAnbae98vuJtUxVyMMzV9QWkV1BodGMFc4gE77HhULKk1Z23igQWJZsDTUDhiZdLxs5pmW1699zEgNt42PtJGxQ4ouL5UZcNv42UpUrrXsnKpAKLkRKZTfpsdp4zmPYfSjMNqPQLqiyDLw1B1b5Vs23pAYNMNJoBJXp3wMsJFngqPtPDWZ9Bgm5361uAZa2yNBBfaJMoumTjAPY54MWzYbeqj7mB7ZvLm1351SVJn8rNqrHAE6fNxbruJVwjzbKzbLmD859ZBd2F1V4SKRQZSAymj9sfJYYCn3Z6KoKzBSgH2QYXoTb93dVGDGqegfwZ9EYq";
    const PRIVATE_KEY_1: &str = "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";

    fn keys(public: &str, private: &str) -> (PublicKey, PrivateKey) {
        let public_key =
            PublicKey::new(BinaryToText::new_from_encoded(public.to_string()).unwrap()).unwrap();
        let private_key =
            PrivateKey::new(BinaryToText::new_from_encoded(private.to_string()).unwrap()).unwrap();
        (public_key, private_key)
    }

    #[test]
    #[wasm_bindgen_test]
    fn encode_decode() {
        let timestamp_search = Timestamp::from_u64(100);
        let timestamp_created = Timestamp::from_u64(200);
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encoded = SearchToken::encode(
            &public_key,
            &private_key,
            timestamp_created,
            Some(timestamp_search),
        )
        .unwrap();
        let decoded = SearchToken::new_from_encoded(encoded, None).unwrap();
        assert_eq!(decoded.public_key(), &public_key);
        assert_eq!(decoded.search_timestamp(), &Some(timestamp_search));
    }

    #[test]
    #[wasm_bindgen_test]
    fn validation() {
        // Decoding issues
        let decoded = SearchToken::new_from_encoded("AAABBCBABCBABCBA".to_string(), None);
        assert_eq!(
            decoded.unwrap_err(),
            TokenErr::ValidationError("Failed to read binary data")
        );

        // Too old token
        let timestamp_search = Timestamp::from_u64(100);
        let timestamp_created = Timestamp::from_u64(200);
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encoded = SearchToken::encode(
            &public_key,
            &private_key,
            timestamp_created,
            Some(timestamp_search),
        )
        .unwrap();
        let decoded =
            SearchToken::new_from_encoded(encoded.clone(), Some(Timestamp::from_u64(300)));
        assert_eq!(decoded.unwrap_err(), TokenErr::TimestampIsTooOld);

        // Broken signature
        let mut bad_data = encoded;
        let bad_symbol = if bad_data.ends_with('1') { "2" } else { "1" };
        bad_data.replace_range(bad_data.len() - 1..bad_data.len(), bad_symbol);
        let decoded = SearchToken::new_from_encoded(bad_data, None);
        assert_eq!(
            decoded.unwrap_err(),
            TokenErr::ValidationError("Payload signature validation failed")
        )
    }
}
