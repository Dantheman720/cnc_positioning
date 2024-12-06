use rusqlite::Connection;
use tauri::Manager;

pub fn initialize_db(app_handle: &tauri::AppHandle) -> rusqlite::Result<Connection> {
    let app_dir = app_handle.path().app_local_data_dir().unwrap();
    let db_path = app_dir.join("router_bits.db");

    let conn = Connection::open(&db_path)?;

    // Create router bits table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS router_bits (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            diameter REAL NOT NULL,
            description TEXT
        )",
        [],
    )?;

    // Create coordinates table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bit_coordinates (
            bit_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            x REAL NOT NULL,
            y REAL NOT NULL,
            z REAL NOT NULL,
            FOREIGN KEY(bit_id) REFERENCES router_bits(id)
        )",
        [],
    )?;

    Ok(conn)
}

// Helper function to get database connection
pub fn get_db(app_handle: &tauri::AppHandle) -> rusqlite::Result<Connection> {
    let app_dir = app_handle.path().app_local_data_dir().unwrap();
    let db_path = app_dir.join("router_bits.db");
    Connection::open(&db_path)
}
