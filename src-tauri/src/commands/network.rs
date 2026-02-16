use std::net::IpAddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip: String,
    pub netmask: String,
}

#[tauri::command]
pub fn get_network_info() -> Result<NetworkInfo, String> {
    let interfaces = get_local_interfaces();
    interfaces
        .into_iter()
        .next()
        .ok_or_else(|| "No network interface found".to_string())
}

fn get_local_interfaces() -> Vec<NetworkInfo> {
    let mut result = Vec::new();
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("ifconfig").output() {
            let output = String::from_utf8_lossy(&output.stdout);
            let mut current_interface: Option<String> = None;
            
            for line in output.lines() {
                if line.starts_with("en") && line.contains(":") {
                    current_interface = line.split(':').next().map(|s| s.to_string());
                } else if let Some(ref iface) = current_interface {
                    if line.contains("inet ") && !line.contains("inet6") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 {
                            result.push(NetworkInfo {
                                interface: iface.clone(),
                                ip: parts[1].to_string(),
                                netmask: parts[3].to_string(),
                            });
                            current_interface = None;
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Use /proc/net/route or ip command
        if let Ok(output) = std::process::Command::new("ip")
            .args(["addr", "show"])
            .output()
        {
            let output = String::from_utf8_lossy(&output.stdout);
            // Parse ip addr output
            for line in output.lines() {
                if line.contains("inet ") && !line.contains("inet6") && !line.contains("127.0.0.1") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Some((ip, mask)) = parts[1].split_once('/') {
                            result.push(NetworkInfo {
                                interface: "eth0".to_string(),
                                ip: ip.to_string(),
                                netmask: mask.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("ipconfig")
            .output()
        {
            let output = String::from_utf8_lossy(&output.stdout);
            // Parse ipconfig output
            // Simplified parsing
        }
    }
    
    result
}
