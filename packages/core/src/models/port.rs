use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum PortState {
    Open,
    #[default]
    Closed,
    Filtered,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    #[default]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub version: Option<String>,
    pub product: Option<String>,
    #[serde(rename = "extraInfo")]
    pub extra_info: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub number: u16,
    pub protocol: Protocol,
    pub state: PortState,
    pub service: Option<Service>,
    pub banner: Option<String>,
}

impl Port {
    pub fn open_tcp(number: u16) -> Self {
        Self {
            number,
            protocol: Protocol::Tcp,
            state: PortState::Open,
            service: None,
            banner: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingSystem {
    pub name: String,
    pub family: String,
    pub version: Option<String>,
    pub confidence: u8,
    pub cpe: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceMetadata {
    #[serde(rename = "httpTitle")]
    pub http_title: Option<String>,
    #[serde(rename = "httpServer")]
    pub http_server: Option<String>,
    #[serde(rename = "sshVersion")]
    pub ssh_version: Option<String>,
    #[serde(rename = "smbInfo")]
    pub smb_info: Option<serde_json::Value>,
    #[serde(rename = "ttl")]
    pub ttl: Option<u8>,
    #[serde(rename = "windowSize")]
    pub window_size: Option<u16>,
}

// Top 100 most common ports
pub const TOP_PORTS: [u16; 100] = [
    80, 443, 22, 21, 25, 3389, 110, 445, 139, 143,
    53, 135, 3306, 8080, 1723, 111, 995, 993, 5900, 1025,
    587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000,
    514, 5060, 179, 1026, 2000, 8443, 8000, 32768, 554, 26,
    1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646, 5000,
    5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121,
    1110, 49155, 6000, 513, 990, 5357, 427, 49156, 543, 544,
    5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070,
    5190, 3000, 5432, 1900, 3986, 13, 1029, 9, 5051, 6646,
    49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
];
