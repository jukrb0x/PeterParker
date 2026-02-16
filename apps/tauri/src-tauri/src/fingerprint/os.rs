use crate::models::OperatingSystem;

/// Detect OS based on TTL and other TCP/IP stack characteristics
pub fn detect_os_from_ttl(ttl: u8) -> OperatingSystem {
    // Common TTL values:
    // Linux: 64
    // Windows: 128
    // macOS: 64
    // Cisco: 255
    // Solaris: 255
    
    let (name, family, confidence) = match ttl {
        0..=64 => ("Linux/macOS", "linux", 70),
        65..=128 => ("Windows", "windows", 85),
        129..=255 => ("Network Device", "embedded", 60),
        _ => ("Unknown", "unknown", 0),
    };
    
    OperatingSystem {
        name: name.to_string(),
        family: family.to_string(),
        version: None,
        confidence,
        cpe: vec![],
    }
}

/// Detect OS from HTTP headers and banner
pub fn detect_os_from_http_headers(server: &str, title: &str) -> Option<OperatingSystem> {
    let server_lower = server.to_lowercase();
    
    if server_lower.contains("microsoft-iis") {
        let version = server_lower
            .split('/')
            .nth(1)
            .map(|v| v.split('.').next().unwrap_or(""))
            .unwrap_or("")
            .to_string();
        
        return Some(OperatingSystem {
            name: format!("Windows Server {}", version),
            family: "windows".to_string(),
            version: Some(version),
            confidence: 80,
            cpe: vec![],
        });
    }
    
    if server_lower.contains("nginx") || server_lower.contains("apache") {
        return Some(OperatingSystem {
            name: "Linux/Unix".to_string(),
            family: "linux".to_string(),
            version: None,
            confidence: 60,
            cpe: vec![],
        });
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_linux() {
        let os = detect_os_from_ttl(64);
        assert_eq!(os.family, "linux");
    }
    
    #[test]
    fn test_detect_windows() {
        let os = detect_os_from_ttl(128);
        assert_eq!(os.family, "windows");
    }
}
