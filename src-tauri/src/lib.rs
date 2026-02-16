pub mod commands;
pub mod db;
pub mod fingerprint;
pub mod models;
pub mod scanner;

use dashmap::DashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

use models::{Device, ScanProgress, ScanResult};

pub type ScanStore = Arc<DashMap<String, (ScanProgress, Option<ScanResult>)>>;
pub type DeviceStore = Arc<RwLock<Vec<Device>>>;

pub fn run() {
    let scan_store: ScanStore = Arc::new(DashMap::new());
    let device_store: DeviceStore = Arc::new(RwLock::new(Vec::new()));

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
            commands::scan::start_scan,
            commands::scan::get_scan_progress,
            commands::scan::pause_scan,
            commands::scan::resume_scan,
            commands::scan::cancel_scan,
            commands::scan::get_scan_result,
            commands::device::get_devices,
            commands::device::get_device,
            commands::device::delete_device,
            commands::device::rescan_device,
            commands::device::export_devices,
            commands::network::get_network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
