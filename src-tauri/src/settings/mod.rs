//! Browser Settings Module
//! Persisted settings that control browser behavior

pub mod commands;

use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Browser settings - persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    // Security settings
    pub tor_enabled: bool,
    pub https_only: bool,
    pub block_trackers: bool,
    pub block_malware: bool,
    pub scan_downloads: bool,
    pub block_ads: bool,
    
    // Privacy settings
    pub strip_metadata: bool,
    pub block_webrtc: bool,
    pub fake_geolocation: bool,
    pub spoof_fingerprint: bool,
    pub strip_referrer: bool,
    pub partition_storage: bool,
    pub auto_regenerate_identity: bool,
    pub secure_keyboard: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            // Security - all on by default
            tor_enabled: false,  // Tor off by default (requires setup)
            https_only: true,
            block_trackers: true,
            block_malware: true,
            scan_downloads: true,
            block_ads: true,
            
            // Privacy - all on by default
            strip_metadata: true,
            block_webrtc: true,
            fake_geolocation: true,
            spoof_fingerprint: true,
            strip_referrer: true,
            partition_storage: true,
            auto_regenerate_identity: false,  // Manual regeneration by default
            secure_keyboard: true,
        }
    }
}

/// Settings manager with persistence
pub struct SettingsManager {
    data_dir: PathBuf,
    settings: Arc<RwLock<BrowserSettings>>,
}

impl SettingsManager {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ServionX Browser");
        
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        
        // Load existing settings or use defaults
        let settings = Self::load_from_disk(&data_dir).unwrap_or_default();
        
        log::info!("Settings loaded: HTTPS-only={}, Trackers={}, Malware={}", 
            settings.https_only, settings.block_trackers, settings.block_malware);
        
        Ok(Self {
            data_dir,
            settings: Arc::new(RwLock::new(settings)),
        })
    }
    
    /// Load settings from disk
    fn load_from_disk(data_dir: &PathBuf) -> Option<BrowserSettings> {
        let path = data_dir.join("settings.json");
        if path.exists() {
            let content = std::fs::read_to_string(&path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    
    /// Save settings to disk
    fn save_to_disk(&self) -> Result<(), String> {
        let path = self.data_dir.join("settings.json");
        let settings = self.settings.read().unwrap();
        let json = serde_json::to_string_pretty(&*settings).map_err(|e| e.to_string())?;
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Get all settings
    pub fn get_settings(&self) -> BrowserSettings {
        self.settings.read().unwrap().clone()
    }
    
    /// Update all settings
    pub fn update_settings(&self, new_settings: BrowserSettings) -> Result<(), String> {
        *self.settings.write().unwrap() = new_settings;
        self.save_to_disk()?;
        log::info!("Settings updated and saved");
        Ok(())
    }
    
    /// Update a single setting
    pub fn set_setting(&self, key: &str, value: bool) -> Result<(), String> {
        {
            let mut settings = self.settings.write().unwrap();
            match key {
                "torEnabled" => settings.tor_enabled = value,
                "httpsOnly" => settings.https_only = value,
                "blockTrackers" => settings.block_trackers = value,
                "blockMalware" => settings.block_malware = value,
                "scanDownloads" => settings.scan_downloads = value,
                "blockAds" => settings.block_ads = value,
                "stripMetadata" => settings.strip_metadata = value,
                "blockWebRTC" => settings.block_webrtc = value,
                "fakeGeolocation" => settings.fake_geolocation = value,
                "spoofFingerprint" => settings.spoof_fingerprint = value,
                "stripReferrer" => settings.strip_referrer = value,
                "partitionStorage" => settings.partition_storage = value,
                "autoRegenerateIdentity" => settings.auto_regenerate_identity = value,
                "secureKeyboard" => settings.secure_keyboard = value,
                _ => return Err(format!("Unknown setting: {}", key)),
            }
        }
        self.save_to_disk()?;
        log::info!("Setting '{}' updated to {}", key, value);
        Ok(())
    }
    
    /// Get a single setting value
    pub fn get_setting(&self, key: &str) -> Result<bool, String> {
        let settings = self.settings.read().unwrap();
        match key {
            "torEnabled" => Ok(settings.tor_enabled),
            "httpsOnly" => Ok(settings.https_only),
            "blockTrackers" => Ok(settings.block_trackers),
            "blockMalware" => Ok(settings.block_malware),
            "scanDownloads" => Ok(settings.scan_downloads),
            "blockAds" => Ok(settings.block_ads),
            "stripMetadata" => Ok(settings.strip_metadata),
            "blockWebRTC" => Ok(settings.block_webrtc),
            "fakeGeolocation" => Ok(settings.fake_geolocation),
            "spoofFingerprint" => Ok(settings.spoof_fingerprint),
            "stripReferrer" => Ok(settings.strip_referrer),
            "partitionStorage" => Ok(settings.partition_storage),
            "autoRegenerateIdentity" => Ok(settings.auto_regenerate_identity),
            "secureKeyboard" => Ok(settings.secure_keyboard),
            _ => Err(format!("Unknown setting: {}", key)),
        }
    }
    
    // Convenience methods for checking settings
    pub fn is_tor_enabled(&self) -> bool {
        self.settings.read().unwrap().tor_enabled
    }
    
    pub fn is_https_only(&self) -> bool {
        self.settings.read().unwrap().https_only
    }
    
    pub fn should_block_trackers(&self) -> bool {
        self.settings.read().unwrap().block_trackers
    }
    
    pub fn should_block_malware(&self) -> bool {
        self.settings.read().unwrap().block_malware
    }
    
    pub fn should_scan_downloads(&self) -> bool {
        self.settings.read().unwrap().scan_downloads
    }
    
    pub fn should_block_ads(&self) -> bool {
        self.settings.read().unwrap().block_ads
    }
    
    pub fn should_spoof_fingerprint(&self) -> bool {
        self.settings.read().unwrap().spoof_fingerprint
    }
    
    pub fn should_block_webrtc(&self) -> bool {
        self.settings.read().unwrap().block_webrtc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_settings() {
        let settings = BrowserSettings::default();
        assert!(settings.https_only);
        assert!(settings.block_trackers);
        assert!(settings.block_malware);
        assert!(!settings.tor_enabled); // Tor off by default
    }
}
