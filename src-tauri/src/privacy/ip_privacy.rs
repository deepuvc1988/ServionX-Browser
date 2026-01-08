// IP Privacy Module
// Generates fake IP headers and prevents leaks

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Fake IP headers for privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FakeIpHeaders {
    pub x_forwarded_for: String,
    pub x_real_ip: String,
    pub client_ip: String,
    pub cf_connecting_ip: String,
    pub true_client_ip: String,
}

/// IP ranges for fake IP generation (public IPs from various regions)
#[derive(Debug)]
struct IpRange {
    start: [u8; 4],
    end: [u8; 4],
    region: &'static str,
}

#[derive(Debug)]
pub struct IpPrivacy {
    ip_ranges: Vec<IpRange>,
}

impl IpPrivacy {
    pub fn new() -> Self {
        Self {
            ip_ranges: vec![
                // US IP ranges
                IpRange { start: [45, 0, 0, 0], end: [45, 255, 255, 255], region: "US" },
                IpRange { start: [104, 0, 0, 0], end: [104, 255, 255, 255], region: "US" },
                IpRange { start: [172, 64, 0, 0], end: [172, 71, 255, 255], region: "US" },
                IpRange { start: [98, 0, 0, 0], end: [98, 127, 255, 255], region: "US" },
                
                // EU IP ranges
                IpRange { start: [185, 0, 0, 0], end: [185, 255, 255, 255], region: "EU" },
                IpRange { start: [195, 0, 0, 0], end: [195, 255, 255, 255], region: "EU" },
                IpRange { start: [212, 0, 0, 0], end: [212, 127, 255, 255], region: "EU" },
                
                // Asia IP ranges
                IpRange { start: [203, 0, 0, 0], end: [203, 255, 255, 255], region: "ASIA" },
                IpRange { start: [110, 0, 0, 0], end: [110, 255, 255, 255], region: "ASIA" },
                IpRange { start: [175, 0, 0, 0], end: [175, 255, 255, 255], region: "ASIA" },
                
                // Australia IP ranges
                IpRange { start: [101, 0, 0, 0], end: [101, 127, 255, 255], region: "AU" },
                IpRange { start: [202, 0, 0, 0], end: [202, 127, 255, 255], region: "AU" },
            ],
        }
    }
    
    /// Generate fake IP headers
    pub fn generate_headers(&self) -> FakeIpHeaders {
        let fake_ip = self.generate_random_ip();
        
        FakeIpHeaders {
            x_forwarded_for: fake_ip.clone(),
            x_real_ip: fake_ip.clone(),
            client_ip: fake_ip.clone(),
            cf_connecting_ip: fake_ip.clone(),
            true_client_ip: fake_ip,
        }
    }
    
    /// Generate a random public IP address
    fn generate_random_ip(&self) -> String {
        let mut rng = rand::thread_rng();
        
        // Select a random IP range
        let range = &self.ip_ranges[rng.gen_range(0..self.ip_ranges.len())];
        
        // Generate IP within range
        let octets: Vec<u8> = (0..4)
            .map(|i| rng.gen_range(range.start[i]..=range.end[i]))
            .collect();
        
        format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
    
    /// Generate fake IP for a specific region
    pub fn generate_ip_for_region(&self, region: &str) -> Option<String> {
        let mut rng = rand::thread_rng();
        
        let matching: Vec<_> = self.ip_ranges
            .iter()
            .filter(|r| r.region.eq_ignore_ascii_case(region))
            .collect();
        
        if matching.is_empty() {
            return None;
        }
        
        let range = matching[rng.gen_range(0..matching.len())];
        
        let octets: Vec<u8> = (0..4)
            .map(|i| rng.gen_range(range.start[i]..=range.end[i]))
            .collect();
        
        Some(format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]))
    }
}

impl Default for IpPrivacy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    #[test]
    fn test_ip_generation() {
        let ip_privacy = IpPrivacy::new();
        let headers = ip_privacy.generate_headers();
        
        // Verify IP is valid
        assert!(headers.x_real_ip.parse::<Ipv4Addr>().is_ok());
        
        // Verify all headers match
        assert_eq!(headers.x_forwarded_for, headers.x_real_ip);
        assert_eq!(headers.client_ip, headers.x_real_ip);
    }
    
    #[test]
    fn test_region_ip_generation() {
        let ip_privacy = IpPrivacy::new();
        let ip = ip_privacy.generate_ip_for_region("US").unwrap();
        
        assert!(ip.parse::<Ipv4Addr>().is_ok());
    }
}
