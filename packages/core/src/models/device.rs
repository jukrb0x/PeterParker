use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{OperatingSystem, Port, DeviceMetadata};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Router,
    Switch,
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Iot,
    Printer,
    Nas,
    Camera,
    Tv,
    GameConsole,
    Server,
    #[default]
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Router => write!(f, "Router"),
            DeviceType::Switch => write!(f, "Switch"),
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Laptop => write!(f, "Laptop"),
            DeviceType::Mobile => write!(f, "Mobile"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::Iot => write!(f, "IoT"),
            DeviceType::Printer => write!(f, "Printer"),
            DeviceType::Nas => write!(f, "NAS"),
            DeviceType::Camera => write!(f, "Camera"),
            DeviceType::Tv => write!(f, "TV"),
            DeviceType::GameConsole => write!(f, "Game Console"),
            DeviceType::Server => write!(f, "Server"),
            DeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub ip: String,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub hostname: Option<String>,
    pub os: Option<OperatingSystem>,
    #[serde(rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "firstSeen")]
    pub first_seen: DateTime<Utc>,
    #[serde(rename = "lastSeen")]
    pub last_seen: DateTime<Utc>,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    pub ports: Vec<Port>,
    pub metadata: DeviceMetadata,
}

impl Device {
    pub fn new(ip: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            ip,
            mac: None,
            vendor: None,
            hostname: None,
            os: None,
            device_type: DeviceType::Unknown,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            is_online: true,
            ports: Vec::new(),
            metadata: DeviceMetadata::default(),
        }
    }

    pub fn classify(&mut self) {
        // Classify device based on ports and metadata
        let has_http = self.ports.iter().any(|p| p.number == 80 || p.number == 443 || p.number == 8080);
        let has_ssh = self.ports.iter().any(|p| p.number == 22);
        let has_rdp = self.ports.iter().any(|p| p.number == 3389);
        let has_smb = self.ports.iter().any(|p| p.number == 445);
        let has_print = self.ports.iter().any(|p| p.number == 9100 || p.number == 631);
        
        // Check HTTP title for hints
        let title = self.metadata.http_title.as_deref().unwrap_or("");
        
        if title.contains("Router") || title.contains("路由器") {
            self.device_type = DeviceType::Router;
        } else if has_print {
            self.device_type = DeviceType::Printer;
        } else if has_rdp || has_smb {
            self.device_type = DeviceType::Desktop;
        } else if has_ssh && has_http {
            self.device_type = DeviceType::Server;
        } else if has_http {
            self.device_type = DeviceType::Iot;
        }
    }
}
