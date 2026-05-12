use rusqlite::{Connection, Result};

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("passkeeper.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            username TEXT,
            secret_type TEXT NOT NULL,
            favorite INTEGER NOT NULL DEFAULT 0,
            encrypted_payload TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            deleted_at TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vault (
            id INTEGER PRIMARY KEY,
            password_hash TEXT NOT NULL,
            salt BLOB NOT NULL
        )",
        [],
    )?;    

    Ok(conn)
}

pub fn get_encrypted_payload_by_id(
    conn: &Connection,
    secret_id: &str,
) -> Result<String> {
    conn.query_row(
        "SELECT encrypted_payload
         FROM secrets
         WHERE id = ?1
         AND deleted_at IS NULL",
        [secret_id],
        |row| row.get(0),
    )
}