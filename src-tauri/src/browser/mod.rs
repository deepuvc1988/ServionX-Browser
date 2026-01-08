// Browser WebView Manager
// Manages native webview windows for actual web browsing with privacy protection

pub mod commands;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use crate::privacy::{PrivacyEngine, TrackerBlocker, HttpsEnforcer};

/// Represents a browser tab with its webview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub is_loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

/// Browser security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub javascript_enabled: bool,
    pub tracker_blocking: bool,
    pub https_only: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            javascript_enabled: true,
            tracker_blocking: true,
            https_only: true,
        }
    }
}

/// Manages browser webviews with privacy injection
pub struct WebViewManager {
    tabs: Arc<RwLock<HashMap<String, BrowserTab>>>,
    settings: Arc<RwLock<SecuritySettings>>,
}

impl WebViewManager {
    pub fn new() -> Self {
        Self {
            tabs: Arc::new(RwLock::new(HashMap::new())),
            settings: Arc::new(RwLock::new(SecuritySettings::default())),
        }
    }
    
    /// Get current security settings
    pub fn get_settings(&self) -> SecuritySettings {
        self.settings.read().unwrap().clone()
    }
    
    /// Update security settings
    pub fn update_settings(&self, settings: SecuritySettings) {
        *self.settings.write().unwrap() = settings;
    }
    
    /// Create a new browser tab with native webview and privacy protection
    pub fn create_tab(&self, app: &AppHandle, tab_id: &str, url: &str) -> Result<BrowserTab, String> {
        use crate::privacy::{MalwareBlocker, MalwareCheckResult, FontFingerprint, 
            ReferrerControl, FingerprintingDetector, StoragePartitioner};
        
        let window_label = format!("tab_{}", tab_id);
        let settings = self.get_settings();
        
        // Check malware blocker first
        let malware_blocker = app.state::<MalwareBlocker>();
        match malware_blocker.check_url(url) {
            MalwareCheckResult::Blocked { reason } => {
                log::warn!("Malware blocker: {}", reason);
                return Err(format!("🛑 Blocked: {}", reason));
            }
            MalwareCheckResult::Suspicious { reason } => {
                log::warn!("Suspicious URL: {}", reason);
                // Continue but log warning
            }
            MalwareCheckResult::Safe => {}
        }
        
        // Get privacy engine for injection scripts
        let privacy_engine = app.state::<PrivacyEngine>();
        let base_injection = privacy_engine.get_injection_script();
        
        // Check tracker blocker
        let tracker_blocker = app.state::<TrackerBlocker>();
        if settings.tracker_blocking && tracker_blocker.should_block(url) {
            return Err(format!("Blocked tracker: {}", url));
        }
        
        // Enforce HTTPS
        let https_enforcer = app.state::<HttpsEnforcer>();
        let final_url = if settings.https_only {
            match https_enforcer.process_url(url) {
                Some(processed) => processed,
                None => return Err(format!("Blocked insecure HTTP URL: {}", url)),
            }
        } else {
            url.to_string()
        };
        
        // Build the webview URL
        let webview_url = if final_url.is_empty() {
            WebviewUrl::App("index.html".into())
        } else {
            WebviewUrl::External(final_url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
        };
        
        // Get fake user agent from privacy engine
        let fake_ua = privacy_engine.get_identity().user_agent.full;
        
        // Combine ALL injection scripts for MAXIMUM protection (7 layers)
        let font_fp = app.state::<FontFingerprint>();
        let referrer_ctrl = app.state::<ReferrerControl>();
        let advanced_fp = app.state::<crate::privacy::AdvancedFingerprintProtection>();
        let network_sec = app.state::<crate::security::NetworkSecurity>();
        let complete_fake = app.state::<crate::privacy::CompleteFakeData>();
        let ultimate = app.state::<crate::privacy::UltimatePrivacyProtection>();
        let upload_prot = app.state::<crate::metadata::FakeFileMetadata>();
        let additional = app.state::<crate::privacy::AdditionalProtection>();
        
        let combined_injection = format!(
            "{}\\n{}\\n{}\\n{}\\n{}\\n{}\\n{}\\n{}\\n{}\\n{}\\n{}",
            base_injection,
            font_fp.get_injection_script(),
            referrer_ctrl.get_injection_script(),
            FingerprintingDetector::get_injection_script(),
            StoragePartitioner::get_injection_script(),
            advanced_fp.get_injection_script(),
            network_sec.get_injection_script(),
            complete_fake.get_master_injection_script(),
            ultimate.get_ultimate_injection_script(),
            upload_prot.get_upload_protection_script(),
            additional.get_injection_script()
        );
        
        log::info!("Combined injection script: {} bytes (7 protection layers)", combined_injection.len());
        
        // Create the webview window with privacy protections
        let _window = WebviewWindowBuilder::new(app, &window_label, webview_url)
            .title("ServionX Browser - Protected")
            .inner_size(1200.0, 800.0)
            .visible(true)
            .initialization_script(&combined_injection)  // Inject ALL privacy scripts
            .user_agent(&fake_ua)  // Use fake user agent
            .build()
            .map_err(|e| e.to_string())?;
        
        // Log security status
        log::info!("Created protected browser tab {} for URL: {}", tab_id, final_url);
        log::info!("  → Tracker blocking: {}", if settings.tracker_blocking { "ON" } else { "OFF" });
        log::info!("  → HTTPS enforced: {}", if settings.https_only { "ON" } else { "OFF" });
        log::info!("  → JavaScript: {}", if settings.javascript_enabled { "ON" } else { "OFF" });
        log::info!("  → Trackers blocked so far: {}", tracker_blocker.get_blocked_count());
        
        let tab = BrowserTab {
            id: tab_id.to_string(),
            url: final_url,
            title: "Loading...".to_string(),
            is_loading: true,
            can_go_back: false,
            can_go_forward: false,
        };
        
        self.tabs.write().unwrap().insert(tab_id.to_string(), tab.clone());
        
        Ok(tab)
    }
    
    /// Navigate a tab to a new URL
    pub fn navigate(&self, app: &AppHandle, tab_id: &str, url: &str) -> Result<(), String> {
        let window_label = format!("tab_{}", tab_id);
        let settings = self.get_settings();
        
        if let Some(window) = app.get_webview_window(&window_label) {
            // Check tracker blocker
            let tracker_blocker = app.state::<TrackerBlocker>();
            if settings.tracker_blocking && tracker_blocker.should_block(url) {
                return Err(format!("Blocked tracker: {}", url));
            }
            
            // Prepare URL with HTTPS enforcement
            let mut final_url = if !url.starts_with("http://") && !url.starts_with("https://") {
                format!("https://{}", url)
            } else {
                url.to_string()
            };
            
            // Enforce HTTPS
            let https_enforcer = app.state::<HttpsEnforcer>();
            if settings.https_only {
                match https_enforcer.process_url(&final_url) {
                    Some(processed) => final_url = processed,
                    None => return Err(format!("Blocked insecure HTTP URL: {}", url)),
                }
            }
            
            window.navigate(final_url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
                .map_err(|e| e.to_string())?;
            
            // Update tab state
            if let Some(tab) = self.tabs.write().unwrap().get_mut(tab_id) {
                tab.url = final_url;
                tab.is_loading = true;
            }
            
            Ok(())
        } else {
            Err("Tab not found".to_string())
        }
    }
    
    /// Close a tab
    pub fn close_tab(&self, app: &AppHandle, tab_id: &str) -> Result<(), String> {
        let window_label = format!("tab_{}", tab_id);
        
        if let Some(window) = app.get_webview_window(&window_label) {
            window.close().map_err(|e| e.to_string())?;
        }
        
        self.tabs.write().unwrap().remove(tab_id);
        
        Ok(())
    }
    
    /// Get all tabs
    pub fn get_tabs(&self) -> Vec<BrowserTab> {
        self.tabs.read().unwrap().values().cloned().collect()
    }
    
    /// Update tab info
    pub fn update_tab(&self, tab_id: &str, title: Option<String>, is_loading: Option<bool>) {
        if let Some(tab) = self.tabs.write().unwrap().get_mut(tab_id) {
            if let Some(t) = title {
                tab.title = t;
            }
            if let Some(l) = is_loading {
                tab.is_loading = l;
            }
        }
    }
}

impl Default for WebViewManager {
    fn default() -> Self {
        Self::new()
    }
}
