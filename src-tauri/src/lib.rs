mod db;

use rusqlite::params;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Secret {
    id: String,
    title: String,
    username: Option<String>,
    secret_type: String,
    favorite: i32,
}

#[derive(Deserialize)]
struct NewSecret {
    id: String,
    title: String,
    username: Option<String>,
    secret_type: String,
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

    conn.execute(
        "INSERT INTO secrets (id, title, username, secret_type, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![
            payload.id,
            payload.title,
            payload.username,
            payload.secret_type
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_secrets,
            create_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
