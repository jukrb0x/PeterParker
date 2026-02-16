use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Device, Port};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScanStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScanMethod {
    Arp,
    Icmp,
    TcpSyn,
    TcpConnect,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    #[serde(rename = "targetRange")]
    pub target_range: String,
    pub ports: PortSelection,
    #[serde(rename = "scanType")]
    pub scan_type: ScanMethod,
    pub timeout: u64,
    pub concurrency: usize,
    #[serde(rename = "enableOsDetection")]
    pub enable_os_detection: bool,
    #[serde(rename = "enableServiceDetection")]
    pub enable_service_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortSelection {
    Top100,
    Top1000,
    All,
    Custom(Vec<u16>),
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target_range: "192.168.1.0/24".to_string(),
            ports: PortSelection::Top100,
            scan_type: ScanMethod::Comprehensive,
            timeout: 2000,
            concurrency: 50,
            enable_os_detection: true,
            enable_service_detection: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    #[serde(rename = "scanId")]
    pub scan_id: String,
    pub status: ScanStatus,
    #[serde(rename = "totalHosts")]
    pub total_hosts: u32,
    #[serde(rename = "scannedHosts")]
    pub scanned_hosts: u32,
    #[serde(rename = "foundDevices")]
    pub found_devices: u32,
    #[serde(rename = "currentHost")]
    pub current_host: Option<String>,
    pub eta: Option<u64>,
    pub error: Option<String>,
}

impl ScanProgress {
    pub fn new(scan_id: String, total_hosts: u32) -> Self {
        Self {
            scan_id,
            status: ScanStatus::Pending,
            total_hosts,
            scanned_hosts: 0,
            found_devices: 0,
            current_host: None,
            eta: None,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    #[serde(rename = "scanId")]
    pub scan_id: String,
    pub config: ScanConfig,
    pub devices: Vec<Device>,
    #[serde(rename = "startedAt")]
    pub started_at: DateTime<Utc>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<DateTime<Utc>>,
}

impl ScanResult {
    pub fn new(scan_id: String, config: ScanConfig) -> Self {
        Self {
            scan_id,
            config,
            devices: Vec::new(),
            started_at: Utc::now(),
            completed_at: None,
        }
    }
}
