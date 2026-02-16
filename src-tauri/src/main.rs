// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dashmap::DashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

use peterparker_lib::{
    commands::*,
    models::{Device, ScanProgress, ScanResult},
};

fn main() {
    // Create shared state
    let scan_store: Arc<DashMap<String, (ScanProgress, Option<ScanResult>)>> =
        Arc::new(DashMap::new());
    let device_store: Arc<RwLock<Vec<Device>>> = Arc::new(RwLock::new(Vec::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(scan_store)
        .manage(device_store)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_scan,
            get_scan_progress,
            pause_scan,
            resume_scan,
            cancel_scan,
            get_scan_result,
            get_devices,
            get_device,
            delete_device,
            rescan_device,
            export_devices,
            get_network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
