use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;
use dashmap::DashMap;

use crate::models::{Device, ScanProgress, ScanResult, ScanConfig};
use crate::scanner::ScannerEngine;

pub type ScanStore = Arc<DashMap<String, (ScanProgress, Option<ScanResult>)>>;
pub type DeviceStore = Arc<RwLock<Vec<Device>>>;

#[tauri::command]
pub async fn start_scan(
    config: ScanConfig,
    progress_store: State<'_, ScanStore>,
    device_store: State<'_, DeviceStore>,
) -> Result<String, String> {
    let scan_id = uuid::Uuid::new_v4().to_string();
    
    // Parse target range
    let range = crate::scanner::IpRange::parse(&config.target_range)
        .map_err(|e| format!("Invalid range: {}", e))?;
    
    let total_hosts = range.size();
    
    // Initialize progress
    let progress = ScanProgress::new(scan_id.clone(), total_hosts);
    progress_store.insert(scan_id.clone(), (progress, None));
    
    // Start scanner in background
    let scan_id_clone = scan_id.clone();
    let progress_store_clone = progress_store.inner().clone();
    let device_store_clone = device_store.inner().clone();
    let config_clone = config.clone();
    
    tokio::spawn(async move {
        let engine = ScannerEngine::new(config_clone);
        let mut result = ScanResult::new(scan_id_clone.clone(), config_clone);
        
        // Update status to running
        if let Some((p, _)) = progress_store_clone.get_mut(&scan_id_clone) {
            p.status = crate::models::ScanStatus::Running;
        }
        
        match engine.scan_range(range).await {
            Ok(devices) => {
                result.devices = devices.clone();
                result.completed_at = Some(chrono::Utc::now());
                
                // Update device store
                let mut store = device_store_clone.write().await;
                for device in devices {
                    if let Some(existing) = store.iter_mut().find(|d| d.ip == device.ip) {
                        *existing = device.clone();
                    } else {
                        store.push(device);
                    }
                }
                
                // Update progress
                if let Some((p, r)) = progress_store_clone.get_mut(&scan_id_clone) {
                    p.status = crate::models::ScanStatus::Completed;
                    p.scanned_hosts = p.total_hosts;
                    p.found_devices = result.devices.len() as u32;
                    *r = Some(result);
                }
            }
            Err(e) => {
                if let Some((p, _)) = progress_store_clone.get_mut(&scan_id_clone) {
                    p.status = crate::models::ScanStatus::Error;
                    p.error = Some(e.to_string());
                }
            }
        }
    });
    
    Ok(scan_id)
}

#[tauri::command]
pub async fn get_scan_progress(
    scan_id: String,
    progress_store: State<'_, ScanStore>,
) -> Result<ScanProgress, String> {
    progress_store
        .get(&scan_id)
        .map(|entry| entry.0.clone())
        .ok_or_else(|| "Scan not found".to_string())
}

#[tauri::command]
pub async fn pause_scan(
    scan_id: String,
    _progress_store: State<'_, ScanStore>,
) -> Result<(), String> {
    // TODO: Implement pause logic
    Err("Pause not yet implemented".to_string())
}

#[tauri::command]
pub async fn resume_scan(
    scan_id: String,
    _progress_store: State<'_, ScanStore>,
) -> Result<(), String> {
    // TODO: Implement resume logic
    Err("Resume not yet implemented".to_string())
}

#[tauri::command]
pub async fn cancel_scan(
    scan_id: String,
    progress_store: State<'_, ScanStore>,
) -> Result<(), String> {
    progress_store.remove(&scan_id);
    Ok(())
}

#[tauri::command]
pub async fn get_scan_result(
    scan_id: String,
    progress_store: State<'_, ScanStore>,
) -> Result<ScanResult, String> {
    progress_store
        .get(&scan_id)
        .and_then(|entry| entry.1.clone())
        .ok_or_else(|| "Scan result not found".to_string())
}
