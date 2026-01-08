// HTTPS Enforcer
// Forces HTTPS connections and blocks insecure HTTP

use std::sync::RwLock;

/// HTTPS enforcement modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpsMode {
    /// Always require HTTPS (block HTTP entirely)
    Strict,
    /// Upgrade to HTTPS if available, warn on HTTP
    Upgrade,
    /// Allow all connections (no enforcement)
    Disabled,
}

/// HTTPS Enforcer that ensures secure connections
pub struct HttpsEnforcer {
    mode: RwLock<HttpsMode>,
    upgraded_count: RwLock<u64>,
    blocked_count: RwLock<u64>,
}

impl HttpsEnforcer {
    pub fn new() -> Self {
        Self {
            mode: RwLock::new(HttpsMode::Upgrade),
            upgraded_count: RwLock::new(0),
            blocked_count: RwLock::new(0),
        }
    }
    
    /// Process a URL and return the enforced URL
    /// Returns None if the URL should be blocked
    pub fn process_url(&self, url: &str) -> Option<String> {
        let mode = *self.mode.read().unwrap();
        
        match mode {
            HttpsMode::Disabled => Some(url.to_string()),
            
            HttpsMode::Upgrade => {
                if url.starts_with("http://") {
                    // Upgrade to HTTPS
                    let upgraded = url.replacen("http://", "https://", 1);
                    let mut count = self.upgraded_count.write().unwrap();
                    *count += 1;
                    log::info!("Upgraded to HTTPS: {}", upgraded);
                    Some(upgraded)
                } else {
                    Some(url.to_string())
                }
            }
            
            HttpsMode::Strict => {
                if url.starts_with("https://") {
                    Some(url.to_string())
                } else if url.starts_with("http://") {
                    let mut count = self.blocked_count.write().unwrap();
                    *count += 1;
                    log::warn!("Blocked insecure HTTP URL: {}", url);
                    None // Block HTTP
                } else {
                    Some(url.to_string())
                }
            }
        }
    }
    
    /// Set the enforcement mode
    pub fn set_mode(&self, mode: HttpsMode) {
        *self.mode.write().unwrap() = mode;
    }
    
    /// Get current mode
    pub fn get_mode(&self) -> HttpsMode {
        *self.mode.read().unwrap()
    }
    
    /// Get count of upgraded connections
    pub fn get_upgraded_count(&self) -> u64 {
        *self.upgraded_count.read().unwrap()
    }
    
    /// Get count of blocked connections
    pub fn get_blocked_count(&self) -> u64 {
        *self.blocked_count.read().unwrap()
    }
}

impl Default for HttpsEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_upgrade_mode() {
        let enforcer = HttpsEnforcer::new();
        enforcer.set_mode(HttpsMode::Upgrade);
        
        assert_eq!(
            enforcer.process_url("http://example.com"),
            Some("https://example.com".to_string())
        );
        assert_eq!(
            enforcer.process_url("https://example.com"),
            Some("https://example.com".to_string())
        );
    }
    
    #[test]
    fn test_strict_mode() {
        let enforcer = HttpsEnforcer::new();
        enforcer.set_mode(HttpsMode::Strict);
        
        assert_eq!(
            enforcer.process_url("https://example.com"),
            Some("https://example.com".to_string())
        );
        assert_eq!(enforcer.process_url("http://example.com"), None);
    }
}
