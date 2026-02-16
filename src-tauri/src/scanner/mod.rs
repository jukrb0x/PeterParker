pub mod engine;
pub mod tcp;
pub mod http;

pub use engine::*;
pub use tcp::*;
pub use http::*;

use std::net::{Ipv4Addr, AddrParseError};

#[derive(Debug, Clone)]
pub struct IpRange {
    network: Ipv4Addr,
    prefix: u8,
}

impl IpRange {
    pub fn parse(s: &str) -> Result<Self, AddrParseError> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(AddrParseError);
        }
        
        let network: Ipv4Addr = parts[0].parse()?;
        let prefix: u8 = parts[1].parse().map_err(|_| AddrParseError)?;
        
        Ok(Self { network, prefix })
    }
    
    pub fn size(&self) -> u32 {
        2u32.pow(32 - self.prefix as u32)
    }
    
    pub fn hosts(&self) -> impl Iterator<Item = Ipv4Addr> {
        let base = u32::from(self.network);
        let host_count = 2u32.pow(32 - self.prefix as u32);
        
        (1..host_count.saturating_sub(1)).filter_map(move |offset| {
            // Skip network and broadcast addresses
            Some(Ipv4Addr::from(base + offset))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_range() {
        let range = IpRange::parse("192.168.1.0/24").unwrap();
        assert_eq!(range.size(), 256);
    }
}
