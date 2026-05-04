mod db;
mod crypto;

use serde::{Serialize, Deserialize};
use crypto::{derive_key, decrypt, encrypt};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use std::sync::Mutex;

struct AppState {
    key: Mutex<Option<Vec<u8>>>,
}

#[derive(Serialize, Debug)]
struct Secret {
    id: String,
    title: String,
    username: Option<String>,
    secret_type: String,
    favorite: i32,
    encrypted_payload:  Option<String>,
}

#[derive(Serialize, Deserialize)]
struct NewSecret {
    id: String,
    title: String,
    username: Option<String>,
    secret_type: String,
    password: Option<String>,
}



#[tauri::command]
fn get_secrets() -> Result<Vec<Secret>, String> {
    let conn = db::connect().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, title, username, secret_type, favorite, encrypted_payload FROM secrets")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Secret {
                id: row.get(0)?,
                title: row.get(1)?,
                username: row.get(2)?,
                secret_type: row.get(3)?,
                favorite: row.get(4)?,
                encrypted_payload: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for item in rows {
        result.push(item.map_err(|e| e.to_string())?);
    }

    println!("Secrets from DB: {:?}", result);

    Ok(result)
}

#[tauri::command]
fn create_secret(
    payload: NewSecret,
    state: tauri::State<AppState>
) -> Result<(), String> {
    let key_guard = state.key.lock().unwrap();
    let key = key_guard.as_ref().ok_or("Vault locked")?;

    let conn = db::connect().map_err(|e| e.to_string())?;

    let secret_payload = serde_json::json!({
        "password": payload.password
    });

    let payload_json = serde_json::to_string(&secret_payload).unwrap();

    let encrypted_payload = encrypt(key, &payload_json);

    conn.execute(
        "INSERT INTO secrets (id, title, username, secret_type, encrypted_payload, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
        rusqlite::params![
            payload.id,
            payload.title,
            payload.username,
            payload.secret_type,
            encrypted_payload
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
fn get_secret_value(
    encrypted_payload: String,
    state: tauri::State<AppState>
) -> Result<String, String> {
    let key_guard = state.key.lock().unwrap();
    let key = key_guard.as_ref().ok_or("Vault locked")?;

    let decrypted = decrypt(key, &encrypted_payload);

    Ok(decrypted)
}


#[tauri::command]
fn setup_vault(password: String) -> Result<(), String> {
    let conn = db::connect().map_err(|e| e.to_string())?;

    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);

    let key = derive_key(&password, &salt);

    let hash = general_purpose::STANDARD.encode(&key);

    conn.execute(
        "INSERT INTO vault (password_hash, salt) VALUES (?1, ?2)",
        rusqlite::params![hash, salt],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn unlock_vault(
    password: String,
    state: tauri::State<AppState>
) -> Result<bool, String> {
    let conn = db::connect().map_err(|e| e.to_string())?;

    let (stored_hash, salt): (String, Vec<u8>) = conn
        .query_row(
            "SELECT password_hash, salt FROM vault LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    let key = derive_key(&password, &salt);
    let computed_hash = base64::engine::general_purpose::STANDARD.encode(&key);

    if computed_hash == stored_hash {
        let mut guard = state.key.lock().unwrap();
        *guard = Some(key);
        return Ok(true);
    }

    Ok(false)
}


#[tauri::command]
fn vault_exists() -> Result<bool, String> {
    let conn = db::connect().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM vault")
        .map_err(|e| e.to_string())?;

    let count: i64 = stmt
        .query_row([], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(count > 0)
}

#[tauri::command]
fn lock_vault(state: tauri::State<AppState>) -> Result<(), String> {
    let mut guard = state.key.lock().unwrap();
    *guard = None;
    Ok(())
}



pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_secrets,
            create_secret,
            get_secret_value,
            setup_vault,
            unlock_vault,
            vault_exists,
            lock_vault
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
