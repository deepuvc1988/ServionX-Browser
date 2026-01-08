// Network Tools
// Ping, port scanning, DNS lookup, HTTP testing

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Ping result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub host: String,
    pub ip: Option<String>,
    pub reachable: bool,
    pub latency_ms: Option<u64>,
    pub packets_sent: u32,
    pub packets_received: u32,
    pub packet_loss_percent: f32,
}

/// Port scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub host: String,
    pub port: u16,
    pub open: bool,
    pub service: Option<String>,
    pub latency_ms: Option<u64>,
}

/// DNS lookup result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResult {
    pub domain: String,
    pub addresses: Vec<String>,
    pub lookup_time_ms: u64,
    pub record_type: String,
}

/// HTTP request result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResult {
    pub url: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub latency_ms: u64,
    pub content_length: Option<u64>,
}

/// Network diagnostic tools
pub struct NetworkTools;

impl NetworkTools {
    pub fn new() -> Self {
        Self
    }
    
    /// Ping a host
    pub async fn ping(&self, host: &str, count: u32) -> PingResult {
        use std::process::Command;
        
        let start = Instant::now();
        
        // Use system ping command
        #[cfg(target_os = "windows")]
        let output = Command::new("ping")
            .args(["-n", &count.to_string(), host])
            .output();
        
        #[cfg(not(target_os = "windows"))]
        let output = Command::new("ping")
            .args(["-c", &count.to_string(), host])
            .output();
        
        let elapsed = start.elapsed().as_millis() as u64;
        
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let success = out.status.success();
                
                // Try to resolve the IP
                let ip = self.resolve_host(host).await.ok();
                
                // Parse packet loss (simplified)
                let packet_loss = if stdout.contains("100% packet loss") || stdout.contains("100.0% packet loss") {
                    100.0
                } else if stdout.contains("0% packet loss") || stdout.contains("0.0% packet loss") {
                    0.0
                } else {
                    // Try to find packet loss percentage
                    50.0 // Default estimate
                };
                
                let received = ((100.0 - packet_loss) / 100.0 * count as f32) as u32;
                
                PingResult {
                    host: host.to_string(),
                    ip,
                    reachable: success && packet_loss < 100.0,
                    latency_ms: if success { Some(elapsed / count as u64) } else { None },
                    packets_sent: count,
                    packets_received: received,
                    packet_loss_percent: packet_loss,
                }
            }
            Err(_) => PingResult {
                host: host.to_string(),
                ip: None,
                reachable: false,
                latency_ms: None,
                packets_sent: count,
                packets_received: 0,
                packet_loss_percent: 100.0,
            },
        }
    }
    
    /// Scan a single port
    pub fn scan_port(&self, host: &str, port: u16, timeout_ms: u64) -> PortScanResult {
        let start = Instant::now();
        
        let addr = format!("{}:{}", host, port);
        let socket_addr: Result<SocketAddr, _> = addr.parse();
        
        let (open, latency) = if let Ok(addr) = socket_addr {
            match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
                Ok(_) => (true, Some(start.elapsed().as_millis() as u64)),
                Err(_) => (false, None),
            }
        } else {
            // Try DNS resolution
            match std::net::ToSocketAddrs::to_socket_addrs(&addr) {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
                            Ok(_) => (true, Some(start.elapsed().as_millis() as u64)),
                            Err(_) => (false, None),
                        }
                    } else {
                        (false, None)
                    }
                }
                Err(_) => (false, None),
            }
        };
        
        PortScanResult {
            host: host.to_string(),
            port,
            open,
            service: self.get_common_service(port),
            latency_ms: latency,
        }
    }
    
    /// Scan multiple ports
    pub fn scan_ports(&self, host: &str, ports: &[u16], timeout_ms: u64) -> Vec<PortScanResult> {
        ports.iter()
            .map(|&port| self.scan_port(host, port, timeout_ms))
            .collect()
    }
    
    /// DNS lookup
    pub async fn dns_lookup(&self, domain: &str) -> Result<DnsResult, String> {
        let start = Instant::now();
        
        let addresses = self.resolve_all_hosts(domain).await?;
        
        Ok(DnsResult {
            domain: domain.to_string(),
            addresses,
            lookup_time_ms: start.elapsed().as_millis() as u64,
            record_type: "A/AAAA".to_string(),
        })
    }
    
    /// Resolve a hostname to a single IP
    async fn resolve_host(&self, host: &str) -> Result<String, String> {
        use std::net::ToSocketAddrs;
        
        let addr = format!("{}:80", host);
        match addr.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    Ok(addr.ip().to_string())
                } else {
                    Err("No addresses found".to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
    
    /// Resolve a hostname to all IPs
    async fn resolve_all_hosts(&self, host: &str) -> Result<Vec<String>, String> {
        use std::net::ToSocketAddrs;
        
        let addr = format!("{}:80", host);
        match addr.to_socket_addrs() {
            Ok(addrs) => Ok(addrs.map(|a| a.ip().to_string()).collect()),
            Err(e) => Err(e.to_string()),
        }
    }
    
    /// Make an HTTP request
    pub async fn http_request(
        &self,
        url: &str,
        method: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<HttpResult, String> {
        let start = Instant::now();
        
        let client = reqwest::Client::new();
        
        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "HEAD" => client.head(url),
            "PATCH" => client.patch(url),
            _ => return Err(format!("Unsupported method: {}", method)),
        };
        
        // Add custom headers
        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                request = request.header(&key, &value);
            }
        }
        
        // Add body
        if let Some(b) = body {
            request = request.body(b);
        }
        
        let response = request.send().await
            .map_err(|e| e.to_string())?;
        
        let status_code = response.status().as_u16();
        let status_text = response.status().canonical_reason()
            .unwrap_or("Unknown")
            .to_string();
        
        let content_length = response.content_length();
        
        // Convert headers
        let headers: HashMap<String, String> = response.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        
        let body = response.text().await.ok();
        
        Ok(HttpResult {
            url: url.to_string(),
            status_code,
            status_text,
            headers,
            body,
            latency_ms: start.elapsed().as_millis() as u64,
            content_length,
        })
    }
    
    /// Get common service name for a port
    fn get_common_service(&self, port: u16) -> Option<String> {
        match port {
            20 => Some("FTP Data".to_string()),
            21 => Some("FTP".to_string()),
            22 => Some("SSH".to_string()),
            23 => Some("Telnet".to_string()),
            25 => Some("SMTP".to_string()),
            53 => Some("DNS".to_string()),
            80 => Some("HTTP".to_string()),
            110 => Some("POP3".to_string()),
            143 => Some("IMAP".to_string()),
            443 => Some("HTTPS".to_string()),
            445 => Some("SMB".to_string()),
            993 => Some("IMAPS".to_string()),
            995 => Some("POP3S".to_string()),
            1433 => Some("MSSQL".to_string()),
            1521 => Some("Oracle".to_string()),
            3306 => Some("MySQL".to_string()),
            3389 => Some("RDP".to_string()),
            5432 => Some("PostgreSQL".to_string()),
            5900 => Some("VNC".to_string()),
            6379 => Some("Redis".to_string()),
            8080 => Some("HTTP Proxy".to_string()),
            8443 => Some("HTTPS Alt".to_string()),
            27017 => Some("MongoDB".to_string()),
            _ => None,
        }
    }
}

impl Default for NetworkTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_port_service_lookup() {
        let tools = NetworkTools::new();
        assert_eq!(tools.get_common_service(22), Some("SSH".to_string()));
        assert_eq!(tools.get_common_service(443), Some("HTTPS".to_string()));
        assert_eq!(tools.get_common_service(12345), None);
    }
}
