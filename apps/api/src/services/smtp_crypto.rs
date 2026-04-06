//! AES-256-GCM encryption for SMTP passwords at rest (key derived from `PE_JWT_SECRET`).

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use rand::RngCore as _;
use sha2::{Digest, Sha256};

use crate::errors::ApiError;

fn derive_key(jwt_secret: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"pe:smtp:pw:v1");
    hasher.update(jwt_secret.as_bytes());
    let out = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&out[..32]);
    key
}

/// Encrypt `plain` for storage in `smtp_settings.password_encrypted`.
pub fn encrypt_password(jwt_secret: &str, plain: &str) -> Result<String, ApiError> {
    let key = derive_key(jwt_secret);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| ApiError::Internal)?;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plain.as_bytes())
        .map_err(|_| ApiError::Internal)?;
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    Ok(base64::engine::general_purpose::STANDARD.encode(combined))
}

#[must_use]
pub fn decrypt_password(jwt_secret: &str, encoded: &str) -> Option<String> {
    let raw = base64::engine::general_purpose::STANDARD
        .decode(encoded.as_bytes())
        .ok()?;
    if raw.len() < 12 {
        return None;
    }
    let (nonce_bytes, ct) = raw.split_at(12);
    let key = derive_key(jwt_secret);
    let cipher = Aes256Gcm::new_from_slice(&key).ok()?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plain = cipher.decrypt(nonce, ct).ok()?;
    String::from_utf8(plain).ok()
}
