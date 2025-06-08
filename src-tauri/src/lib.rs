// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod network;

use network::{register_service, discover_peers};
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
}
