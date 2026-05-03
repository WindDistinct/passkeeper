use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead};
use rand::RngCore;
use argon2::Argon2;
use base64::{encode, decode};

pub fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    let argon2 = Argon2::default();

    let mut key = vec![0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .unwrap();

    key
}

pub fn encrypt(key: &[u8], plaintext: &str) -> String {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .unwrap();

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    encode(result)
}

pub fn decrypt(key: &[u8], data: &str) -> String {
    let decoded = decode(data).unwrap();

    let (nonce_bytes, ciphertext) = decoded.split_at(12);

    let cipher = Aes256Gcm::new_from_slice(key).unwrap();

    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .unwrap();

    String::from_utf8(plaintext).unwrap()
}
