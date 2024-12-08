// db.rs
use rusqlite::{Connection, Result, params};
use uuid::Uuid;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn initialize_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_dir = app_handle.path().app_local_data_dir().unwrap();
    let db_path = app_dir.join("router_bits.db");

    let mut conn = Connection::open(&db_path)?;

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

    // Check if tables are empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM router_bits", [], |row| row.get(0))?;

    if count == 0 {
        insert_default_router_bits(&mut conn)?;
        insert_default_coordinates(&mut conn)?;
    }

    Ok(conn)
}
pub fn get_db(app_handle: &AppHandle) -> Result<Connection> {
    let app_dir = app_handle.path().app_local_data_dir().unwrap();
    let db_path = app_dir.join("router_bits.db");
    Connection::open(&db_path)
}


pub fn insert_default_router_bits(conn: &mut Connection) -> Result<()> {
    let defaults = vec![
        (
            "550e8400-e29b-41d4-a716-446655440000",
            "Straight Bit 1/4\"",
            "Straight",
            0.25,
            "General purpose straight cutting bit"
        ),
        (
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            "V-Groove 60°",
            "V-Groove",
            0.5,
            "For V-carving and chamfering"
        ),
        (
            "550e8400-e29b-41d4-a716-446655440001",
            "Ball Nose 1/8\"",
            "Ball Nose",
            0.125,
            "For 3D carving and surfacing"
        ),
        (
            "7f2c4a1b-8d5e-4c3f-9f6a-1d2b3e4f5a6b",
            "Downcut Spiral 3/8\"",
            "Downcut Spiral",
            0.375,
            "Downcut spiral for clean top surface and reduced tearout"
        ),
        (
            "9e8d7c6b-5a4f-3e2d-1c0b-9a8b7c6d5e4f",
            "Compression 3/8\"",
            "Compression",
            0.375,
            "Compression spiral for clean cuts on both top and bottom surfaces"
        ),
    ];

    let tx = conn.transaction()?;
    for (id, name, bit_type, diameter, description) in defaults {
        tx.execute(
            "INSERT OR IGNORE INTO router_bits (id, name, type, diameter, description)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, bit_type, diameter, description],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn insert_default_coordinates(conn: &mut Connection) -> Result<()> {
    let defaults = vec![
        ("550e8400-e29b-41d4-a716-446655440000", "Straight Bit 1/4\"", 3.8186, 3.5563, 5.555),
        ("6ba7b810-9dad-11d1-80b4-00c04fd430c8", "V-Groove 60°", 2.6744, 2.9678, -7.3963),
        ("550e8400-e29b-41d4-a716-446655440001", "Ball Nose 1/8\"", 4.0186, 3.7563, -4.2291),
        ("7f2c4a1b-8d5e-4c3f-9f6a-1d2b3e4f5a6b", "Downcut Spiral 3/8\"", 3.8186, 3.5563, -4.0291),
        ("9e8d7c6b-5a4f-3e2d-1c0b-9a8b7c6d5e4f", "Compression 3/8\"", 2.6744, 2.9678, -6.5069),
    ];

    let tx = conn.transaction()?;
    for (bit_id, name, x, y, z) in defaults {
        tx.execute(
            "INSERT OR IGNORE INTO bit_coordinates (bit_id, name, x, y, z) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![bit_id, name, x, y, z],
        )?;
    }
    tx.commit()?;
    Ok(())
}