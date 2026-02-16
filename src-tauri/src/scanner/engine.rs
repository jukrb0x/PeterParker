use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::models::{Device, ScanConfig, Port};
use super::{IpRange, tcp_scan_ports, fetch_http_info};

pub struct ScannerEngine {
    config: ScanConfig,
}

impl ScannerEngine {
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }
    
    pub async fn scan_range(&self, range: IpRange) -> Result<Vec<Device>, Box<dyn std::error::Error + Send + Sync>> {
        let concurrency = self.config.concurrency;
        let timeout = Duration::from_millis(self.config.timeout);
        
        let ports: Vec<u16> = match &self.config.ports {
            crate::models::PortSelection::Top100 => crate::models::port::TOP_PORTS.to_vec(),
            crate::models::PortSelection::Top1000 => {
                let mut ports = crate::models::port::TOP_PORTS.to_vec();
                // Add more ports for top 1000
                ports.extend(1001..1100);
                ports
            }
            crate::models::PortSelection::All => (1..=65535).collect(),
            crate::models::PortSelection::Custom(p) => p.clone(),
        };
        
        let devices = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let hosts: Vec<Ipv4Addr> = range.hosts().collect();
        
        // Use semaphore for concurrency control
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut tasks = JoinSet::new();
        
        for ip in hosts {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let ports = ports.clone();
            let devices = devices.clone();
            let timeout_val = timeout;
            
            tasks.spawn(async move {
                let _permit = permit;
                
                // Quick ping check first
                if !quick_ping_check(ip).await {
                    return;
                }
                
                // Scan ports
                let open_ports = tcp_scan_ports(&ip.to_string(), &ports[..20.min(ports.len())], timeout_val).await;
                
                if open_ports.is_empty() {
                    return;
                }
                
                let mut device = Device::new(ip.to_string());
                device.ports = open_ports;
                
                // Fetch HTTP info if port 80 or 8080 is open
                if device.ports.iter().any(|p| p.number == 80 || p.number == 8080) {
                    if let Some((title, server)) = fetch_http_info(&ip.to_string(), timeout_val).await {
                        device.metadata.http_title = Some(title);
                        device.metadata.http_server = Some(server);
                    }
                }
                
                device.classify();
                
                let mut devices = devices.lock().await;
                devices.push(device);
            });
        }
        
        // Wait for all tasks
        while tasks.join_next().await.is_some() {}
        
        let devices = devices.lock().await.clone();
        Ok(devices)
    }
}

async fn quick_ping_check(ip: Ipv4Addr) -> bool {
    use std::process::Command;
    
    let ip_str = ip.to_string();
    
    #[cfg(target_os = "macos")]
    let output = Command::new("ping")
        .args(["-c", "1", "-W", "100", &ip_str])
        .output();
    
    #[cfg(target_os = "linux")]
    let output = Command::new("ping")
        .args(["-c", "1", "-W", "1", &ip_str])
        .output();
    
    #[cfg(target_os = "windows")]
    let output = Command::new("ping")
        .args(["-n", "1", "-w", "100", &ip_str])
        .output();
    
    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

pub async fn quick_scan_ports(ip: &str, ports: &[u16]) -> Vec<Port> {
    let timeout = Duration::from_millis(1000);
    tcp_scan_ports(ip, ports, timeout).await
}
