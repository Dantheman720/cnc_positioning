// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use rusqlite::{params, Result};

mod db;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouterBit {
    pub id: Uuid,
    pub name: String,
    pub r#type: String,
    pub diameter: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRouterBitRequest {
    pub name: String,
    pub r#type: String,
    pub diameter: f64,
    pub description: String,
}

#[tauri::command]
async fn create_router_bit(app_handle: AppHandle, data: &str) -> Result<RouterBit, String> {
    let request: CreateRouterBitRequest = serde_json::from_str(data)
        .map_err(|e| format!("Failed to parse request: {}", e))?;

    let new_bit = RouterBit {
        id: Uuid::new_v4(),
        name: request.name,
        r#type: request.r#type,
        diameter: request.diameter,
        description: request.description,
    };

    let conn = db::get_db(&app_handle)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    conn.execute(
        "INSERT INTO router_bits (id, name, type, diameter, description) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            new_bit.id.to_string(),
            new_bit.name,
            new_bit.r#type,
            new_bit.diameter,
            new_bit.description
        ],
    )
        .map_err(|e| format!("Failed to insert router bit: {}", e))?;

    Ok(new_bit)
}
#[tauri::command]
async fn get_router_bits(app_handle: AppHandle) -> Result<Vec<RouterBit>, String> {
    let conn = db::get_db(&app_handle)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let mut stmt = conn.prepare("SELECT id, name, type, diameter, description FROM router_bits")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let bits = stmt.query_map([], |row| {
        Ok(RouterBit {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            name: row.get(1)?,
            r#type: row.get(2)?,
            diameter: row.get(3)?,
            description: row.get(4)?,
        })
    })
        .map_err(|e| format!("Failed to query router bits: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect router bits: {}", e))?;

    Ok(bits)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub router_bit: RouterBit,
    pub plywood_thickness: f64,
    pub calculate_workpiece_zero: bool,
    pub calculate_workpiece_height: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct BitCoordinates {
    bit_id: String,
    name: String,
    x: f64,
    y: f64,
    z: f64,
}

impl BitCoordinates {
    pub fn get_bit_id(&self) -> &str {
        &self.bit_id
    }

    pub fn set_bit_id(&mut self, bit_id: String) {
        self.bit_id = bit_id;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}

fn find_coordinates(bit_id: Uuid, coordinates_path: &str) -> Result<BitCoordinates, String> {
    let file = File::open(coordinates_path)
        .map_err(|e| format!("Failed to open coordinates file: {}", e))?;

    let mut reader = Reader::from_reader(file);

    for result in reader.deserialize() {
        let record: BitCoordinates =
            result.map_err(|e| format!("Failed to parse CSV record: {}", e))?;

        // Compare UUID strings
        if record.bit_id == bit_id.to_string() {
            return Ok(record);
        }
    }

    Err(format!("No coordinates found for bit ID: {}", bit_id))
}

pub async fn initialize_bit_coordinates(app_handle: &AppHandle) -> Result<(), String> {
    let app_dir = app_handle.path().app_local_data_dir().unwrap();
    let file_path = app_dir.join("bit_coordinates.csv");

    // Check if file already exists
    if file_path.exists() {
        println!("Bit coordinates file already exists at: {:?}", file_path);
        return Ok(());
    }

    // Create default coordinates
    let default_coordinates = vec![
        BitCoordinates {
            bit_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "Straight Bit 1/4\"".to_string(),
            x: 3.8186,
            y: 3.5563,
            z: -4.0291,
        },
        BitCoordinates {
            bit_id: "6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string(),
            name: "V-Groove 60°".to_string(),
            x: 3.9186,
            y: 3.6563,
            z: -4.1291,
        },
        BitCoordinates {
            bit_id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
            name: "Ball Nose 1/8\"".to_string(),
            x: 4.0186,
            y: 3.7563,
            z: -4.2291,
        },
        BitCoordinates {
            bit_id: "7f2c4a1b-8d5e-4c3f-9f6a-1d2b3e4f5a6b".to_string(),
            name: "Downcut Spiral 3/8\"".to_string(),
            x: 3.8186,
            y: 3.5563,
            z: -4.0291,
        },
        BitCoordinates {
            bit_id: "9e8d7c6b-5a4f-3e2d-1c0b-9a8b7c6d5e4f".to_string(),
            name: "Compression 3/8\"".to_string(),
            x: 3.8186,
            y: 3.5563,
            z: -4.0291,
        },
    ];

    // Ensure the directory exists
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Create and write to the CSV file
    let writer = File::create(&file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    let mut csv_writer = Writer::from_writer(writer);

    // Write records
    for coord in default_coordinates {
        csv_writer
            .serialize(coord)
            .map_err(|e| format!("Failed to write record: {}", e))?;
    }

    // Flush the writer to ensure all data is written
    csv_writer
        .flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))?;

    println!("Created new bit coordinates file at: {:?}", file_path);
    Ok(())
}
struct GCodeGenerator {
    app_handle: AppHandle,
    coordinates: BitCoordinates,
    coordinates_path: String,
    plywood_thickness: f64,
    calculate_workpiece_zero: bool,
    calculate_workpiece_height: bool,
}

impl GCodeGenerator {
    async fn new(app_handle: AppHandle, data: &str) -> Result<Self, String> {
        let app_data_dir = app_handle.path().app_local_data_dir().unwrap();
        let coordinates_path = app_data_dir
            .join("bit_coordinates.csv")
            .to_str()
            .ok_or("Failed to construct coordinates path")?
            .to_string();

        let _ = initialize_bit_coordinates(&app_handle).await;

        let request: GenerateRequest = serde_json::from_str(data).map_err(|e| e.to_string())?;
        println!("Received request: {:?}", request);

        let mut coordinates = find_coordinates(request.router_bit.id, &coordinates_path)?;

        // Apply offset based on plywood thickness when calculating workpiece zero
        if request.calculate_workpiece_zero {
            coordinates.set_z(coordinates.get_z() - request.plywood_thickness);
        }

        Ok(Self {
            app_handle,
            coordinates,
            coordinates_path,
            plywood_thickness: request.plywood_thickness,
            calculate_workpiece_zero: request.calculate_workpiece_zero,
            calculate_workpiece_height: request.calculate_workpiece_height,
        })
    }

    fn generate_common_header(&self) -> String {
        "G20 ; Set machine to inch mode\n\
         G90 ; Set to absolute positioning\n\n"
            .to_string()
    }

    fn write_gcode(&self, filename: &str, gcode: String) -> Result<(), String> {
        let file_path = self
            .app_handle
            .path()
            .download_dir()
            .unwrap()
            .join(filename);

        std::fs::write(&file_path, gcode).map_err(|e| e.to_string())?;
        println!("Generated G-code file at: {:?}", file_path);
        Ok(())
    }
}
#[tauri::command]
async fn move_to_workpiece_zero(app_handle: AppHandle, data: &str) -> Result<(), String> {
    let generator = GCodeGenerator::new(app_handle, data).await?;

    let gcode = format!(
        "( Move to specified X, Y, and Z coordinates in machine coordinates )\n\n{}\
         ( Move to X{}, Y{}, Z{} in machine coordinates )\n\
         G53 G0 X{} Y{} ; Rapid move to new X and Y in machine coordinates\n\
         G53 G0 Z{} ; Rapid move to new Z in machine coordinates\n\n\
         M30 ; End of program\n",
        generator.generate_common_header(),
        generator.coordinates.x,
        generator.coordinates.y,
        generator.coordinates.z,
        generator.coordinates.x,
        generator.coordinates.y,
        generator.coordinates.z
    );

    generator.write_gcode("SET_ZERO_LOCATION.TAP", gcode)
}
#[tauri::command]
async fn move_to_spoilboard_zero(app_handle: AppHandle, data: &str) -> Result<(), String> {
    let generator = GCodeGenerator::new(app_handle, data).await?;

    let gcode = format!(
        "( Move to spoilboard zero position )\n\n{}\
         ( Move to machine coordinates )\n\
         G53 G0 X0.7234 Y1.0276 ; Rapid move to machine coordinates\n\n\
         M30 ; End of program\n",
        generator.generate_common_header()
    );

    generator.write_gcode("MOVE_TO_SPOILBOARD_ZERO.TAP", gcode)
}

#[tauri::command]
async fn set_z_machine_coordinate(app_handle: AppHandle, data: &str) -> Result<(), String> {
    let generator = GCodeGenerator::new(app_handle, data).await?;

    // Calculate Z coordinate based on workpiece height
    let z_coordinate = if generator.calculate_workpiece_height {
        generator.coordinates.get_z() + generator.plywood_thickness + 2.0
    } else {
        generator.coordinates.get_z()
    };

    let gcode = format!(
        "( Move to workpiece position and then Z machine coordinate )\n\n{}\
         ( Move to workpiece X=3, Y=3 )\n\
         G0 X3 Y3 ; Rapid move to workpiece position\n\n\
         ( Move to Z{} in machine coordinates )\n\
         G53 G0 Z{} ; Rapid move to new Z in machine coordinates\n\n\
         M30 ; End of program\n",
        generator.generate_common_header(),
        z_coordinate,
        z_coordinate
    );

    generator.write_gcode("SET_Z_MACHINE_COORDINATE.TAP", gcode)
}
#[tauri::command]
async fn get_bit_coordinates(app_handle: AppHandle) -> Result<Vec<BitCoordinates>, String> {
    let conn = db::get_db(&app_handle)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let mut stmt = conn.prepare("SELECT bit_id, name, x, y, z FROM bit_coordinates")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let coords = stmt.query_map([], |row| {
        Ok(BitCoordinates {
            bit_id: row.get(0)?,
            name: row.get(1)?,
            x: row.get(2)?,
            y: row.get(3)?,
            z: row.get(4)?,
        })
    })
        .map_err(|e| format!("Failed to query coordinates: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect coordinates: {}", e))?;

    Ok(coords)
}

#[tauri::command]
async fn modify_bit_coordinates(app_handle: AppHandle, coordinate: BitCoordinates) -> Result<(), String> {
    let conn = db::get_db(&app_handle)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    conn.execute(
        "UPDATE bit_coordinates SET x = ?1, y = ?2, z = ?3 WHERE bit_id = ?4",
        params![coordinate.x, coordinate.y, coordinate.z, coordinate.bit_id],
    )
        .map_err(|e| format!("Failed to update coordinates: {}", e))?;

    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            db::initialize_db(&app.handle())?;
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_bit_coordinates,
            modify_bit_coordinates,
            move_to_workpiece_zero,
            set_z_machine_coordinate,
            create_router_bit,
            get_router_bits,
            move_to_spoilboard_zero
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
