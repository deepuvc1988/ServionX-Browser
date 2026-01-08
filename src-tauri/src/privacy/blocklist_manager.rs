// Auto-Updating Blocklist System
// Fetches and updates blocklists from open source feeds

use std::collections::HashSet;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Open source blocklist feeds
pub const BLOCKLIST_FEEDS: &[(&str, &str)] = &[
    // EasyList - Ad blocking
    ("easylist", "https://easylist.to/easylist/easylist.txt"),
    // EasyPrivacy - Tracker blocking
    ("easyprivacy", "https://easylist.to/easylist/easyprivacy.txt"),
    // Peter Lowe's Ad and tracking server list
    ("pgl", "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0"),
    // Malware domains
    ("malware", "https://malware-filter.gitlab.io/malware-filter/urlhaus-filter-hosts.txt"),
    // Phishing domains (PhishTank)
    ("phishing", "https://data.phishtank.com/data/online-valid.json"),
    // URLHaus - Malware URLs
    ("urlhaus", "https://urlhaus.abuse.ch/downloads/hostfile/"),
];

/// Blockable rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockRule {
    Domain(String),
    UrlPattern(String),
    CssSelector(String),
    ScriptPattern(String),
}

/// Auto-updating blocklist manager
pub struct BlocklistManager {
    // Domain-based blocks
    blocked_domains: RwLock<HashSet<String>>,
    // URL pattern blocks
    url_patterns: RwLock<Vec<String>>,
    // CSS selectors for cosmetic filtering
    cosmetic_filters: RwLock<Vec<String>>,
    // Statistics
    total_rules: RwLock<usize>,
    last_update: RwLock<Option<i64>>,
    enabled: RwLock<bool>,
    blocked_count: RwLock<u64>,
}

impl BlocklistManager {
    pub fn new() -> Self {
        let mut domains = HashSet::new();
        
        // Include built-in tracker domains
        for domain in super::tracker_blocker::TRACKER_DOMAINS {
            domains.insert(domain.to_string());
        }
        
        // Add common ad domains
        let ad_domains = vec![
            "doubleclick.net", "googlesyndication.com", "googleadservices.com",
            "adnxs.com", "adsrvr.org", "adform.net", "advertising.com",
            "outbrain.com", "taboola.com", "criteo.com", "pubmatic.com",
            "rubiconproject.com", "openx.net", "casalemedia.com",
            "bidswitch.net", "spotxchange.com", "smartadserver.com",
            "moatads.com", "serving-sys.com", "flashtalking.com",
            "adroll.com", "mathtag.com", "lijit.com", "sharethrough.com",
        ];
        
        for domain in ad_domains {
            domains.insert(domain.to_string());
        }
        
        Self {
            blocked_domains: RwLock::new(domains),
            url_patterns: RwLock::new(Vec::new()),
            cosmetic_filters: RwLock::new(Vec::new()),
            total_rules: RwLock::new(0),
            last_update: RwLock::new(None),
            enabled: RwLock::new(true),
            blocked_count: RwLock::new(0),
        }
    }
    
    /// Check if a URL should be blocked
    pub fn should_block(&self, url: &str) -> bool {
        if !*self.enabled.read().unwrap() {
            return false;
        }
        
        let url_lower = url.to_lowercase();
        
        // Check domain blocks
        if let Some(domain) = extract_domain(&url_lower) {
            let domains = self.blocked_domains.read().unwrap();
            for blocked in domains.iter() {
                if domain == *blocked || domain.ends_with(&format!(".{}", blocked)) {
                    *self.blocked_count.write().unwrap() += 1;
                    log::debug!("Blocked by domain rule: {}", domain);
                    return true;
                }
            }
        }
        
        // Check URL patterns
        let patterns = self.url_patterns.read().unwrap();
        for pattern in patterns.iter() {
            if url_lower.contains(pattern) {
                *self.blocked_count.write().unwrap() += 1;
                log::debug!("Blocked by URL pattern: {}", pattern);
                return true;
            }
        }
        
        false
    }
    
    /// Parse EasyList format rules
    pub fn parse_easylist_rules(&self, content: &str) {
        let mut domains = self.blocked_domains.write().unwrap();
        let mut patterns = self.url_patterns.write().unwrap();
        let mut cosmetic = self.cosmetic_filters.write().unwrap();
        let mut count = 0;
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('!') || line.starts_with('[') {
                continue;
            }
            
            // Domain block: ||example.com^
            if line.starts_with("||") && line.ends_with('^') {
                let domain = line.trim_start_matches("||").trim_end_matches('^');
                if !domain.contains('/') && !domain.contains('*') {
                    domains.insert(domain.to_lowercase());
                    count += 1;
                }
            }
            // Cosmetic filter: ##.ad-banner
            else if line.contains("##") {
                if let Some(selector) = line.split("##").nth(1) {
                    cosmetic.push(selector.to_string());
                    count += 1;
                }
            }
            // URL pattern
            else if !line.starts_with("@@") && !line.contains('#') {
                patterns.push(line.to_lowercase());
                count += 1;
            }
        }
        
        *self.total_rules.write().unwrap() += count;
    }
    
    /// Parse hosts file format
    pub fn parse_hosts_rules(&self, content: &str) {
        let mut domains = self.blocked_domains.write().unwrap();
        let mut count = 0;
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Hosts format: 0.0.0.0 example.com or 127.0.0.1 example.com
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let domain = parts[1].to_lowercase();
                if domain != "localhost" && !domain.starts_with("local") {
                    domains.insert(domain);
                    count += 1;
                }
            }
        }
        
        *self.total_rules.write().unwrap() += count;
    }
    
    /// Get cosmetic filter CSS for injection
    pub fn get_cosmetic_css(&self) -> String {
        let filters = self.cosmetic_filters.read().unwrap();
        if filters.is_empty() {
            return String::new();
        }
        
        let selectors: Vec<String> = filters.iter()
            .take(1000) // Limit for performance
            .cloned()
            .collect();
        
        format!("{} {{ display: none !important; }}", selectors.join(", "))
    }
    
    /// Get injection script for ad blocking
    pub fn get_injection_script(&self) -> String {
        let css = self.get_cosmetic_css();
        let blocked_count = *self.blocked_count.read().unwrap();
        
        format!(r#"
// Smart Ad Blocking
(function() {{
    'use strict';
    
    // Inject cosmetic filters
    const style = document.createElement('style');
    style.textContent = `{css}`;
    document.head.appendChild(style);
    
    // Block inline ads
    const adPatterns = [
        /adsense/i, /adsbygoogle/i, /googletag/i,
        /doubleclick/i, /taboola/i, /outbrain/i
    ];
    
    // Monitor and block ad scripts
    const originalCreate = document.createElement.bind(document);
    document.createElement = function(tagName) {{
        const element = originalCreate(tagName);
        if (tagName.toLowerCase() === 'script') {{
            const originalSet = element.__lookupSetter__('src');
            Object.defineProperty(element, 'src', {{
                set: function(value) {{
                    for (const pattern of adPatterns) {{
                        if (pattern.test(value)) {{
                            console.log('%c[ServionX] Blocked ad script: ' + value, 'color: #ef4444;');
                            return;
                        }}
                    }}
                    originalSet.call(this, value);
                }}
            }});
        }}
        return element;
    }};
    
    console.log('%c[ServionX] Smart ad blocking active. Requests blocked: {blocked_count}', 'color: #22c55e;');
}})();
"#, css = css, blocked_count = blocked_count)
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> BlocklistStats {
        BlocklistStats {
            total_rules: *self.total_rules.read().unwrap(),
            blocked_domains: self.blocked_domains.read().unwrap().len(),
            url_patterns: self.url_patterns.read().unwrap().len(),
            cosmetic_filters: self.cosmetic_filters.read().unwrap().len(),
            blocked_count: *self.blocked_count.read().unwrap(),
            last_update: *self.last_update.read().unwrap(),
        }
    }
    
    /// Add a domain to block
    pub fn add_domain(&self, domain: &str) {
        self.blocked_domains.write().unwrap().insert(domain.to_lowercase());
    }
    
    /// Set enabled state
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }
    
    /// Update last update time
    pub fn set_last_update(&self, timestamp: i64) {
        *self.last_update.write().unwrap() = Some(timestamp);
    }
}

impl Default for BlocklistManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Blocklist statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocklistStats {
    pub total_rules: usize,
    pub blocked_domains: usize,
    pub url_patterns: usize,
    pub cosmetic_filters: usize,
    pub blocked_count: u64,
    pub last_update: Option<i64>,
}

/// Extract domain from URL
fn extract_domain(url: &str) -> Option<String> {
    let url = url.trim_start_matches("https://")
        .trim_start_matches("http://");
    let domain = url.split('/').next()?;
    let domain = domain.split(':').next()?;
    Some(domain.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_domain_blocking() {
        let manager = BlocklistManager::new();
        
        assert!(manager.should_block("https://doubleclick.net/ad"));
        assert!(manager.should_block("https://ads.googlesyndication.com/script.js"));
        assert!(!manager.should_block("https://google.com"));
    }
    
    #[test]
    fn test_easylist_parsing() {
        let manager = BlocklistManager::new();
        let rules = r#"
! Comment line
[Adblock Plus 2.0]
||example-ads.com^
||tracking.domain.com^
##.advertisement
##.ad-banner
"#;
        manager.parse_easylist_rules(rules);
        
        assert!(manager.should_block("https://example-ads.com/script.js"));
    }
}
