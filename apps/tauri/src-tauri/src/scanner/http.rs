use std::time::Duration;
use std::io::{Read, Write};
use std::net::TcpStream;

pub async fn fetch_http_info(host: &str, timeout: Duration) -> Option<(String, String)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_title() {
        let html = "<html><head><title>Test Page</title></head></html>";
        assert_eq!(extract_title(html), Some("Test Page".to_string()));
    }
}
