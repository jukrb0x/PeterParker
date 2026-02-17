use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Network operation failed: {0}")]
    Network(#[from] std::io::Error),

    #[error("ARP operation failed: {0}")]
    Arp(String),

    #[error("Port scan failed for {host}:{port}")]
    PortScan { host: String, port: u16 },

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Timeout occurred after {0}ms")]
    Timeout(u64),

    #[error("Invalid IP address: {0}")]
    InvalidIp(String),

    #[error("Invalid port range: {0}")]
    InvalidPortRange(String),

    #[error("Invalid MAC address: {0}")]
    InvalidMac(String),

    #[error("Permission denied for operation: {0}")]
    Permission(String),

    #[error("Scanner configuration error: {0}")]
    Config(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Scan already in progress")]
    ScanInProgress,

    #[error("No scan in progress")]
    NoScanInProgress,

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, ScannerError>;

impl ScannerError {
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(std::io::Error::new(
            std::io::ErrorKind::Other,
            msg.into(),
        ))
    }

    pub fn arp(msg: impl Into<String>) -> Self {
        Self::Arp(msg.into())
    }

    pub fn port_scan(host: impl Into<String>, port: u16) -> Self {
        Self::PortScan {
            host: host.into(),
            port,
        }
    }

    pub fn timeout(ms: u64) -> Self {
        Self::Timeout(ms)
    }

    pub fn invalid_ip(ip: impl Into<String>) -> Self {
        Self::InvalidIp(ip.into())
    }

    pub fn invalid_port_range(range: impl Into<String>) -> Self {
        Self::InvalidPortRange(range.into())
    }

    pub fn invalid_mac(mac: impl Into<String>) -> Self {
        Self::InvalidMac(mac.into())
    }

    pub fn permission(op: impl Into<String>) -> Self {
        Self::Permission(op.into())
    }

    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    pub fn device_not_found(id: impl Into<String>) -> Self {
        Self::DeviceNotFound(id.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}