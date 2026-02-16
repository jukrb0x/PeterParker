use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

use crate::models::{Device, ScanConfig, Port, PortState, Protocol};
use super::{IpRange};

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
            let permit = semaphore.clone().acquire_owned().await?;
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
    use std::net::{TcpStream, ToSocketAddrs};
    
    let mut open_ports = Vec::new();
    
    for port in ports {
        let addr = format!("{}:{}", host, port);
        
        match tokio::time::timeout(
            timeout,
            tokio::task::spawn_blocking({
                let addr = addr.clone();
                let timeout = timeout;
                move || {
                    TcpStream::connect_timeout(
                        &addr.to_socket_addrs().ok()?.next()?,
                        timeout,
                    ).ok()
                }
            })
        ).await {
            Ok(Ok(Some(_))) => {
                open_ports.push(Port::open_tcp(*port));
            }
            _ => continue,
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
            "GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: PeterParker/0.1\r\n\r\n",
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
    
    let title_content = &html[start..end];
    let content_start = title_content.find('>')?;
    
    Some(title_content[content_start + 1..].trim().to_string())
}
