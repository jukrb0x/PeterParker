use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::scan::start_scan,
            crate::commands::scan::get_scan_progress,
            crate::commands::scan::pause_scan,
            crate::commands::scan::resume_scan,
            crate::commands::scan::cancel_scan,
            crate::commands::scan::get_scan_result,
            crate::commands::device::get_devices,
            crate::commands::device::get_device,
            crate::commands::device::delete_device,
            crate::commands::device::rescan_device,
            crate::commands::device::export_devices,
            crate::commands::network::get_network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
