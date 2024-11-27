// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{AppHandle, Manager};


#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    let app_dir = app_handle.path().download_dir().unwrap();
    dbg!(&app_dir);
    let file_path = app_dir.join("example.txt");

    let thing = match std::fs::write(file_path, name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    };



    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn write_positioning_file(app_handle: AppHandle, content: String) -> Result<(), String> {
    let app_dir = app_handle.path().download_dir().unwrap();
    let file_path = app_dir.join("example.txt");

    match std::fs::write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, write_positioning_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
