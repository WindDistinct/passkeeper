use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead};
use rand::RngCore;
use argon2::Argon2;
use base64::{
    engine::general_purpose,
    Engine as _
};

pub fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    let argon2 = Argon2::default();

    let mut key = vec![0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;

    Ok(key)
}

pub fn encrypt(key: &[u8], plaintext: &str) -> Result<String, String> {
    let cipher =
    Aes256Gcm::new_from_slice(key)
        .map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    Ok(general_purpose::STANDARD.encode(result))}

pub fn decrypt(key: &[u8], data: &str) -> Result<String, String> {
    let decoded =
        general_purpose::STANDARD.decode(data)
            .map_err(|e| e.to_string())?;

    if decoded.len() < 12 {
        return Err("Invalid encrypted payload".into());
    }

    let (nonce_bytes, ciphertext) = decoded.split_at(12);

    let cipher =
        Aes256Gcm::new_from_slice(key)
            .map_err(|e| e.to_string())?;

    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext)
        .map_err(|e| e.to_string())
}
