use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Global vendor lookup table loaded from manuf.txt
static VENDOR_DB: Lazy<RwLock<HashMap<String, VendorInfo>>> = Lazy::new(|| {
    let db = RwLock::new(HashMap::new());
    load_vendor_db(&db);
    db
});

/// Vendor information
#[derive(Debug, Clone)]
pub struct VendorInfo {
    /// Short name (8 chars max, for Wireshark compatibility)
    pub short_name: String,
    /// Full vendor name
    pub full_name: Option<String>,
}

/// Load vendor database from embedded manuf.txt
fn load_vendor_db(db: &RwLock<HashMap<String, VendorInfo>>) {
    // Try to load from embedded file (path is relative to this file location in src/fingerprint/)
    let manuf_data = include_str!("../../data/manuf.txt");
    
    let mut db = match db.write() {
        Ok(d) => d,
        Err(_) => return,
    };
    
    for line in manuf_data.lines() {
        // Skip comments and empty lines
        let line_str: &str = line;
        if line_str.starts_with('#') || line_str.trim().is_empty() {
            continue;
        }
        
        // Parse line: MAC\tShortName[\tFullName]
        let parts: Vec<&str> = line_str.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        
        let mac_prefix = parts[0];
        
        // Handle CIDR notation (e.g., 00:1B:C5:00:00:00/36)
        // For now, we only use the base prefix for lookup
        let mac_prefix = if let Some(slash_pos) = mac_prefix.find('/') {
            &mac_prefix[..slash_pos]
        } else {
            mac_prefix
        };
        
        // Normalize MAC prefix: remove colons, convert to uppercase
        let normalized = normalize_mac_prefix(mac_prefix);
        
        // Skip if too short (need at least 6 hex chars = 3 bytes)
        if normalized.len() < 6 {
            continue;
        }
        
        let short_name = parts[1].to_string();
        let full_name = if parts.len() > 2 && !parts[2].starts_with('#') {
            Some(parts[2].to_string())
        } else {
            None
        };
        
        // Store with different key lengths for flexible lookup
        // Key format: first 6 chars (3 bytes), optionally more
        let key = if normalized.len() >= 6 {
            &normalized[..6]
        } else {
            continue;
        };
        
        // Only insert if not already present (first match wins)
        if !db.contains_key(key) {
            db.insert(key.to_string(), VendorInfo {
                short_name,
                full_name,
            });
        }
        
        // Also store with longer prefix if available
        if normalized.len() >= 8 {
            let longer_key = &normalized[..8];
            if !db.contains_key(longer_key) {
                db.insert(longer_key.to_string(), VendorInfo {
                    short_name: parts[1].to_string(),
                    full_name: if parts.len() > 2 && !parts[2].starts_with('#') {
                        Some(parts[2].to_string())
                    } else {
                        None
                    },
                });
            }
        }
    }
}

/// Normalize MAC prefix: remove colons/dashes/dots, convert to uppercase
fn normalize_mac_prefix(mac: &str) -> String {
    mac.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<String>()
        .to_uppercase()
}

/// Lookup vendor by MAC address
/// Returns the vendor info if found (cloned for safe return)
pub fn lookup_vendor(mac: &str) -> Option<VendorInfo> {
    let normalized = normalize_mac_prefix(mac);
    
    if normalized.len() < 6 {
        return None;
    }
    
    let db = match VENDOR_DB.read() {
        Ok(d) => d,
        Err(_) => return None,
    };
    
    // Try full prefix first (more specific match)
    if normalized.len() >= 8 {
        let key = &normalized[..8];
        if let Some(vendor) = db.get(key) {
            return Some(vendor.clone());
        }
    }
    
    // Fall back to 3-byte OUI
    let key = &normalized[..6];
    db.get(key).cloned()
}

/// Lookup vendor by OUI (first 3 octets)
/// oui should be in format "XX:XX:XX" or "XXXXXX"
pub fn lookup_vendor_by_oui(oui: &str) -> Option<VendorInfo> {
    let normalized = normalize_mac_prefix(oui);
    
    if normalized.len() < 6 {
        return None;
    }
    
    let db = match VENDOR_DB.read() {
        Ok(d) => d,
        Err(_) => return None,
    };
    
    let key = &normalized[..6];
    db.get(key).cloned()
}

/// Get the OUI (first 3 octets) from a MAC address
pub fn get_oui(mac: &str) -> String {
    mac.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .take(6)
        .collect::<String>()
        .to_uppercase()
}

/// Get formatted OUI with colons (XX:XX:XX format)
pub fn get_oui_formatted(mac: &str) -> String {
    let oui = get_oui(mac);
    format!("{}:{}:{}", &oui[..2], &oui[2..4], &oui[4..6])
}

/// Get the number of vendors loaded
pub fn vendor_count() -> usize {
    match VENDOR_DB.read() {
        Ok(db) => db.len(),
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_mac_prefix() {
        assert_eq!(normalize_mac_prefix("00:1A:2B"), "001A2B");
        assert_eq!(normalize_mac_prefix("00-1A-2B-3C-4D-5E"), "001A2B3C4D5E");
        assert_eq!(normalize_mac_prefix("001a.2b3c.4d5e"), "001A2B3C4D5E");
    }
    
    #[test]
    fn test_get_oui() {
        assert_eq!(get_oui("A0:F3:C1:12:34:56"), "A0F3C1");
        assert_eq!(get_oui("00:1B:63:AB:CD:EF"), "001B63");
    }
    
    #[test]
    fn test_get_oui_formatted() {
        assert_eq!(get_oui_formatted("a0:f3:c1:12:34:56"), "A0:F3:C1");
    }
    
    #[test]
    fn test_vendor_count() {
        // Should have loaded thousands of vendors
        assert!(vendor_count() > 10000);
    }
    
    #[test]
    fn test_lookup_vendor() {
        // Test some known vendors
        // Apple
        let apple = lookup_vendor("00:1B:63:AB:CD:EF");
        assert!(apple.is_some());
        
        // Check that we get a result
        let vendor = lookup_vendor("00:00:00");
        assert!(vendor.is_some());
    }
}
