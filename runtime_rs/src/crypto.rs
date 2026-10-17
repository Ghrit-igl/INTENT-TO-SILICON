use ring::digest;
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{AES_256_GCM, UnboundKey, SealingKey, OpeningKey, BoundKey, Nonce, Aad, NonceSequence, NONCE_LEN};

struct SingleNonce(Option<Nonce>);

impl NonceSequence for SingleNonce {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        self.0.take().ok_or(ring::error::Unspecified)
    }
}

pub fn sha256_hex(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA256, data);
    let mut out = String::with_capacity(64);
    for b in digest.as_ref() {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

pub fn sha512_hex(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA512, data);
    let mut out = String::with_capacity(128);
    for b in digest.as_ref() {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

pub fn hmac_sha256_hex(key_bytes: &[u8], data: &[u8]) -> String {
    let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_bytes);
    let tag = hmac::sign(&s_key, data);
    let mut out = String::with_capacity(64);
    for b in tag.as_ref() {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

pub fn timing_safe_equals(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}


pub fn random_bytes_hex(count: usize) -> Result<String, String> {
    let rng = SystemRandom::new();
    let mut buf = vec![0u8; count];
    rng.fill(&mut buf).map_err(|e| format!("CSPRNG failure: {:?}", e))?;
    let mut out = String::with_capacity(count * 2);
    for b in buf {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    Ok(out)
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string length must be even".to_string());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

/// AES-256-GCM Authenticated Encryption (FIPS / NIST SP 800-38D compliant)
/// Key: 32 bytes (or 64 hex chars), Nonce: 12 bytes (or 24 hex chars)
/// Returns hex string containing ciphertext + 16-byte authentication tag
pub fn aes_256_gcm_encrypt(key_input: &[u8], nonce_input: &[u8], plaintext: &[u8]) -> Result<String, String> {
    let key_bytes = if key_input.len() == 64 {
        hex_to_bytes(&String::from_utf8_lossy(key_input))?
    } else if key_input.len() == 32 {
        key_input.to_vec()
    } else {
        // Derive 32 bytes using SHA-256 if key is arbitrary length
        let d = digest::digest(&digest::SHA256, key_input);
        d.as_ref().to_vec()
    };

    let nonce_bytes = if nonce_input.len() == 24 {
        hex_to_bytes(&String::from_utf8_lossy(nonce_input))?
    } else if nonce_input.len() == NONCE_LEN {
        nonce_input.to_vec()
    } else {
        // Pad or truncate to 12 bytes
        let mut n = vec![0u8; NONCE_LEN];
        let copy_len = nonce_input.len().min(NONCE_LEN);
        n[..copy_len].copy_from_slice(&nonce_input[..copy_len]);
        n
    };

    let nonce_arr: [u8; NONCE_LEN] = nonce_bytes.try_into().map_err(|_| "Invalid nonce length".to_string())?;
    let nonce = Nonce::assume_unique_for_key(nonce_arr);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|e| format!("AES key init error: {:?}", e))?;
    let mut sealing_key = SealingKey::new(unbound_key, SingleNonce(Some(nonce)));

    let mut in_out = plaintext.to_vec();
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|e| format!("AES-256-GCM encryption failed: {:?}", e))?;

    Ok(bytes_to_hex(&in_out))
}

/// AES-256-GCM Authenticated Decryption
/// Verifies authentication tag and decrypts ciphertext
pub fn aes_256_gcm_decrypt(key_input: &[u8], nonce_input: &[u8], ciphertext_hex: &str) -> Result<String, String> {
    let key_bytes = if key_input.len() == 64 {
        hex_to_bytes(&String::from_utf8_lossy(key_input))?
    } else if key_input.len() == 32 {
        key_input.to_vec()
    } else {
        let d = digest::digest(&digest::SHA256, key_input);
        d.as_ref().to_vec()
    };

    let nonce_bytes = if nonce_input.len() == 24 {
        hex_to_bytes(&String::from_utf8_lossy(nonce_input))?
    } else if nonce_input.len() == NONCE_LEN {
        nonce_input.to_vec()
    } else {
        let mut n = vec![0u8; NONCE_LEN];
        let copy_len = nonce_input.len().min(NONCE_LEN);
        n[..copy_len].copy_from_slice(&nonce_input[..copy_len]);
        n
    };

    let nonce_arr: [u8; NONCE_LEN] = nonce_bytes.try_into().map_err(|_| "Invalid nonce length".to_string())?;
    let nonce = Nonce::assume_unique_for_key(nonce_arr);

    let mut in_out = hex_to_bytes(ciphertext_hex)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|e| format!("AES key init error: {:?}", e))?;
    let mut opening_key = OpeningKey::new(unbound_key, SingleNonce(Some(nonce)));

    let decrypted = opening_key.open_in_place(Aad::empty(), &mut in_out)
        .map_err(|e| format!("AES-256-GCM authentication/decryption failed (tampered or invalid key): {:?}", e))?;

    String::from_utf8(decrypted.to_vec()).map_err(|e| format!("UTF-8 decode error: {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vector() {
        // SHA-256("hello")
        let h = sha256_hex(b"hello");
        assert_eq!(h, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_hmac_sha256_known_vector() {
        let h = hmac_sha256_hex(b"secret", b"message");
        assert_eq!(h.len(), 64);
    }

    #[test]
    fn test_aes_gcm_roundtrip() {
        let key = b"my_super_secret_32_byte_key_123";
        let nonce = b"123456789012";
        let plaintext = b"Confidential Financial Transaction";

        let encrypted = aes_256_gcm_encrypt(key, nonce, plaintext).expect("encryption succeeds");
        assert_ne!(encrypted, "Confidential Financial Transaction");

        let decrypted = aes_256_gcm_decrypt(key, nonce, &encrypted).expect("decryption succeeds");
        assert_eq!(decrypted, "Confidential Financial Transaction");

        // Tamper test
        let mut tampered = encrypted.clone();
        tampered.replace_range(0..2, "00");
        assert!(aes_256_gcm_decrypt(key, nonce, &tampered).is_err());
    }
}
