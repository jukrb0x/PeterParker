use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip: String,
    pub netmask: String,
}

#[tauri::command]
pub fn get_network_info() -> Result<NetworkInfo, String> {
    get_local_interfaces()
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
    
    result
}
