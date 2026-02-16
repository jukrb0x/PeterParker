use axum::{
    extract::{State, Path},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;
use peterparker_core::models::{Device, ScanConfig, ScanProgress, ScanResult, ScanStatus};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn get_devices(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<Device>>> {
    let devices = state.devices.read().await.clone();
    Json(ApiResponse {
        success: true,
        data: Some(devices),
        error: None,
    })
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(ip): Path<String>,
) -> Json<ApiResponse<Device>> {
    let devices = state.devices.read().await;
    match devices.iter().find(|d| d.ip == ip).cloned() {
        Some(device) => Json(ApiResponse {
            success: true,
            data: Some(device),
            error: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Device not found".to_string()),
        }),
    }
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub target_range: String,
}

pub async fn start_scan(
    State(state): State<AppState>,
    Json(req): Json<ScanRequest>,
) -> Json<ApiResponse<String>> {
    let scan_id = uuid::Uuid::new_v4().to_string();
    
    let progress = ScanProgress {
        scan_id: scan_id.clone(),
        status: ScanStatus::Running,
        total_hosts: 254,
        scanned_hosts: 0,
        found_devices: 0,
        current_host: None,
        eta: Some(30),
        error: None,
    };
    
    {
        let mut scans = state.scans.write().await;
        scans.push((progress, None));
    }
    
    Json(ApiResponse {
        success: true,
        data: Some(scan_id),
        error: None,
    })
}

pub async fn get_scan_progress(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<ScanProgress>> {
    let scans = state.scans.read().await;
    
    match scans.iter().find(|(p, _)| p.scan_id == id) {
        Some((progress, _)) => Json(ApiResponse {
            success: true,
            data: Some(progress.clone()),
            error: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Scan not found".to_string()),
        }),
    }
}
