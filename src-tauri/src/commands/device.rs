use tauri::State;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::models::Device;
use super::scan::DeviceStore;

#[tauri::command]
pub async fn get_devices(
    device_store: State<'_, DeviceStore>,
) -> Result<Vec<Device>, String> {
    let store = device_store.read().await;
    Ok(store.clone())
}

#[tauri::command]
pub async fn get_device(
    ip: String,
    device_store: State<'_, DeviceStore>,
) -> Result<Option<Device>, String> {
    let store = device_store.read().await;
    Ok(store.iter().find(|d| d.ip == ip).cloned())
}

#[tauri::command]
pub async fn delete_device(
    id: String,
    device_store: State<'_, DeviceStore>,
) -> Result<(), String> {
    let mut store = device_store.write().await;
    store.retain(|d| d.id != id);
    Ok(())
}

#[tauri::command]
pub async fn rescan_device(
    ip: String,
    device_store: State<'_, DeviceStore>,
) -> Result<Device, String> {
    // Quick rescan of single device
    let mut device = Device::new(ip.clone());
    
    // TCP scan for common ports
    let ports = crate::scanner::quick_scan_ports(&ip, &crate::models::port::TOP_PORTS[..20]).await;
    device.ports = ports;
    device.classify();
    
    // Update store
    let mut store = device_store.write().await;
    if let Some(existing) = store.iter_mut().find(|d| d.ip == ip) {
        *existing = device.clone();
    } else {
        store.push(device.clone());
    }
    
    Ok(device)
}

#[tauri::command]
pub async fn export_devices(
    device_store: State<'_, DeviceStore>,
) -> Result<String, String> {
    let store = device_store.read().await;
    serde_json::to_string_pretty(&*store).map_err(|e| e.to_string())
}
