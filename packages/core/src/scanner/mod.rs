pub mod engine;
pub mod tcp;
pub mod http;
pub mod arp;

pub use engine::*;
pub use tcp::*;
pub use http::*;
pub use arp::*;

use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct IpRange {
    network: Ipv4Addr,
    prefix: u8,
}

#[derive(Error, Debug)]
pub enum IpRangeError {
    #[error("invalid IP range format")]
    InvalidFormat,
    #[error("invalid IP address")]
    InvalidIp(#[from] std::net::AddrParseError),
    #[error("invalid prefix")]
    InvalidPrefix,
}

impl IpRange {
    pub fn parse(s: &str) -> Result<Self, IpRangeError> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(IpRangeError::InvalidFormat);
        }
        
        let network: Ipv4Addr = parts[0].parse()?;
        let prefix: u8 = parts[1].parse().map_err(|_| IpRangeError::InvalidPrefix)?;
        
        if prefix > 32 {
            return Err(IpRangeError::InvalidPrefix);
        }
        
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
    
    #[test]
    fn test_parse_range_invalid() {
        assert!(IpRange::parse("192.168.1.0").is_err());
        assert!(IpRange::parse("invalid").is_err());
        assert!(IpRange::parse("192.168.1.0/33").is_err());
    }
    
    #[test]
    fn test_hosts() {
        let range = IpRange::parse("192.168.1.0/24").unwrap();
        let hosts: Vec<_> = range.hosts().take(5).collect();
        assert_eq!(hosts[0], Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(hosts[4], Ipv4Addr::new(192, 168, 1, 5));
    }
}
