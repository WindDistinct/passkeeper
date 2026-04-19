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
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
