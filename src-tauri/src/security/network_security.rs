//! Network Security Module
//! Provides DNS over HTTPS, certificate transparency, and cookie hardening

use serde::{Deserialize, Serialize};

/// DNS over HTTPS provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DohProvider {
    Cloudflare,     // 1.1.1.1
    Google,         // 8.8.8.8
    Quad9,          // 9.9.9.9
    NextDns,        // Custom
    Custom(String), // User-defined
}

impl DohProvider {
    pub fn get_url(&self) -> &str {
        match self {
            DohProvider::Cloudflare => "https://cloudflare-dns.com/dns-query",
            DohProvider::Google => "https://dns.google/dns-query",
            DohProvider::Quad9 => "https://dns.quad9.net/dns-query",
            DohProvider::NextDns => "https://dns.nextdns.io",
            DohProvider::Custom(url) => url.as_str(),
        }
    }
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurity {
    // DNS over HTTPS
    pub doh_enabled: bool,
    pub doh_provider: DohProvider,
    
    // Certificate security
    pub cert_transparency_enabled: bool,
    pub reject_expired_certs: bool,
    pub reject_self_signed: bool,
    pub min_tls_version: String, // "1.2" or "1.3"
    
    // Mixed content
    pub block_mixed_content: bool,
    pub upgrade_insecure_requests: bool,
    
    // Cookie hardening
    pub force_samesite_strict: bool,
    pub force_secure_cookies: bool,
    pub block_third_party_cookies: bool,
    
    // Request security
    pub block_insecure_downloads: bool,
    pub block_ftp: bool,
    pub block_data_urls_in_frames: bool,
}

impl Default for NetworkSecurity {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkSecurity {
    pub fn new() -> Self {
        Self {
            doh_enabled: true,
            doh_provider: DohProvider::Cloudflare,
            
            cert_transparency_enabled: true,
            reject_expired_certs: true,
            reject_self_signed: true,
            min_tls_version: "1.2".to_string(),
            
            block_mixed_content: true,
            upgrade_insecure_requests: true,
            
            force_samesite_strict: true,
            force_secure_cookies: true,
            block_third_party_cookies: true,
            
            block_insecure_downloads: true,
            block_ftp: true,
            block_data_urls_in_frames: true,
        }
    }
    
    /// Get network security injection script
    pub fn get_injection_script(&self) -> String {
        format!(r#"
(function() {{
    'use strict';
    
    // ============================================
    // NETWORK SECURITY - ServionX
    // ============================================
    
    const config = {{
        blockMixedContent: {block_mixed},
        upgradeInsecure: {upgrade_insecure},
        forceSameSite: {force_samesite},
        forceSecure: {force_secure},
        blockThirdParty: {block_third_party},
        blockDataUrls: {block_data_urls}
    }};
    
    // === MIXED CONTENT BLOCKING ===
    if (config.blockMixedContent && window.location.protocol === 'https:') {{
        // Monitor for insecure resource loading
        const observer = new MutationObserver((mutations) => {{
            mutations.forEach((mutation) => {{
                mutation.addedNodes.forEach((node) => {{
                    if (node.tagName === 'IMG' || node.tagName === 'SCRIPT' || node.tagName === 'LINK') {{
                        const src = node.src || node.href;
                        if (src && src.startsWith('http:')) {{
                            console.warn('[ServionX] Blocked insecure resource:', src);
                            node.remove();
                        }}
                    }}
                }});
            }});
        }});
        observer.observe(document.documentElement, {{ childList: true, subtree: true }});
    }}
    
    // === UPGRADE INSECURE REQUESTS ===
    if (config.upgradeInsecure) {{
        // Intercept fetch requests
        const originalFetch = window.fetch;
        window.fetch = function(url, options) {{
            if (typeof url === 'string' && url.startsWith('http:')) {{
                url = url.replace('http:', 'https:');
                console.log('[ServionX] Upgraded insecure request:', url);
            }}
            return originalFetch.call(this, url, options);
        }};
        
        // Intercept XMLHttpRequest
        const originalOpen = XMLHttpRequest.prototype.open;
        XMLHttpRequest.prototype.open = function(method, url, ...args) {{
            if (typeof url === 'string' && url.startsWith('http:')) {{
                url = url.replace('http:', 'https:');
                console.log('[ServionX] Upgraded XHR request:', url);
            }}
            return originalOpen.call(this, method, url, ...args);
        }};
    }}
    
    // === COOKIE SECURITY ===
    if (config.forceSameSite || config.forceSecure) {{
        const originalCookieSetter = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie').set;
        Object.defineProperty(document, 'cookie', {{
            set: function(cookieString) {{
                let enhanced = cookieString;
                
                // Force SameSite=Strict
                if (config.forceSameSite && !enhanced.includes('SameSite')) {{
                    enhanced += '; SameSite=Strict';
                }}
                
                // Force Secure flag
                if (config.forceSecure && window.location.protocol === 'https:' && !enhanced.includes('Secure')) {{
                    enhanced += '; Secure';
                }}
                
                return originalCookieSetter.call(this, enhanced);
            }},
            get: Object.getOwnPropertyDescriptor(Document.prototype, 'cookie').get,
            configurable: true
        }});
        console.log('[ServionX] Cookie security enhanced');
    }}
    
    // === THIRD-PARTY COOKIE BLOCKING ===
    if (config.blockThirdParty) {{
        // Block third-party iframes from accessing cookies
        if (window.self !== window.top) {{
            const parentHost = new URL(document.referrer).hostname;
            const currentHost = window.location.hostname;
            if (parentHost !== currentHost) {{
                Object.defineProperty(document, 'cookie', {{
                    get: () => '',
                    set: () => {{}},
                    configurable: false
                }});
                console.log('[ServionX] Third-party cookies blocked');
            }}
        }}
    }}
    
    // === DATA URL BLOCKING IN FRAMES ===
    if (config.blockDataUrls) {{
        // Block data: URLs in iframes
        const observer = new MutationObserver((mutations) => {{
            mutations.forEach((mutation) => {{
                mutation.addedNodes.forEach((node) => {{
                    if (node.tagName === 'IFRAME') {{
                        const src = node.src;
                        if (src && (src.startsWith('data:') || src.startsWith('javascript:'))) {{
                            console.warn('[ServionX] Blocked dangerous iframe:', src.substring(0, 50));
                            node.remove();
                        }}
                    }}
                }});
            }});
        }});
        observer.observe(document.documentElement, {{ childList: true, subtree: true }});
    }}
    
    // === REFERRER POLICY ===
    // Set strict referrer policy
    const meta = document.createElement('meta');
    meta.name = 'referrer';
    meta.content = 'strict-origin-when-cross-origin';
    document.head.appendChild(meta);
    
    // === CSP ENHANCEMENT ===
    // Add additional CSP directives via meta tag
    const csp = document.createElement('meta');
    csp.httpEquiv = 'Content-Security-Policy';
    csp.content = "upgrade-insecure-requests; block-all-mixed-content;";
    document.head.appendChild(csp);
    
    console.log('[ServionX] Network security active');
    console.log('[ServionX] Mixed content blocking: ' + config.blockMixedContent);
    console.log('[ServionX] Cookie hardening: SameSite=' + config.forceSameSite + ', Secure=' + config.forceSecure);
}})();
"#,
            block_mixed = self.block_mixed_content,
            upgrade_insecure = self.upgrade_insecure_requests,
            force_samesite = self.force_samesite_strict,
            force_secure = self.force_secure_cookies,
            block_third_party = self.block_third_party_cookies,
            block_data_urls = self.block_data_urls_in_frames,
        )
    }
    
    /// Check if URL should be blocked
    pub fn should_block_url(&self, url: &str) -> (bool, &'static str) {
        // Block FTP
        if self.block_ftp && url.starts_with("ftp://") {
            return (true, "FTP protocol blocked");
        }
        
        // Block data URLs (for main frame only)
        if self.block_data_urls_in_frames && url.starts_with("data:text/html") {
            return (true, "Data URL blocked");
        }
        
        // Block javascript URLs
        if url.starts_with("javascript:") {
            return (true, "JavaScript URL blocked");
        }
        
        (false, "")
    }
}

/// Certificate Transparency Log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertTransparencyLog {
    pub name: String,
    pub url: String,
    pub is_trusted: bool,
}

impl CertTransparencyLog {
    pub fn get_trusted_logs() -> Vec<Self> {
        vec![
            CertTransparencyLog {
                name: "Google Argon".to_string(),
                url: "https://ct.googleapis.com/logs/argon2024/".to_string(),
                is_trusted: true,
            },
            CertTransparencyLog {
                name: "Cloudflare Nimbus".to_string(),
                url: "https://ct.cloudflare.com/logs/nimbus2024/".to_string(),
                is_trusted: true,
            },
            CertTransparencyLog {
                name: "DigiCert Yeti".to_string(),
                url: "https://yeti2024.ct.digicert.com/log/".to_string(),
                is_trusted: true,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_security_defaults() {
        let ns = NetworkSecurity::new();
        assert!(ns.doh_enabled);
        assert!(ns.block_mixed_content);
        assert!(ns.force_samesite_strict);
        assert!(ns.block_third_party_cookies);
    }
    
    #[test]
    fn test_url_blocking() {
        let ns = NetworkSecurity::new();
        let (blocked, _) = ns.should_block_url("ftp://files.example.com");
        assert!(blocked);
        
        let (blocked, _) = ns.should_block_url("javascript:alert(1)");
        assert!(blocked);
    }
    
    #[test]
    fn test_doh_provider_urls() {
        assert_eq!(DohProvider::Cloudflare.get_url(), "https://cloudflare-dns.com/dns-query");
        assert_eq!(DohProvider::Google.get_url(), "https://dns.google/dns-query");
    }
}
