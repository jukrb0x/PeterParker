use std::net::Ipv4Addr;
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;

/// ARP table entry
#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: Ipv4Addr,
    pub mac: String,
}

/// Get ARP table entries for the local network
pub async fn get_arp_table() -> Result<Vec<ArpEntry>, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "macos")]
    {
        get_arp_table_macos().await
    }
    
    #[cfg(target_os = "linux")]
    {
        get_arp_table_linux().await
    }
    
    #[cfg(target_os = "windows")]
    {
        get_arp_table_windows().await
    }
}

/// Parse ARP table from macOS `arp -a` output
#[cfg(target_os = "macos")]
async fn get_arp_table_macos() -> Result<Vec<ArpEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let output = timeout(
        Duration::from_secs(5),
        tokio::task::spawn_blocking(|| {
            Command::new("arp")
                .args(["-a"])
                .output()
        })
    ).await
    .map_err(|_| "Timeout getting ARP table")??
    .map_err(|e| format!("Failed to execute arp command: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to get ARP table".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    
    // Parse lines like: ? (192.168.1.1) at a0:f3:c1:12:34:56 on en0 [ethernet]
    for line in stdout.lines() {
        if let Some(entry) = parse_arp_line_macos(line) {
            entries.push(entry);
        }
    }
    
    Ok(entries)
}

#[cfg(target_os = "macos")]
fn parse_arp_line_macos(line: &str) -> Option<ArpEntry> {
    // Format: ? (192.168.1.1) at a0:f3:c1:12:34:56 on en0 [ethernet]
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 4 {
        return None;
    }
    
    // Find IP address in parentheses
    let ip_str = parts.iter()
        .find(|p| p.starts_with('(') && p.ends_with(')'))?
        .trim_matches(|c| c == '(' || c == ')');
    
    let ip: Ipv4Addr = ip_str.parse().ok()?;
    
    // Find MAC address (after "at")
    let at_idx = parts.iter().position(|p| *p == "at")?;
    if at_idx + 1 >= parts.len() {
        return None;
    }
    
    let mac = parts[at_idx + 1].to_string();
    
    // Validate MAC format
    if mac.len() != 17 || !mac.contains(':') {
        return None;
    }
    
    Some(ArpEntry { ip, mac })
}

/// Parse ARP table from Linux `ip neigh` or `arp -a` output
#[cfg(target_os = "linux")]
async fn get_arp_table_linux() -> Result<Vec<ArpEntry>, Box<dyn std::error::Error + Send + Sync>> {
    // Try `ip neigh` first (modern Linux)
    let result = timeout(
        Duration::from_secs(5),
        tokio::task::spawn_blocking(|| {
            Command::new("ip")
                .args(["neigh", "show"])
                .output()
        })
    ).await;
    
    match result {
        Ok(Ok(output)) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut entries = Vec::new();
            
            // Parse lines like: 192.168.1.1 dev eth0 lladdr a0:f3:c1:12:34:56 REACHABLE
            for line in stdout.lines() {
                if let Some(entry) = parse_ip_neigh_line(line) {
                    entries.push(entry);
                }
            }
            
            return Ok(entries);
        }
        _ => {}
    }
    
    // Fall back to `arp -a`
    let output = timeout(
        Duration::from_secs(5),
        tokio::task::spawn_blocking(|| {
            Command::new("arp")
                .args(["-a"])
                .output()
        })
    ).await
    .map_err(|_| "Timeout getting ARP table")??
    .map_err(|e| format!("Failed to execute arp command: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to get ARP table".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    
    for line in stdout.lines() {
        if let Some(entry) = parse_arp_line_linux(line) {
            entries.push(entry);
        }
    }
    
    Ok(entries)
}

#[cfg(target_os = "linux")]
fn parse_ip_neigh_line(line: &str) -> Option<ArpEntry> {
    // Format: 192.168.1.1 dev eth0 lladdr a0:f3:c1:12:34:56 REACHABLE
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 5 {
        return None;
    }
    
    let ip: Ipv4Addr = parts[0].parse().ok()?;
    
    // Find lladdr index
    let lladdr_idx = parts.iter().position(|p| *p == "lladdr")?;
    if lladdr_idx + 1 >= parts.len() {
        return None;
    }
    
    let mac = parts[lladdr_idx + 1].to_string();
    
    // Validate MAC format
    if !mac.contains(':') {
        return None;
    }
    
    Some(ArpEntry { ip, mac })
}

#[cfg(target_os = "linux")]
fn parse_arp_line_linux(line: &str) -> Option<ArpEntry> {
    // Format varies, but often: ? (192.168.1.1) at a0:f3:c1:12:34:56 [ether] on eth0
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 4 {
        return None;
    }
    
    // Find IP in parentheses
    let ip_str = parts.iter()
        .find(|p| p.starts_with('(') && p.ends_with(')'))?
        .trim_matches(|c| c == '(' || c == ')');
    
    let ip: Ipv4Addr = ip_str.parse().ok()?;
    
    // Find MAC after "at"
    let at_idx = parts.iter().position(|p| *p == "at")?;
    if at_idx + 1 >= parts.len() {
        return None;
    }
    
    let mac = parts[at_idx + 1].to_string();
    
    if !mac.contains(':') {
        return None;
    }
    
    Some(ArpEntry { ip, mac })
}

/// Parse ARP table from Windows `arp -a` output
#[cfg(target_os = "windows")]
async fn get_arp_table_windows() -> Result<Vec<ArpEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let output = timeout(
        Duration::from_secs(5),
        tokio::task::spawn_blocking(|| {
            Command::new("arp")
                .args(["-a"])
                .output()
        })
    ).await
    .map_err(|_| "Timeout getting ARP table")??
    .map_err(|e| format!("Failed to execute arp command: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to get ARP table".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    let mut in_interface = false;
    
    for line in stdout.lines() {
        // Skip header lines
        if line.contains("Interface:") {
            in_interface = true;
            continue;
        }
        
        if !in_interface {
            continue;
        }
        
        // Parse lines like:   192.168.1.1           a0-f3-c1-12-34-56     dynamic
        if let Some(entry) = parse_arp_line_windows(line) {
            entries.push(entry);
        }
    }
    
    Ok(entries)
}

#[cfg(target_os = "windows")]
fn parse_arp_line_windows(line: &str) -> Option<ArpEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return None;
    }
    
    let ip: Ipv4Addr = parts[0].parse().ok()?;
    
    // Windows uses dashes in MAC: a0-f3-c1-12-34-56
    let mac = parts[1].replace('-', ":").to_lowercase();
    
    // Skip invalid entries
    if mac == "ff:ff:ff:ff:ff:ff" || mac == "00:00:00:00:00:00" {
        return None;
    }
    
    Some(ArpEntry { ip, mac })
}

/// Send ARP request to populate ARP table for a specific IP
/// This is used to "ping" via ARP (works even if ICMP is blocked)
pub async fn arp_ping(ip: Ipv4Addr) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Send a ping to trigger ARP resolution
    let ip_str = ip.to_string();
    
    #[cfg(target_os = "macos")]
    let _output = timeout(
        Duration::from_secs(2),
        tokio::task::spawn_blocking(move || {
            Command::new("ping")
                .args(["-c", "1", "-W", "1", "-q", &ip_str])
                .output()
        })
    ).await;
    
    #[cfg(target_os = "linux")]
    let _output = timeout(
        Duration::from_secs(2),
        tokio::task::spawn_blocking(move || {
            Command::new("ping")
                .args(["-c", "1", "-W", "1", "-q", &ip_str])
                .output()
        })
    ).await;
    
    #[cfg(target_os = "windows")]
    let _output = timeout(
        Duration::from_secs(2),
        tokio::task::spawn_blocking(move || {
            Command::new("ping")
                .args(["-n", "1", "-w", "1000", &ip_str])
                .output()
        })
    ).await;
    
    // Wait a moment for ARP table to be updated
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Check ARP table for this IP
    let arp_table = get_arp_table().await?;
    
    if let Some(entry) = arp_table.iter().find(|e| e.ip == ip) {
        Ok(entry.mac.clone())
    } else {
        Err("No ARP response".into())
    }
}

#[cfg(test)]
mod tests {
    // ARP tests would go here
}
