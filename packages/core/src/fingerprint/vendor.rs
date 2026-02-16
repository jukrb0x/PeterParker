use std::collections::HashMap;
use once_cell::sync::Lazy;

/// MAC address vendor lookup
/// First 3 octets (OUI) -> Vendor name
static VENDOR_DB: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // Common vendors (partial list - full database should be loaded from file)
    m.insert("00:1A:2B", "TP-Link");
    m.insert("00:1B:2B", "TP-Link");
    m.insert("A0:F3:C1", "TP-Link");
    m.insert("50:C7:BF", "TP-Link");
    m.insert("14:CF:92", "TP-Link");
    
    m.insert("00:03:93", "Apple");
    m.insert("00:05:02", "Apple");
    m.insert("00:0A:27", "Apple");
    m.insert("00:0A:95", "Apple");
    m.insert("00:0D:93", "Apple");
    m.insert("00:0F:24", "Apple");
    m.insert("00:10:FA", "Apple");
    m.insert("00:11:24", "Apple");
    m.insert("00:14:51", "Apple");
    m.insert("00:16:CB", "Apple");
    m.insert("00:17:F2", "Apple");
    m.insert("00:19:E3", "Apple");
    m.insert("00:1B:63", "Apple");
    m.insert("00:1C:B3", "Apple");
    m.insert("00:1D:4F", "Apple");
    m.insert("00:1E:52", "Apple");
    m.insert("00:1F:5B", "Apple");
    m.insert("00:1F:F3", "Apple");
    m.insert("00:21:E9", "Apple");
    m.insert("00:22:41", "Apple");
    m.insert("00:23:12", "Apple");
    m.insert("00:23:6C", "Apple");
    m.insert("00:23:DF", "Apple");
    m.insert("00:24:36", "Apple");
    m.insert("00:25:00", "Apple");
    m.insert("00:25:4B", "Apple");
    m.insert("00:25:BC", "Apple");
    m.insert("00:26:08", "Apple");
    m.insert("00:26:4A", "Apple");
    m.insert("00:26:B0", "Apple");
    m.insert("00:26:BB", "Apple");
    m.insert("00:30:65", "Apple");
    m.insert("00:50:E4", "Apple");
    m.insert("00:A0:40", "Apple");
    m.insert("00:A0:3B", "Apple");
    m.insert("04:0C:CE", "Apple");
    m.insert("04:1E:64", "Apple");
    m.insert("04:26:18", "Apple");
    m.insert("04:4B:ED", "Apple");
    m.insert("04:54:53", "Apple");
    m.insert("04:5E:60", "Apple");
    m.insert("04:DB:56", "Apple");
    m.insert("04:E5:36", "Apple");
    m.insert("08:00:07", "Apple");
    m.insert("08:00:07", "Apple");
    m.insert("10:1C:0C", "Apple");
    m.insert("10:41:7F", "Apple");
    m.insert("10:6B:60", "Apple");
    m.insert("10:9A:DD", "Apple");
    m.insert("10:B5:5E", "Apple");
    m.insert("10:DD:B1", "Apple");
    m.insert("14:10:9F", "Apple");
    m.insert("14:20:5E", "Apple");
    m.insert("14:5A:05", "Apple");
    m.insert("14:7D:DA", "Apple");
    m.insert("14:99:E2", "Apple");
    m.insert("14:A3:2F", "Apple");
    m.insert("14:BD:61", "Apple");
    m.insert("14:CF:21", "Apple");
    m.insert("14:D3:15", "Apple");
    m.insert("14:DD:A9", "Apple");
    m.insert("18:34:51", "Apple");
    m.insert("18:65:90", "Apple");
    m.insert("18:78:3E", "Apple");
    m.insert("18:AF:05", "Apple");
    m.insert("18:B8:78", "Apple");
    m.insert("18:E0:6B", "Apple");
    m.insert("18:E2:2E", "Apple");
    m.insert("18:F0:77", "Apple");
    m.insert("18:F6:43", "Apple");
    m.insert("1C:1A:C0", "Apple");
    m.insert("1C:36:BB", "Apple");
    m.insert("1C:4D:70", "Apple");
    m.insert("1C:56:FE", "Apple");
    m.insert("1C:6B:76", "Apple");
    m.insert("1C:72:97", "Apple");
    m.insert("1C:9E:87", "Apple");
    m.insert("1C:AB:A7", "Apple");
    m.insert("1C:B7:2C", "Apple");
    m.insert("1C:B8:6E", "Apple");
    m.insert("1C:BA:8C", "Apple");
    m.insert("1C:D9:38", "Apple");
    m.insert("20:3C:AE", "Apple");
    m.insert("20:52:45", "Apple");
    m.insert("20:7B:D2", "Apple");
    m.insert("20:AB:37", "Apple");
    m.insert("20:C9:D0", "Apple");
    m.insert("20:E5:2A", "Apple");
    m.insert("20:F3:A3", "Apple");
    m.insert("24:00:BA", "Apple");
    m.insert("24:07:2F", "Apple");
    m.insert("24:1E:F4", "Apple");
    m.insert("24:24:88", "Apple");
    m.insert("24:4B:03", "Apple");
    m.insert("24:62:AC", "Apple");
    m.insert("24:72:3C", "Apple");
    m.insert("24:A0:74", "Apple");
    m.insert("24:A2:E1", "Apple");
    m.insert("24:A4:3C", "Apple");
    m.insert("24:A6:6F", "Apple");
    m.insert("24:A8:02", "Apple");
    m.insert("24:AB:81", "Apple");
    m.insert("24:B6:FD", "Apple");
    m.insert("24:D0:ED", "Apple");
    m.insert("24:D7:6F", "Apple");
    m.insert("24:E2:31", "Apple");
    m.insert("24:E3:14", "Apple");
    m.insert("24:F6:77", "Apple");
    
    // Xiaomi / Redmi
    m.insert("00:0E:A6", "Xiaomi");
    m.insert("28:ED:6A", "Xiaomi");
    m.insert("34:80:B3", "Xiaomi");
    m.insert("3C:8B:FE", "Xiaomi");
    m.insert("44:EA:D8", "Xiaomi");
    m.insert("48:BF:6B", "Xiaomi");
    m.insert("50:EC:50", "Xiaomi");
    m.insert("64:09:80", "Xiaomi");
    m.insert("64:70:33", "Xiaomi");
    m.insert("64:BC:0C", "Xiaomi");
    m.insert("6C:5C:14", "Xiaomi");
    m.insert("6C:8D:C1", "Xiaomi");
    m.insert("74:A3:E4", "Xiaomi");
    m.insert("78:02:F8", "Xiaomi");
    m.insert("7C:1E:52", "Xiaomi");
    m.insert("7C:2B:35", "Xiaomi");
    m.insert("7C:8B:CA", "Xiaomi");
    m.insert("88:C6:26", "Xiaomi");
    m.insert("8C:BE:BE", "Xiaomi");
    m.insert("8C:F9:49", "Xiaomi");
    m.insert("90:F6:52", "Xiaomi");
    m.insert("9C:2E:A1", "Xiaomi");
    m.insert("9C:99:A0", "Xiaomi");
    m.insert("AC:F7:F3", "Xiaomi");
    m.insert("B0:E2:35", "Xiaomi");
    m.insert("B4:6B:FC", "Xiaomi");
    m.insert("C0:EE:FB", "Xiaomi");
    m.insert("C4:0B:CB", "Xiaomi");
    m.insert("C4:B1:6C", "Xiaomi");
    m.insert("C8:1D:96", "Xiaomi");
    m.insert("C8:7B:2B", "Xiaomi");
    m.insert("CC:AF:78", "Xiaomi");
    m.insert("D0:88:35", "Xiaomi");
    m.insert("D4:EE:07", "Xiaomi");
    m.insert("D8:63:75", "Xiaomi");
    m.insert("DC:7F:76", "Xiaomi");
    m.insert("E0:D9:E0", "Xiaomi");
    m.insert("E4:46:DA", "Xiaomi");
    m.insert("E8:50:8B", "Xiaomi");
    m.insert("EC:26:CA", "Xiaomi");
    m.insert("EC:88:8F", "Xiaomi");
    m.insert("F0:B4:29", "Xiaomi");
    m.insert("F0:F6:1C", "Xiaomi");
    m.insert("F8:A4:5F", "Xiaomi");
    m.insert("FC:64:BA", "Xiaomi");
    m.insert("FC:F5:28", "Xiaomi");
    
    // Huawei
    m.insert("00:18:82", "Huawei");
    m.insert("00:1A:A1", "Huawei");
    m.insert("00:1E:10", "Huawei");
    m.insert("00:25:68", "Huawei");
    m.insert("08:19:A6", "Huawei");
    m.insert("0C:96:BF", "Huawei");
    m.insert("10:1D:8C", "Huawei");
    m.insert("10:3D:1C", "Huawei");
    m.insert("18:27:2C", "Huawei");
    m.insert("1C:5A:3E", "Huawei");
    m.insert("20:08:ED", "Huawei");
    m.insert("20:4E:7F", "Huawei");
    m.insert("24:09:95", "Huawei");
    m.insert("24:40:BB", "Huawei");
    m.insert("28:2C:B2", "Huawei");
    m.insert("2C:FD:FE", "Huawei");
    m.insert("30:07:4D", "Huawei");
    m.insert("30:9E:E4", "Huawei");
    m.insert("34:1C:57", "Huawei");
    m.insert("34:A3:95", "Huawei");
    m.insert("38:BC:1A", "Huawei");
    m.insert("3C:FA:43", "Huawei");
    m.insert("44:5F:E7", "Huawei");
    m.insert("48:7D:2B", "Huawei");
    m.insert("4C:EA:72", "Huawei");
    m.insert("54:95:70", "Huawei");
    m.insert("58:00:E3", "Huawei");
    m.insert("5C:8D:4E", "Huawei");
    m.insert("60:6B:BD", "Huawei");
    m.insert("64:5D:86", "Huawei");
    m.insert("68:A8:6D", "Huawei");
    m.insert("70:19:0D", "Huawei");
    m.insert("70:54:D2", "Huawei");
    m.insert("74:28:41", "Huawei");
    m.insert("78:25:A3", "Huawei");
    m.insert("78:F5:FD", "Huawei");
    m.insert("7C:1E:52", "Huawei");
    m.insert("7C:A1:77", "Huawei");
    m.insert("7C:D3:F3", "Huawei");
    m.insert("80:4A:14", "Huawei");
    m.insert("84:1E:BF", "Huawei");
    m.insert("88:25:93", "Huawei");
    m.insert("88:E3:AB", "Huawei");
    m.insert("8C:71:F8", "Huawei");
    m.insert("8C:BE:BE", "Huawei");
    m.insert("90:0D:B2", "Huawei");
    m.insert("90:2B:D2", "Huawei");
    m.insert("94:87:26", "Huawei");
    m.insert("98:40:BB", "Huawei");
    m.insert("9C:B6:D0", "Huawei");
    m.insert("A0:8F:73", "Huawei");
    m.insert("A4:C3:F0", "Huawei");
    m.insert("A8:9B:52", "Huawei");
    m.insert("AC:23:3F", "Huawei");
    m.insert("B0:CF:CF", "Huawei");
    m.insert("B4:FB:E4", "Huawei");
    m.insert("B8:BC:1B", "Huawei");
    m.insert("BC:CF:CC", "Huawei");
    m.insert("C0:61:AE", "Huawei");
    m.insert("C4:04:15", "Huawei");
    m.insert("C8:79:6F", "Huawei");
    m.insert("C8:85:50", "Huawei");
    m.insert("CC:B1:1A", "Huawei");
    m.insert("D0:77:14", "Huawei");
    m.insert("D4:7B:B0", "Huawei");
    m.insert("D8:2B:C5", "Huawei");
    m.insert("DC:08:0F", "Huawei");
    m.insert("DC:2B:61", "Huawei");
    m.insert("E0:24:7F", "Huawei");
    m.insert("E4:58:B5", "Huawei");
    m.insert("E8:4E:CE", "Huawei");
    m.insert("EC:6C:A5", "Huawei");
    m.insert("F0:72:6A", "Huawei");
    m.insert("F4:45:82", "Huawei");
    m.insert("F8:15:D4", "Huawei");
    m.insert("FC:00:BF", "Huawei");
    m.insert("FC:3D:93", "Huawei");
    
    // Samsung
    m.insert("00:00:F0", "Samsung");
    m.insert("00:07:AB", "Samsung");
    m.insert("00:0D:E5", "Samsung");
    m.insert("00:12:FB", "Samsung");
    m.insert("00:13:77", "Samsung");
    m.insert("00:15:99", "Samsung");
    m.insert("00:16:6B", "Samsung");
    m.insert("00:17:C4", "Samsung");
    m.insert("00:18:45", "Samsung");
    m.insert("00:1A:8A", "Samsung");
    m.insert("00:1B:59", "Samsung");
    m.insert("00:1D:29", "Samsung");
    m.insert("00:1E:7D", "Samsung");
    m.insert("00:1F:CE", "Samsung");
    m.insert("00:22:15", "Samsung");
    m.insert("00:23:39", "Samsung");
    m.insert("00:24:90", "Samsung");
    m.insert("00:25:38", "Samsung");
    m.insert("00:26:37", "Samsung");
    m.insert("00:26:82", "Samsung");
    m.insert("00:4F:28", "Samsung");
    m.insert("04:46:04", "Samsung");
    m.insert("08:00:27", "Samsung");
    m.insert("08:66:98", "Samsung");
    m.insert("08:AE:0D", "Samsung");
    m.insert("0C:48:85", "Samsung");
    m.insert("0C:89:10", "Samsung");
    m.insert("0C:B4:7A", "Samsung");
    m.insert("0C:D2:B5", "Samsung");
    m.insert("0C:D9:26", "Samsung");
    m.insert("10:08:C1", "Samsung");
    m.insert("10:20:6E", "Samsung");
    m.insert("10:68:3F", "Samsung");
    m.insert("10:B1:DF", "Samsung");
    m.insert("10:BF:48", "Samsung");
    m.insert("10:D5:42", "Samsung");
    m.insert("14:1A:A3", "Samsung");
    m.insert("14:5F:94", "Samsung");
    m.insert("14:A8:67", "Samsung");
    m.insert("14:AB:F5", "Samsung");
    m.insert("14:DE:5E", "Samsung");
    m.insert("18:4F:32", "Samsung");
    m.insert("18:AF:F9", "Samsung");
    m.insert("18:E1:2B", "Samsung");
    m.insert("1C:4B:0D", "Samsung");
    m.insert("1C:62:B8", "Samsung");
    m.insert("1C:91:80", "Samsung");
    m.insert("1C:DC:8E", "Samsung");
    m.insert("20:02:AF", "Samsung");
    m.insert("20:3E:90", "Samsung");
    m.insert("20:59:4F", "Samsung");
    m.insert("20:A5:F5", "Samsung");
    m.insert("20:C8:2A", "Samsung");
    m.insert("20:D3:90", "Samsung");
    m.insert("24:4B:81", "Samsung");
    m.insert("24:7D:5E", "Samsung");
    m.insert("24:A2:E1", "Samsung");
    m.insert("24:E3:14", "Samsung");
    m.insert("28:1A:4A", "Samsung");
    m.insert("28:3A:4D", "Samsung");
    m.insert("28:5E:D9", "Samsung");
    m.insert("28:67:51", "Samsung");
    m.insert("28:79:61", "Samsung");
    m.insert("28:A1:83", "Samsung");
    m.insert("28:C2:DD", "Samsung");
    m.insert("28:C6:8E", "Samsung");
    m.insert("28:ED:6A", "Samsung");
    m.insert("28:F0:76", "Samsung");
    m.insert("2C:0E:3D", "Samsung");
    m.insert("2C:3B:6F", "Samsung");
    m.insert("2C:44:FD", "Samsung");
    m.insert("2C:54:91", "Samsung");
    m.insert("2C:68:37", "Samsung");
    m.insert("2C:8A:72", "Samsung");
    m.insert("2C:9C:A1", "Samsung");
    m.insert("2C:BE:08", "Samsung");
    m.insert("2C:BE:97", "Samsung");
    m.insert("2C:D6:1D", "Samsung");
    
    m
});

/// Look up vendor from MAC address
pub fn lookup_vendor(mac: &str) -> Option<&'static str> {
    // Normalize MAC to uppercase with colons
    let normalized = mac
        .to_uppercase()
        .replace('-', ":')
        .replace('.', ":");
    
    // Extract first 3 octets
    let parts: Vec<&str> = normalized.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    
    let oui = format!("{}:{}:{}", parts[0], parts[1], parts[2]);
    
    VENDOR_DB.get(oui.as_str()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lookup_apple() {
        let vendor = lookup_vendor("00:1B:63:XX:XX:XX");
        assert_eq!(vendor, Some("Apple"));
    }
    
    #[test]
    fn test_lookup_xiaomi() {
        let vendor = lookup_vendor("28:ED:6A:XX:XX:XX");
        assert_eq!(vendor, Some("Xiaomi"));
    }
}
