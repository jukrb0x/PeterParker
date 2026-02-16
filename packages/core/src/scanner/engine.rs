use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::fingerprint::vendor::{lookup_vendor, get_oui};
use crate::models::{Device, ScanConfig, Port};
use super::{IpRange, arp};

/// Events emitted during scanning
#[derive(Debug, Clone)]
pub enum ScanEvent {
    /// Progress update
    Progress {
        scanned: u32,
        total: u32,
        found: u32,
        current: Option<String>,
    },
    /// Device discovered
    DeviceFound(Device),
    /// Scan completed
    Completed,
    /// Scan error
    Error(String),
}

pub struct ScannerEngine {
    config: ScanConfig,
    event_tx: Option<mpsc::Sender<ScanEvent>>,
}

impl ScannerEngine {
    pub fn new(config: ScanConfig) -> Self {
        Self { 
            config,
            event_tx: None,
        }
    }

    /// Create engine with event channel for progress updates
    pub fn with_events(config: ScanConfig, event_tx: mpsc::Sender<ScanEvent>) -> Self {
        Self { 
            config,
            event_tx: Some(event_tx),
        }
    }
    
    pub async fn scan_range(&self, range: IpRange) -> Result<Vec<Device>, Box<dyn std::error::Error + Send + Sync>> {
        let timeout = Duration::from_millis(self.config.timeout);
        let concurrency = self.config.concurrency;
        
        // Get ARP table first for MAC addresses
        let arp_table = arp::get_arp_table().await.unwrap_or_default();
        let arp_map: std::collections::HashMap<Ipv4Addr, String> = arp_table
            .into_iter()
            .map(|e| (e.ip, e.mac))
            .collect();
        
        // Determine ports to scan
        let ports: Vec<u16> = match &self.config.ports {
            crate::models::PortSelection::Top100 => crate::models::port::TOP_PORTS[..20].to_vec(), // Scan fewer for speed
            crate::models::PortSelection::Top1000 => crate::models::port::TOP_PORTS[..50].to_vec(),
            crate::models::PortSelection::All => (1..=1000).collect(), // Cap at 1000 for performance
            crate::models::PortSelection::Custom(p) => p.clone(),
        };
        
        let hosts: Vec<Ipv4Addr> = range.hosts().collect();
        let total_hosts = hosts.len() as u32;
        
        let devices = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let scanned = Arc::new(AtomicU32::new(0));
        let found = Arc::new(AtomicU32::new(0));
        
        // Use semaphore for concurrency control
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut tasks = JoinSet::new();
        
        for ip in hosts {
            let permit = semaphore.clone().acquire_owned().await?;
            let ports = ports.clone();
            let devices = devices.clone();
            let scanned = scanned.clone();
            let found = found.clone();
            let arp_map = arp_map.clone();
            let event_tx = self.event_tx.clone();
            
            tasks.spawn(async move {
                let _permit = permit;
                
                // Try ARP first (faster than ping for local network)
                let mac = arp_map.get(&ip).cloned();
                
                // If no ARP entry, try ping
                let is_online = if mac.is_some() {
                    true
                } else {
                    quick_ping_check(ip).await
                };
                
                // Get MAC via ARP ping if needed
                let mac = if is_online && mac.is_none() {
                    arp::arp_ping(ip).await.ok()
                } else {
                    mac
                };
                
                // Count as scanned regardless of result
                let current_scanned = scanned.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                
                // Scan ports for online hosts
                let open_ports = if is_online {
                    tcp_scan_ports(&ip.to_string(), &ports, timeout).await
                } else {
                    Vec::new()
                };
                
                // Create device if online or has ports
                if is_online || !open_ports.is_empty() {
                    let mut device = Device::new(ip.to_string());
                    device.mac = mac.clone();
                    device.is_online = is_online || !open_ports.is_empty();
                    device.ports = open_ports;
                    
                    // Look up vendor from MAC
                    if let Some(ref mac) = mac {
                        let oui = get_oui(mac);
                        if let Some(vendor) = lookup_vendor(&oui) {
                            // Use full name if available, otherwise short name
                            device.vendor = vendor.full_name.clone()
                                .or_else(|| Some(vendor.short_name.clone()));
                        }
                    }
                    
                    // Try to get HTTP info
                    if device.ports.iter().any(|p| p.number == 80) {
                        if let Some((title, server)) = fetch_http_info(&ip.to_string(), timeout).await {
                            device.metadata.http_title = Some(title);
                            device.metadata.http_server = Some(server);
                        }
                    }
                    
                    // Classify device
                    device.classify();
                    
                    let current_found = found.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    
                    // Add to devices list
                    {
                        let mut devices = devices.lock().await;
                        devices.push(device.clone());
                    }
                    
                    // Send event
                    if let Some(tx) = event_tx {
                        let _ = tx.send(ScanEvent::DeviceFound(device)).await;
                        let _ = tx.send(ScanEvent::Progress {
                            scanned: current_scanned,
                            total: total_hosts,
                            found: current_found,
                            current: Some(ip.to_string()),
                        }).await;
                    }
                } else {
                    // Send progress even for non-responsive hosts
                    if current_scanned % 10 == 0 {
                        if let Some(ref tx) = event_tx {
                            let current_found = found.load(std::sync::atomic::Ordering::SeqCst);
                            let _ = tx.send(ScanEvent::Progress {
                                scanned: current_scanned,
                                total: total_hosts,
                                found: current_found,
                                current: Some(ip.to_string()),
                            }).await;
                        }
                    }
                }
            });
        }
        
        // Wait for all tasks
        while tasks.join_next().await.is_some() {}
        
        // Send completion event
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(ScanEvent::Completed).await;
        }
        
        let devices = devices.lock().await.clone();
        Ok(devices)
    }
}

async fn quick_ping_check(ip: Ipv4Addr) -> bool {
    let ip_str = ip.to_string();
    
    #[cfg(target_os = "macos")]
    let output = tokio::process::Command::new("ping")
        .args(["-c", "1", "-W", "1", &ip_str])
        .output();
    
    #[cfg(target_os = "linux")]
    let output = tokio::process::Command::new("ping")
        .args(["-c", "1", "-W", "1", &ip_str])
        .output();
    
    #[cfg(target_os = "windows")]
    let output = tokio::process::Command::new("ping")
        .args(["-n", "1", "-w", "1000", &ip_str])
        .output();
    
    match output.await {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

async fn tcp_scan_ports(host: &str, ports: &[u16], timeout: Duration) -> Vec<Port> {
    use std::net::TcpStream;
    
    let mut open_ports = Vec::new();
    
    for &port in ports {
        let addr = format!("{}:{}", host, port);
        
        match tokio::time::timeout(
            timeout,
            tokio::task::spawn_blocking({
                let addr = addr.clone();
                let timeout = timeout;
                move || {
                    TcpStream::connect_timeout(
                        &addr.parse().ok()?,
                        timeout,
                    ).ok()
                }
            })
        ).await {
            Ok(Ok(Some(_))) => {
                open_ports.push(Port::open_tcp(port));
            }
            _ => {}
        }
    }
    
    open_ports
}

async fn fetch_http_info(host: &str, timeout: Duration) -> Option<(String, String)> {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    
    let host = host.to_string();
    
    let result = tokio::task::spawn_blocking(move || {
        let addr = format!("{}:80", host);
        let mut stream = TcpStream::connect_timeout(
            &addr.parse().ok()?,
            timeout,
        ).ok()?;
        
        let request = format!(
            "GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: PeterParker/0.1\r\nConnection: close\r\n\r\n",
            host
        );
        stream.write_all(request.as_bytes()).ok()?;
        
        let mut response = String::new();
        stream.read_to_string(&mut response).ok()?;
        
        Some(response)
    }).await;
    
    let response = result.ok()??;
    
    // Extract server header
    let server = response
        .lines()
        .find(|line| line.to_lowercase().starts_with("server:"))
        .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
        .unwrap_or_default();
    
    // Extract title
    let title = extract_title(&response).unwrap_or_default();
    
    if title.is_empty() && server.is_empty() {
        return None;
    }
    
    Some((title, server))
}

fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title")?;
    let end = lower.find("</title>")?;
    
    if start >= end {
        return None;
    }
    
    let title_content = &html[start..end];
    let content_start = title_content.find('>')?;
    
    let title = title_content[content_start + 1..].trim();
    
    // Limit title length
    if title.len() > 100 {
        Some(format!("{}...", &title[..97]))
    } else {
        Some(title.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_title() {
        let html = "<html><head><title>Test Page</title></head><body></body></html>";
        assert_eq!(extract_title(html), Some("Test Page".to_string()));
        
        let html = "<html><head><TITLE>UPPERCASE</TITLE></head></html>";
        assert_eq!(extract_title(html), Some("UPPERCASE".to_string()));
    }
}
