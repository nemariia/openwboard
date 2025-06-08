// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;

use network::{register_service, discover_peers};
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
    let app_handle = app.handle().clone(); // Fix clone

    let (tx, rx) = std::sync::mpsc::channel();
    discover_peers(tx);
    register_service();

    tauri::async_runtime::spawn(async move {
        while let Ok(peer) = rx.recv() {
            println!("Discovered peer: {}", peer);
            let _ = app_handle.emit("peer-discovered", peer); // Use .emit instead
        }
    });

    Ok(())
})
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    let _registration = register_service();

    let (tx, rx) = std::sync::mpsc::channel();
    discover_peers(tx);
}
