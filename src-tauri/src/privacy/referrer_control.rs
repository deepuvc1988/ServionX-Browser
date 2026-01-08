// Referrer Control
// Controls and strips referrer headers for privacy

use std::sync::RwLock;

/// Referrer policy options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferrerPolicy {
    /// Never send referrer header
    NoReferrer,
    /// Only send origin (domain) not full path
    OriginOnly,
    /// Only send referrer to same origin
    SameOrigin,
    /// Send full referrer (default browser behavior)
    Full,
}

/// Controls referrer header behavior
pub struct ReferrerControl {
    policy: RwLock<ReferrerPolicy>,
    stripped_count: RwLock<u64>,
}

impl ReferrerControl {
    pub fn new() -> Self {
        Self {
            policy: RwLock::new(ReferrerPolicy::NoReferrer), // Most private by default
            stripped_count: RwLock::new(0),
        }
    }
    
    /// Get current policy
    pub fn get_policy(&self) -> ReferrerPolicy {
        *self.policy.read().unwrap()
    }
    
    /// Set policy
    pub fn set_policy(&self, policy: ReferrerPolicy) {
        *self.policy.write().unwrap() = policy;
    }
    
    /// Get count of stripped referrers
    pub fn get_stripped_count(&self) -> u64 {
        *self.stripped_count.read().unwrap()
    }
    
    /// Process a referrer based on current policy
    pub fn process_referrer(&self, referrer: &str, target_url: &str) -> Option<String> {
        let policy = *self.policy.read().unwrap();
        
        match policy {
            ReferrerPolicy::NoReferrer => {
                *self.stripped_count.write().unwrap() += 1;
                None
            }
            ReferrerPolicy::OriginOnly => {
                // Extract just the origin
                if let Some(origin) = extract_origin(referrer) {
                    Some(origin)
                } else {
                    None
                }
            }
            ReferrerPolicy::SameOrigin => {
                // Only send if same origin
                let ref_origin = extract_origin(referrer);
                let target_origin = extract_origin(target_url);
                
                if ref_origin == target_origin {
                    Some(referrer.to_string())
                } else {
                    *self.stripped_count.write().unwrap() += 1;
                    None
                }
            }
            ReferrerPolicy::Full => {
                Some(referrer.to_string())
            }
        }
    }
    
    /// Generate JavaScript injection for referrer control
    pub fn get_injection_script(&self) -> String {
        r#"
// Referrer Control Protection
(function() {
    'use strict';
    
    // Override document.referrer
    Object.defineProperty(document, 'referrer', {
        get: function() { return ''; },
        configurable: false
    });
    
    console.log('%c[ServionX] Referrer stripped for privacy', 'color: #22c55e;');
})();
"#.to_string()
    }
}

impl Default for ReferrerControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract origin (scheme + host) from URL
fn extract_origin(url: &str) -> Option<String> {
    // Simple extraction
    if let Some(idx) = url.find("://") {
        let rest = &url[idx + 3..];
        if let Some(slash_idx) = rest.find('/') {
            return Some(format!("{}{}", &url[..idx + 3], &rest[..slash_idx]));
        } else {
            return Some(url.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_referrer() {
        let control = ReferrerControl::new();
        control.set_policy(ReferrerPolicy::NoReferrer);
        
        assert_eq!(
            control.process_referrer("https://example.com/page", "https://other.com"),
            None
        );
    }
    
    #[test]
    fn test_origin_only() {
        let control = ReferrerControl::new();
        control.set_policy(ReferrerPolicy::OriginOnly);
        
        assert_eq!(
            control.process_referrer("https://example.com/secret/page", "https://other.com"),
            Some("https://example.com".to_string())
        );
    }
}
