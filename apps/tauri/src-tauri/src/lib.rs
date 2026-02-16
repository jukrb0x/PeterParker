use dashmap::DashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

use peterparker_core::models::{Device, ScanProgress, ScanResult};

pub mod commands;

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
            commands::scan::get_devices,
            commands::scan::get_device,
            commands::scan::delete_device,
            commands::scan::rescan_device,
            commands::scan::export_devices,
            commands::scan::get_network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
