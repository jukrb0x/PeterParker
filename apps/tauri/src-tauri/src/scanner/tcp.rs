use std::time::Duration;
use std::net::{TcpStream, ToSocketAddrs};

use crate::models::Port;

pub async fn tcp_scan_ports(host: &str, ports: &[u16], timeout: Duration) -> Vec<Port> {
    let mut open_ports = Vec::new();
    
    for port in ports {
        let addr = format!("{}:{}", host, port);
        
        match tokio::time::timeout(
            timeout,
            tokio::task::spawn_blocking(move || {
                TcpStream::connect_timeout(
                    &addr.to_socket_addrs()?.next().unwrap(),
                    timeout,
                )
            })
        ).await {
            Ok(Ok(Ok(_))) => {
                open_ports.push(Port::open_tcp(*port));
            }
            _ => continue,
        }
    }
    
    open_ports
}

pub async fn grab_banner(host: &str, port: u16, timeout: Duration) -> Option<String> {
    let addr = format!("{}:{}", host, port);
    
    let result = tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || {
            let mut stream = TcpStream::connect_timeout(
                &addr.to_socket_addrs()?.next()?,
                timeout,
            )?;
            
            use std::io::{Read, Write};
            
            // Send HTTP request for HTTP ports
            if port == 80 || port == 8080 || port == 443 {
                let request = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", host);
                stream.write_all(request.as_bytes()).ok()?;
            }
            
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).ok()?;
            Some(String::from_utf8_lossy(&buf[..n]).to_string())
        })
    ).await;
    
    match result {
        Ok(Ok(Some(banner))) => Some(banner),
        _ => None,
    }
}
