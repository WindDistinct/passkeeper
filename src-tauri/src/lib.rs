mod db;
mod crypto;

use rusqlite::params;
use serde::{Serialize, Deserialize};
use crypto::{derive_key, decrypt, encrypt};

#[derive(Serialize)]
struct Secret {
    id: String,
    title: String,
    username: Option<String>,
    secret_type: String,
    favorite: i32,
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
        .prepare("SELECT id, title, username, secret_type, favorite FROM secrets")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Secret {
                id: row.get(0)?,
                title: row.get(1)?,
                username: row.get(2)?,
                secret_type: row.get(3)?,
                favorite: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for item in rows {
        result.push(item.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[tauri::command]
fn create_secret(payload: NewSecret) -> Result<(), String> {
    let conn = db::connect().map_err(|e| e.to_string())?;

    let master_password = "dev_master_key";
    let salt = b"fixed_salt";

    let key = derive_key(master_password, salt);

    let secret_payload = serde_json::json!({
        "password": payload.password
    });
    
    let payload_json = serde_json::to_string(&secret_payload).unwrap();
    
    let encrypted_payload = encrypt(&key, &payload_json);

    conn.execute(
        "INSERT INTO secrets (id, title, username, secret_type, encrypted_payload, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
        params![
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
fn get_secret_value(encrypted_payload: String) -> Result<String, String> {
    let master_password = "dev_master_key";
    let salt = b"fixed_salt";

    let key = derive_key(master_password, salt);

    let decrypted = decrypt(&key, &encrypted_payload);

    Ok(decrypted)
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_secrets,
            create_secret,
            get_secret_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
