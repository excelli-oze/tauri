// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_datapass as datapass;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// fn main() {
//     tauri::Builder::default()
//         .plugin(datapass::init())
//         .setup(|app| {
//             // You can add any additional setup code here
//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
        .plugin(datapass::init())
        .setup(|app| {
            // You can add any additional setup code here
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
