// Tracker and Ad Blocker
// Blocks known tracking and advertising domains

use std::collections::HashSet;
use std::sync::RwLock;

/// Known tracker domains to block
pub const TRACKER_DOMAINS: &[&str] = &[
    // Google Analytics & Ads
    "google-analytics.com",
    "googleadservices.com",
    "googlesyndication.com",
    "googletagmanager.com",
    "googletagservices.com",
    "doubleclick.net",
    "googletraveladservices.com",
    "googleads.g.doubleclick.net",
    
    // Facebook
    "facebook.net",
    "fbcdn.net",
    "connect.facebook.net",
    "pixel.facebook.com",
    
    // Twitter
    "analytics.twitter.com",
    "ads-twitter.com",
    "static.ads-twitter.com",
    
    // Microsoft
    "bat.bing.com",
    "clarity.ms",
    
    // Amazon
    "amazon-adsystem.com",
    "assoc-amazon.com",
    
    // General Trackers
    "scorecardresearch.com",
    "quantserve.com",
    "adnxs.com",
    "adsrvr.org",
    "agkn.com",
    "adsymptotic.com",
    "adtech.de",
    "advertising.com",
    "atdmt.com",
    "bidswitch.net",
    "bluekai.com",
    "casalemedia.com",
    "chartbeat.net",
    "clicktale.net",
    "contextweb.com",
    "criteo.com",
    "criteo.net",
    "crwdcntrl.net",
    "demdex.net",
    "dotomi.com",
    "everesttech.net",
    "exelator.com",
    "eyeota.net",
    "flashtalking.com",
    "hotjar.com",
    "krxd.net",
    "liadm.com",
    "lijit.com",
    "mathtag.com",
    "mediamath.com",
    "mixpanel.com",
    "moatads.com",
    "mookie1.com",
    "myvisualiq.net",
    "narrativ.com",
    "newrelic.com",
    "nr-data.net",
    "omtrdc.net",
    "openx.net",
    "optimizely.com",
    "outbrain.com",
    "pardot.com",
    "pippio.com",
    "pubmatic.com",
    "rlcdn.com",
    "rubiconproject.com",
    "segment.com",
    "segment.io",
    "serving-sys.com",
    "sharethrough.com",
    "siftscience.com",
    "simpli.fi",
    "sitescout.com",
    "smartadserver.com",
    "spotxchange.com",
    "taboola.com",
    "tapad.com",
    "teads.tv",
    "tidaltv.com",
    "tremorhub.com",
    "tribalfusion.com",
    "turn.com",
    "undertone.com",
    "w55c.net",
    "yieldlab.net",
    "yieldmo.com",
    "zemanta.com",
    
    // Common Third-Party Scripts
    "cdn.amplitude.com",
    "cdn.segment.com",
    "cdn.heapanalytics.com",
    "cdn.mxpnl.com",
    "static.hotjar.com",
    "script.hotjar.com",
    "fullstory.com",
    "rs.fullstory.com",
];

/// Tracker blocker that maintains a blocklist
pub struct TrackerBlocker {
    blocked_domains: RwLock<HashSet<String>>,
    enabled: RwLock<bool>,
    blocked_count: RwLock<u64>,
}

impl TrackerBlocker {
    pub fn new() -> Self {
        let mut domains = HashSet::new();
        for domain in TRACKER_DOMAINS {
            domains.insert(domain.to_string());
        }
        
        Self {
            blocked_domains: RwLock::new(domains),
            enabled: RwLock::new(true),
            blocked_count: RwLock::new(0),
        }
    }
    
    /// Check if a URL should be blocked
    pub fn should_block(&self, url: &str) -> bool {
        if !*self.enabled.read().unwrap() {
            return false;
        }
        
        // Extract domain from URL
        if let Some(domain) = extract_domain(url) {
            let domains = self.blocked_domains.read().unwrap();
            
            // Check if domain or any parent domain is blocked
            for blocked in domains.iter() {
                if domain == *blocked || domain.ends_with(&format!(".{}", blocked)) {
                    let mut count = self.blocked_count.write().unwrap();
                    *count += 1;
                    log::info!("Blocked tracker: {}", domain);
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get count of blocked trackers
    pub fn get_blocked_count(&self) -> u64 {
        *self.blocked_count.read().unwrap()
    }
    
    /// Add a domain to the blocklist
    pub fn add_domain(&self, domain: &str) {
        self.blocked_domains.write().unwrap().insert(domain.to_string());
    }
    
    /// Remove a domain from the blocklist
    pub fn remove_domain(&self, domain: &str) {
        self.blocked_domains.write().unwrap().remove(domain);
    }
    
    /// Enable or disable the blocker
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }
    
    /// Check if blocker is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }
    
    /// Get all blocked domains
    pub fn get_blocked_domains(&self) -> Vec<String> {
        self.blocked_domains.read().unwrap().iter().cloned().collect()
    }
}

impl Default for TrackerBlocker {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract domain from a URL
fn extract_domain(url: &str) -> Option<String> {
    // Simple domain extraction
    let url = url.trim_start_matches("https://")
        .trim_start_matches("http://");
    
    let domain = url.split('/').next()?;
    let domain = domain.split(':').next()?; // Remove port if present
    
    Some(domain.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tracker_blocking() {
        let blocker = TrackerBlocker::new();
        
        assert!(blocker.should_block("https://google-analytics.com/collect"));
        assert!(blocker.should_block("https://www.googletagmanager.com/gtag.js"));
        assert!(blocker.should_block("https://pixel.facebook.com/tr"));
        
        assert!(!blocker.should_block("https://google.com"));
        assert!(!blocker.should_block("https://www.example.com"));
    }
    
    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path"), Some("example.com".to_string()));
        assert_eq!(extract_domain("http://sub.example.com:8080/path"), Some("sub.example.com".to_string()));
    }
}
