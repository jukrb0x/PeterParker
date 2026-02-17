use tauri::{State, Emitter, AppHandle};
use tokio::sync::mpsc;
use uuid::Uuid;

use peterparker_core::models::{ScanConfig, ScanProgress, ScanResult, ScanStatus};
use peterparker_core::scanner::{ScannerEngine, ScanEvent};

use crate::{ScanStore, DeviceStore};

/// Start a new network scan
#[tauri::command]
pub async fn start_scan(
    config: ScanConfig,
    scan_store: State<'_, ScanStore>,
    device_store: State<'_, DeviceStore>,
    app: AppHandle,
) -> Result<String, String> {
    let scan_id = Uuid::new_v4().to_string();
    
    // Parse IP range
    let range = peterparker_core::scanner::IpRange::parse(&config.target_range)
        .map_err(|_| format!("Invalid IP range: {}", config.target_range))?;
    
    let total_hosts = range.size();
    
    // Initialize progress
    let progress = ScanProgress::new(scan_id.clone(), total_hosts);
    scan_store.insert(scan_id.clone(), (progress, None));
    
    // Create event channel
    let (event_tx, mut event_rx) = mpsc::channel::<ScanEvent>(100);
    
    let scan_id_clone = scan_id.clone();
    let scan_store_clone = scan_store.inner().clone();
    let device_store_clone = device_store.inner().clone();
    let config_clone = config.clone();
    
    // Spawn scan task
    tokio::spawn(async move {
        let engine = ScannerEngine::with_events(config_clone.clone(), event_tx.clone());
        let mut result = ScanResult::new(scan_id_clone.clone(), config_clone);
        let mut devices = Vec::new();
        
        // Update status to running
        if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
            entry.0.status = ScanStatus::Running;
        }
        
        // Start scan in a separate task
        let scan_task = tokio::spawn(async move {
            engine.scan_range(range).await
        });
        
        // Handle events while scan is running
        loop {
            tokio::select! {
                Some(event) = event_rx.recv() => {
                    match event {
                        ScanEvent::Progress { scanned, total, found, current } => {
                            if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                                entry.0.scanned_hosts = scanned;
                                entry.0.total_hosts = total;
                                entry.0.found_devices = found;
                                entry.0.current_host = current.clone();
                            }
                            
                            // Emit event to frontend
                            let _ = app.emit("scan-progress", serde_json::json!({
                                "scanId": scan_id_clone.clone(),
                                "scanned": scanned,
                                "total": total,
                                "found": found,
                                "current": current,
                            }));
                        }
                        ScanEvent::DeviceFound(device) => {
                            devices.push((*device).clone());

                            // Add to device store
                            let mut store = device_store_clone.write().await;
                            if let Some(existing) = store.iter_mut().find(|d| d.ip == device.ip) {
                                *existing = (*device).clone();
                            } else {
                                store.push((*device).clone());
                            }

                            // Emit event to frontend
                            let _ = app.emit("device-found", serde_json::json!({
                                "scanId": scan_id_clone.clone(),
                                "device": *device,
                            }));
                        }
                        ScanEvent::Completed => {
                            break;
                        }
                        ScanEvent::Error(e) => {
                            if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                                entry.0.status = ScanStatus::Error;
                                entry.0.error = Some(e.clone());
                            }
                            let _ = app.emit("scan-error", serde_json::json!({
                                "scanId": scan_id_clone.clone(),
                                "error": e,
                            }));
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(30)) => {
                    // Timeout
                    break;
                }
            }
        }
        
        // Wait for scan task to complete
        match scan_task.await {
            Ok(Ok(scan_devices)) => {
                result.devices = scan_devices;
                result.completed_at = Some(chrono::Utc::now());
                
                if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                    entry.0.status = ScanStatus::Completed;
                    entry.0.scanned_hosts = entry.0.total_hosts;
                    entry.0.found_devices = result.devices.len() as u32;
                    entry.1 = Some(result);
                }
                
                // Emit completion event
                let _ = app.emit("scan-completed", serde_json::json!({
                    "scanId": scan_id_clone.clone(),
                    "devices": devices.len(),
                }));
            }
            Ok(Err(e)) => {
                if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                    entry.0.status = ScanStatus::Error;
                    entry.0.error = Some(e.to_string());
                }
                let _ = app.emit("scan-error", serde_json::json!({
                    "scanId": scan_id_clone.clone(),
                    "error": e.to_string(),
                }));
            }
            Err(_) => {
                if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                    entry.0.status = ScanStatus::Error;
                    entry.0.error = Some("Scan task panicked".to_string());
                }
            }
        }
    });
    
    Ok(scan_id)
}

/// Get scan progress
#[tauri::command]
pub async fn get_scan_progress(
    scan_id: String,
    scan_store: State<'_, ScanStore>,
) -> Result<ScanProgress, String> {
    scan_store
        .get(&scan_id)
        .map(|entry| entry.0.clone())
        .ok_or_else(|| "Scan not found".to_string())
}

/// Pause scan (placeholder - not implemented)
#[tauri::command]
pub async fn pause_scan(_scan_id: String, _scan_store: State<'_, ScanStore>) -> Result<(), String> {
    // TODO: Implement pause functionality
    Ok(())
}

/// Resume scan (placeholder - not implemented)
#[tauri::command]
pub async fn resume_scan(_scan_id: String, _scan_store: State<'_, ScanStore>) -> Result<(), String> {
    // TODO: Implement resume functionality
    Ok(())
}

/// Cancel a scan
#[tauri::command]
pub async fn cancel_scan(scan_id: String, scan_store: State<'_, ScanStore>) -> Result<(), String> {
    scan_store.remove(&scan_id);
    Ok(())
}

/// Get scan result
#[tauri::command]
pub async fn get_scan_result(
    scan_id: String,
   _store: State<'_, ScanStore>,
) -> Result<ScanResult, String> {
    _store
        .get(&scan_id)
        .and_then(|entry| entry.1.clone())
        .ok_or_else(|| "Scan result not found".to_string())
}

/// Get all devices
#[tauri::command]
pub async fn get_devices(device_store: State<'_, DeviceStore>) -> Result<Vec<peterparker_core::models::Device>, String> {
    let devices = device_store.read().await.clone();
    Ok(devices)
}

/// Get a specific device
#[tauri::command]
pub async fn get_device(
    ip: String,
    device_store: State<'_, DeviceStore>,
) -> Result<Option<peterparker_core::models::Device>, String> {
    let devices = device_store.read().await;
    Ok(devices.iter().find(|d| d.ip == ip).cloned())
}

/// Delete a device
#[tauri::command]
pub async fn delete_device(
    ip: String,
    device_store: State<'_, DeviceStore>,
) -> Result<(), String> {
    let mut devices = device_store.write().await;
    devices.retain(|d| d.ip != ip);
    Ok(())
}

/// Rescan a specific device
#[tauri::command]
pub async fn rescan_device(
    ip: String,
    device_store: State<'_, DeviceStore>,
) -> Result<peterparker_core::models::Device, String> {
    let devices = device_store.read().await;
    devices
        .iter()
        .find(|d| d.ip == ip)
        .cloned()
        .ok_or_else(|| "Device not found".to_string())
}

/// Export all devices as JSON
#[tauri::command]
pub async fn export_devices(device_store: State<'_, DeviceStore>) -> Result<String, String> {
    let devices = device_store.read().await.clone();
    serde_json::to_string_pretty(&devices)
        .map_err(|e| format!("Failed to serialize devices: {}", e))
}

/// Get network information
#[tauri::command]
pub async fn get_network_info() -> Result<serde_json::Value, String> {
    // TODO: Implement proper network info detection
    Ok(serde_json::json!({
        "interface": "eth0",
        "ip": "192.168.1.1",
        "netmask": "255.255.255.0"
    }))
}
