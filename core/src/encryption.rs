/*
From https://medium.com/b2w-engineering-en/sharing-aes-key-using-rsa-with-openssl-bc470afd2fb7:
- [x] Generate random AES encryption key
- [x] Encrypt payload using AES with generated key
- [x] Generate RSA public/private keys
- [x] Using RSA public key encrypt AES key
- [x] Concat RSA payload with AES payload
- [x] Split RSA encrypted payload to two parts: AES key and AES payload
- [x] Using RSA private key decrypt AES key
- [x] Using AES key decrypt AES payload

TODO: Error handling doesn't exists, we unwrap() everything and panic. Create proper error handling
TODO: Verify/Sign
TODO: More tests, prettify code
*/
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};

use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};

// According to https://stackoverflow.com/a/64481383 nonce can be reused as long as we
// never reuse the same key. We always generate new AES key for every payload, so it's fine
const NONCE: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const LINE_ENDING: LineEnding = LineEnding::LF;
const PADDING: PaddingScheme = PaddingScheme::PKCS1v15Encrypt;
const SEPARATOR: &str = ":";

pub struct EncryptionKeys {
    pub public: String,
    pub private: String,
}

pub fn generate_keys() -> EncryptionKeys {
    // TODO 4096 takes minutes with WebAssembly. Check if suggested "opt-level=3" compiler option would help
    let bits = 2048;
    let private = RsaPrivateKey::new(&mut OsRng, bits).unwrap();
    let public = private.to_public_key();
    let private = base64::encode(private.to_pkcs8_pem(LINE_ENDING).unwrap());
    let public = base64::encode(public.to_public_key_pem(LINE_ENDING).unwrap());
    EncryptionKeys { public, private }
}

pub fn encrypt(payload: String, public_key: String) -> String {
    let public_key = String::from_utf8(base64::decode(public_key).unwrap()).unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();
    let aes_payload = aes_encrypt(payload);
    let enc_aes_key = public_key
        .encrypt(&mut OsRng, PADDING, &aes_payload.key)
        .unwrap();
    let mut envelope = base64::encode(&enc_aes_key);
    envelope.push_str(SEPARATOR);
    envelope.push_str(&base64::encode(&aes_payload.payload));
    envelope
}

pub fn decrypt(payload: String, private_key: String) -> String {
    let private_key = String::from_utf8(base64::decode(private_key).unwrap()).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key).unwrap();
    let (key, payload) = payload.split_once(SEPARATOR).unwrap();
    let decoded_key = base64::decode(key).unwrap();
    let decoded_payload = base64::decode(payload).unwrap();
    let dec_data = private_key.decrypt(PADDING, &decoded_key).unwrap();
    aes_decrypt(dec_data, &decoded_payload[..])
}

struct AESPayload {
    key: Vec<u8>,
    payload: Vec<u8>,
}

fn aes_encrypt(payload: String) -> AESPayload {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&NONCE);
    let ciphertext = cipher.encrypt(nonce, payload.as_bytes()).unwrap();
    AESPayload {
        key: Vec::from(key.as_slice()),
        payload: ciphertext,
    }
}

fn aes_decrypt(key: Vec<u8>, payload: &[u8]) -> String {
    let key = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&NONCE);
    let plaintext = key.decrypt(nonce, payload.as_ref()).unwrap();
    String::from_utf8(plaintext).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypy_decrypt() {
        let keys = generate_keys();
        let message = "2022-08-18 22:20 23:20 qqself. Working on encryption\n";
        let encrypted = encrypt(message.to_string(), keys.public);
        let decrypted = decrypt(encrypted, keys.private);
        assert_eq!(message, decrypted);
    }

    #[test]
    fn encrypt_decrypt_repeat() {
        let public = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUF3eXdoQnZONmdUSWtaVjZ1c0tqcApGNmJEd0hXK1g2S3V2cUxzTTVuUHdHY1c0d3JFOVNudHVKd3NLNWFISUJLaVNBcVpFcDdVUnhFZWRwVmp2ZzZtCmZQRFlaQVlKaTJMZnIzc0RBRURsYVpIRTRHUXRaSVdZTnBIODFTL3l6aWNveTZiQjkyMnJNcEhnZk1IckVpREsKWmpwbitRUnphaVRwQytmaXZnVTN4ak1hM3Z2My9BWXRVNjVzRmdvVTB2aS9URE5PZ3lKbm9WekF3a29rVUdGOQpEZVVaWUpyTXY2M3JucWE0WTVtRlcyNmprU21mc2RXdjZuZy9GTC9PUU1uZWMydGhQa2VhOENObE5wc1ZxYk02ClpMQnQ0a05SOXdnOEtud1IvdDh0V2pPL2xjL1lhcVorZjdOY2g4SW9jcXFMSFRadmFqMjEwWnVXMU4vcllTbmgKOVFJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";
        let private = "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1JSUV2UUlCQURBTkJna3Foa2lHOXcwQkFRRUZBQVNDQktjd2dnU2pBZ0VBQW9JQkFRRERMQ0VHODNxQk1pUmwKWHE2d3FPa1hwc1BBZGI1Zm9xNitvdXd6bWMvQVp4YmpDc1QxS2UyNG5Dd3Jsb2NnRXFKSUNwa1NudFJIRVI1MgpsV08rRHFaODhOaGtCZ21MWXQrdmV3TUFRT1Zwa2NUZ1pDMWtoWmcya2Z6VkwvTE9KeWpMcHNIM2Jhc3lrZUI4Cndlc1NJTXBtT21mNUJITnFKT2tMNStLK0JUZkdNeHJlKy9mOEJpMVRybXdXQ2hUUytMOU1NMDZESW1laFhNREMKU2lSUVlYME41UmxnbXN5L3JldWVwcmhqbVlWYmJxT1JLWit4MWEvcWVEOFV2ODVBeWQ1emEyRStSNXJ3STJVMgpteFdwc3pwa3NHM2lRMUgzQ0R3cWZCSCszeTFhTTcrVno5aHFwbjUvczF5SHdpaHlxb3NkTm05cVBiWFJtNWJVCjMrdGhLZUgxQWdNQkFBRUNnZ0VBSDhMSnFTRHNPYVFZU2lyTFVOSmNCRTVPSXI3R2VUZTNLaXpITldwOVpYa2gKUDFkY1QrMC8wMEFPN0JUMTdtRTU2WEZZRWJDcEtOdC9VcHJpL1dYUzhCZ2Rqc0Z3d2k4WDZhMmlBbU9OTWMzWAo5UkE0VnRocXVjNnVEMTliS3ZNeWIxbTJ6UFlhMndMa1Bra3NZT25YSmlXeEQ5UUttQ1pqaWwxMUI2ay90MUFHClpvRGo1YXBIZWNFTXYwTmFmSVVac0VDemtDWGpidmNYYUNINngxRitBTndqT3VIOE9hYzFmdUh3RkZaWFVFN0MKcG5DdmV1OGttWnlkK08zbnp2MHV3MWN3UDRvYW5LSjBrOEpnTVVMZ1hNQjNLQVBEbWFMRHRKQ2Y4T1AxVmtYOApCSGhtbjB1a003NnRHUTN4cWV6dGs0eWJNbzhZcklmMXg0U0ZNSzVNUVFLQmdRRDlmMDlDcUlwejJMU24vbGZDCnZJTHhRRkViRlFma2h2eUMySUVJTzRQMmh3Yk5lQ1J1MFZackJrendFZTVYVnh5ZVlSVmxZZDVaRVdMaTRNS3UKZ3NtY1E5UUtaSzRzMS9VZjFKdUp4bFR2NGpUb29UYlZ2MnNrR015eHh4c01NbGQvSjRCY3FuYXN6MVZhK0QvMQpwamJNZHJyTmFLSXROMEtmN01EaE1Rc09FUUtCZ1FERkdXaWZ3ZVYxS2pia09lQWdOZ0xTYUJpOTZVM1JkR3pvCktweEYxMFBremh1c0JNSHp2MEtSUm9rWVJOY3YzNlFRNjd0MnZTc1VHWFRQSHJOTGRKOHJXSVRMVXA0bnVNT2sKS3lCMDdOOUZzYUZIMTBwSHRpRHdQU3dENHhWVUtIb0xuSnFLZU9uUDdXbGpQUUcyanZDNFg3Y3JFRkNXbXlYLwpQNkc1SlgvQnBRS0JnRzd3TFRiRjNibXZKL2l4TmFVU3JOZDV4MFRyNGtWZjBkaE5lY0twa1NudGs4dU4vWnZGCjMyVDlIK1NjWkVzeTQySm9xRjF6QkVURWVCdEVNODc5aTgvb1d6NmxLeGlTOWtLVTMvRGVyRU5ESXRaLzN5aVMKR2doMUZmZmpZL0hsZ2ljeW1WbDBmSFZLVTFkNWR6eEJIcEJDa2FQZFc4UWYvL2RQUEdZUStYNlJBb0dBWnJkcApLT2k1bGlmYjEzQzU5czl2QlF3dTZMbnZUaHJvcnMyV0IyZTZBRUhSZGdJOHViU1JFbmk1OWFYYXB4NWJ1RjNCCm9vM1BKM2hrd1pQMFNIeU1mdUJ3eEMxWU1zYzlYZzhEcVN2UTU5YzhmOTRZM2VweW5BQ2xNWmxrZ2lsZUFUTXAKN3NRNHhaMnFjSWRZbnpVN0Nqc0hrQysrYjk2SEhOQ2lqWVNYWWVrQ2dZRUFwVzM3SmFKWnBhejBlMndJSUt1dQoxalhPdm5aNjZBR0c5dTJOMXJRMUg3eEozRGJFQmtqaEFzaDIwdFlKM3Q2WkVrT1NZa0VJWE9tQm1EZFFGN3RqCktPWGdLL09FaFdZTHZXaFNJNndQUklYTStiN3JxNkxLS1o1SjFWZjhWcHVsKys1TTJtSkhJK3B2KzQwZGhsdk4KblVpVVp6RUkwcXdRT25pUElmS1hzVjg9Ci0tLS0tRU5EIFBSSVZBVEUgS0VZLS0tLS0K";
        let message = "2022-08-18 22:20 23:20 qqself. Working on encryption\n";
        let encrypted1 = encrypt(message.to_string(), public.to_string());
        let encrypted2 = encrypt(message.to_string(), public.to_string());
        // Ensure encrypted text is new every time
        assert_ne!(encrypted1, encrypted2);
        let decrypted1 = decrypt(encrypted1, private.to_string());
        let decrypted2 = decrypt(encrypted2, private.to_string());
        assert_eq!(decrypted1, decrypted2);
        assert_eq!(decrypted1, message);
    }
}
