// User Agent Generator
// Generates realistic fake user agent strings

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Complete fake user agent data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FakeUserAgent {
    pub full: String,
    pub app_version: String,
    pub platform: String,
    pub vendor: String,
    pub browser_name: String,
    pub browser_version: String,
    pub os_name: String,
    pub os_version: String,
}

#[derive(Debug)]
struct BrowserConfig {
    name: &'static str,
    versions: &'static [&'static str],
    vendor: &'static str,
}

#[derive(Debug)]
struct OsConfig {
    name: &'static str,
    platform: &'static str,
    versions: &'static [&'static str],
}

#[derive(Debug)]
pub struct UserAgentGenerator {
    browsers: Vec<BrowserConfig>,
    windows_versions: Vec<OsConfig>,
    mac_versions: Vec<OsConfig>,
    linux_versions: Vec<OsConfig>,
}

impl UserAgentGenerator {
    pub fn new() -> Self {
        Self {
            browsers: vec![
                BrowserConfig {
                    name: "Chrome",
                    versions: &["120.0.0.0", "119.0.0.0", "118.0.0.0", "121.0.0.0", "122.0.0.0", "123.0.0.0", "124.0.0.0", "125.0.0.0"],
                    vendor: "Google Inc.",
                },
                BrowserConfig {
                    name: "Firefox",
                    versions: &["121.0", "120.0", "119.0", "122.0", "123.0", "124.0"],
                    vendor: "",
                },
                BrowserConfig {
                    name: "Safari",
                    versions: &["17.2", "17.1", "17.0", "16.6", "17.3", "17.4"],
                    vendor: "Apple Computer, Inc.",
                },
                BrowserConfig {
                    name: "Edge",
                    versions: &["120.0.0.0", "119.0.0.0", "121.0.0.0", "122.0.0.0", "123.0.0.0"],
                    vendor: "Google Inc.",
                },
            ],
            windows_versions: vec![
                OsConfig { name: "Windows", platform: "Win32", versions: &["10.0", "11.0"] },
            ],
            mac_versions: vec![
                OsConfig { name: "macOS", platform: "MacIntel", versions: &["10_15_7", "11_0", "12_0", "13_0", "14_0", "14_1", "14_2"] },
            ],
            linux_versions: vec![
                OsConfig { name: "Linux", platform: "Linux x86_64", versions: &["x86_64"] },
            ],
        }
    }
    
    /// Generate a random fake user agent
    pub fn generate(&self) -> FakeUserAgent {
        let mut rng = rand::thread_rng();
        
        // Select random browser
        let browser = &self.browsers[rng.gen_range(0..self.browsers.len())];
        let browser_version = browser.versions[rng.gen_range(0..browser.versions.len())];
        
        // Select random OS (weighted towards Windows)
        let os_choice = rng.gen_range(0..100);
        let (os_name, platform, os_version) = if os_choice < 60 {
            // Windows (60%)
            let os = &self.windows_versions[0];
            let ver = os.versions[rng.gen_range(0..os.versions.len())];
            (os.name, os.platform, ver)
        } else if os_choice < 85 {
            // macOS (25%)
            let os = &self.mac_versions[0];
            let ver = os.versions[rng.gen_range(0..os.versions.len())];
            (os.name, os.platform, ver)
        } else {
            // Linux (15%)
            let os = &self.linux_versions[0];
            let ver = os.versions[0];
            (os.name, os.platform, ver)
        };
        
        // Generate user agent string
        let (full, app_version) = self.build_user_agent(
            browser.name,
            browser_version,
            os_name,
            os_version,
            platform,
        );
        
        FakeUserAgent {
            full,
            app_version,
            platform: platform.to_string(),
            vendor: browser.vendor.to_string(),
            browser_name: browser.name.to_string(),
            browser_version: browser_version.to_string(),
            os_name: os_name.to_string(),
            os_version: os_version.to_string(),
        }
    }
    
    fn build_user_agent(
        &self,
        browser: &str,
        browser_version: &str,
        os_name: &str,
        os_version: &str,
        _platform: &str,
    ) -> (String, String) {
        match browser {
            "Chrome" => {
                let os_string = match os_name {
                    "Windows" => format!("Windows NT {}", os_version),
                    "macOS" => format!("Macintosh; Intel Mac OS X {}", os_version),
                    "Linux" => "X11; Linux x86_64".to_string(),
                    _ => "Windows NT 10.0".to_string(),
                };
                
                let full = format!(
                    "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                    os_string, browser_version
                );
                let app_version = format!(
                    "5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
                    os_string, browser_version
                );
                (full, app_version)
            }
            "Firefox" => {
                let os_string = match os_name {
                    "Windows" => format!("Windows NT {}; Win64; x64", os_version),
                    "macOS" => format!("Macintosh; Intel Mac OS X {}", os_version),
                    "Linux" => "X11; Linux x86_64".to_string(),
                    _ => "Windows NT 10.0; Win64; x64".to_string(),
                };
                
                let full = format!(
                    "Mozilla/5.0({}; rv:{}) Gecko/20100101 Firefox/{}",
                    os_string, browser_version, browser_version
                );
                let app_version = format!(
                    "5.0 ({})",
                    os_string
                );
                (full, app_version)
            }
            "Safari" => {
                let os_string = format!("Macintosh; Intel Mac OS X {}", os_version);
                let webkit_version = "605.1.15";
                
                let full = format!(
                    "Mozilla/5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}",
                    os_string, webkit_version, browser_version, webkit_version
                );
                let app_version = format!(
                    "5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}",
                    os_string, webkit_version, browser_version, webkit_version
                );
                (full, app_version)
            }
            "Edge" => {
                let os_string = match os_name {
                    "Windows" => format!("Windows NT {}", os_version),
                    "macOS" => format!("Macintosh; Intel Mac OS X {}", os_version),
                    _ => "Windows NT 10.0".to_string(),
                };
                
                let full = format!(
                    "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36 Edg/{}",
                    os_string, browser_version, browser_version
                );
                let app_version = format!(
                    "5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36 Edg/{}",
                    os_string, browser_version, browser_version
                );
                (full, app_version)
            }
            _ => {
                let full = format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/{} Safari/537.36",
                    browser_version
                );
                (full.clone(), full)
            }
        }
    }
}

impl Default for UserAgentGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_agent_generation() {
        let gen = UserAgentGenerator::new();
        let ua = gen.generate();
        
        assert!(!ua.full.is_empty());
        assert!(ua.full.contains("Mozilla"));
        assert!(!ua.browser_name.is_empty());
        assert!(!ua.os_name.is_empty());
    }
}
