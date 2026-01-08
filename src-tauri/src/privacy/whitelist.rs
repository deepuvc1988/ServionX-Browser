// Whitelist Manager
// Manages sites that should receive real data

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Manages the whitelist of trusted domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistManager {
    domains: HashSet<String>,
}

impl WhitelistManager {
    pub fn new() -> Self {
        Self {
            domains: HashSet::new(),
        }
    }
    
    /// Check if a domain is whitelisted
    pub fn is_whitelisted(&self, domain: &str) -> bool {
        let domain = Self::normalize_domain(domain);
        
        // Check exact match
        if self.domains.contains(&domain) {
            return true;
        }
        
        // Check if any parent domain is whitelisted
        let parts: Vec<&str> = domain.split('.').collect();
        for i in 0..parts.len().saturating_sub(1) {
            let parent = parts[i..].join(".");
            if self.domains.contains(&parent) {
                return true;
            }
        }
        
        false
    }
    
    /// Add a domain to the whitelist
    pub fn add(&mut self, domain: &str) {
        let domain = Self::normalize_domain(domain);
        log::info!("Added domain to whitelist: {}", &domain);
        self.domains.insert(domain);
    }
    
    /// Remove a domain from the whitelist
    pub fn remove(&mut self, domain: &str) {
        let domain = Self::normalize_domain(domain);
        log::info!("Removed domain from whitelist: {}", &domain);
        self.domains.remove(&domain);
    }
    
    /// Get all whitelisted domains
    pub fn get_all(&self) -> Vec<String> {
        self.domains.iter().cloned().collect()
    }
    
    /// Normalize a domain for consistent matching
    fn normalize_domain(domain: &str) -> String {
        let domain = domain.to_lowercase();
        let domain = domain.trim_start_matches("https://");
        let domain = domain.trim_start_matches("http://");
        let domain = domain.trim_start_matches("www.");
        let domain = domain.split('/').next().unwrap_or(&domain);
        let domain = domain.split(':').next().unwrap_or(domain);
        domain.to_string()
    }
}

impl Default for WhitelistManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_whitelist() {
        let mut manager = WhitelistManager::new();
        
        manager.add("example.com");
        
        assert!(manager.is_whitelisted("example.com"));
        assert!(manager.is_whitelisted("www.example.com"));
        assert!(manager.is_whitelisted("https://example.com/path"));
        assert!(manager.is_whitelisted("sub.example.com"));
        assert!(!manager.is_whitelisted("notexample.com"));
    }
    
    #[test]
    fn test_remove() {
        let mut manager = WhitelistManager::new();
        
        manager.add("example.com");
        assert!(manager.is_whitelisted("example.com"));
        
        manager.remove("example.com");
        assert!(!manager.is_whitelisted("example.com"));
    }
}
