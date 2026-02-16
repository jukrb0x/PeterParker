use tauri::State;
use uuid::Uuid;

use peterparker_core::models::{ScanConfig, ScanProgress, ScanResult, ScanStatus};
use peterparker_core::scanner::ScannerEngine;

use crate::{ScanStore, DeviceStore};

#[tauri::command]
pub async fn start_scan(
    config: ScanConfig,
    scan_store: State<'_, ScanStore>,
    device_store: State<'_, DeviceStore>,
) -> Result<String, String> {
    let scan_id = Uuid::new_v4().to_string();
    
    let range = peterparker_core::scanner::IpRange::parse(&config.target_range)
        .map_err(|_| format!("Invalid IP range: {}", config.target_range))?;
    
    let total_hosts = range.size();
    let progress = ScanProgress::new(scan_id.clone(), total_hosts);
    scan_store.insert(scan_id.clone(), (progress, None));
    
    let scan_id_clone = scan_id.clone();
    let scan_store_clone = scan_store.inner().clone();
    let device_store_clone = device_store.inner().clone();
    let config_clone = config.clone();
    
    tokio::spawn(async move {
        let engine = ScannerEngine::new(config_clone.clone());
        let mut result = ScanResult::new(scan_id_clone.clone(), config_clone);
        
        if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
            entry.0.status = ScanStatus::Running;
        }
        
        match engine.scan_range(range).await {
            Ok(devices) => {
                result.devices = devices.clone();
                result.completed_at = Some(chrono::Utc::now());
                
                let mut store = device_store_clone.write().await;
                for device in devices {
                    if let Some(existing) = store.iter_mut().find(|d| d.ip == device.ip) {
                        *existing = device.clone();
                    } else {
                        store.push(device.clone());
                    }
                }
                
                if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                    entry.0.status = ScanStatus::Completed;
                    entry.0.scanned_hosts = entry.0.total_hosts;
                    entry.0.found_devices = result.devices.len() as u32;
                    entry.1 = Some(result);
                }
            }
            Err(e) => {
                if let Some(mut entry) = scan_store_clone.get_mut(&scan_id_clone) {
                    entry.0.status = ScanStatus::Error;
                    entry.0.error = Some(e.to_string());
                }
            }
        }
    });
    
    Ok(scan_id)
}

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

#[tauri::command]
pub async fn pause_scan(_scan_id: String, _scan_store: State<'_, ScanStore>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn resume_scan(_scan_id: String, _scan_store: State<'_, ScanStore>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn cancel_scan(scan_id: String, scan_store: State<'_, ScanStore>) -> Result<(), String> {
    scan_store.remove(&scan_id);
    Ok(())
}

#[tauri::command]
pub async fn get_scan_result(
    scan_id: String,
    scan_store: State<'_, ScanStore>,
) -> Result<ScanResult, String> {
    scan_store
        .get(&scan_id)
        .and_then(|entry| entry.1.clone())
        .ok_or_else(|| "Scan result not found".to_string())
}
