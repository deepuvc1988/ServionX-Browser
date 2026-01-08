// Profile Manager
// Manages encrypted user profiles and settings with DEFAULT PASSWORDS

use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{LogEntry, LogLevel};
use super::encryption::Encryption;

/// Default passwords (can be changed by user)
pub const DEFAULT_SETTINGS_PASSWORD: &str = "ServionX2024";
pub const DEFAULT_LOGS_PASSWORD: &str = "SecureLogs123";

/// User profile with encrypted settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub password_hash: String,
    pub salt: String,
    pub logs_password_hash: String,
    pub logs_salt: String,
}

/// Profile settings (stored encrypted)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileSettings {
    pub auto_regenerate_identity: bool,
    pub regenerate_interval_minutes: u32,
    pub block_webrtc: bool,
    pub block_geolocation: bool,
    pub strip_upload_metadata: bool,
    pub inject_fake_metadata: bool,
    pub enable_secure_keyboard: bool,
    pub default_search_engine: String,
}

/// Manages user profiles with encryption
pub struct ProfileManager {
    data_dir: PathBuf,
    encryption: Encryption,
    current_profile: Arc<RwLock<Option<UserProfile>>>,
    is_unlocked: Arc<RwLock<bool>>,
    is_logs_unlocked: Arc<RwLock<bool>>,
    session_key: Arc<RwLock<Option<Vec<u8>>>>,
    logs: Arc<RwLock<Vec<LogEntry>>>,
}

impl ProfileManager {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ServionX Browser");
        
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        
        let manager = Self {
            data_dir,
            encryption: Encryption::new(),
            current_profile: Arc::new(RwLock::new(None)),
            is_unlocked: Arc::new(RwLock::new(false)),
            is_logs_unlocked: Arc::new(RwLock::new(false)),
            session_key: Arc::new(RwLock::new(None)),
            logs: Arc::new(RwLock::new(Vec::new())),
        };
        
        // Auto-create default profile with default passwords if none exists
        if !manager.has_profile() {
            if let Err(e) = manager.create_default_profile() {
                log::warn!("Could not create default profile: {}", e);
            } else {
                log::info!("Created default profile with default passwords");
                log::info!("Settings password: {}", DEFAULT_SETTINGS_PASSWORD);
                log::info!("Logs password: {}", DEFAULT_LOGS_PASSWORD);
            }
        }
        
        Ok(manager)
    }
    
    /// Create default profile with default passwords
    fn create_default_profile(&self) -> Result<(), String> {
        let (settings_hash, settings_salt) = self.encryption.hash_password(DEFAULT_SETTINGS_PASSWORD)?;
        let (logs_hash, logs_salt) = self.encryption.hash_password(DEFAULT_LOGS_PASSWORD)?;
        
        let profile = UserProfile {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default".to_string(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            password_hash: settings_hash,
            salt: settings_salt,
            logs_password_hash: logs_hash,
            logs_salt: logs_salt,
        };
        
        self.save_profile(&profile)?;
        Ok(())
    }
    
    /// Check if settings are currently locked
    pub fn is_locked(&self) -> bool {
        !*self.is_unlocked.read().unwrap()
    }
    
    /// Check if logs are currently locked
    pub fn is_logs_locked(&self) -> bool {
        !*self.is_logs_unlocked.read().unwrap()
    }
    
    /// Set the master password (change settings password)
    pub fn set_master_password(&self, password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        let (hash, salt) = self.encryption.hash_password(password)?;
        
        // Load existing profile or create new
        let mut profile = self.load_profile()?.unwrap_or_else(|| {
            let (logs_hash, logs_salt) = self.encryption.hash_password(DEFAULT_LOGS_PASSWORD).unwrap();
            UserProfile {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Default".to_string(),
                created_at: Utc::now(),
                last_accessed: Utc::now(),
                password_hash: String::new(),
                salt: String::new(),
                logs_password_hash: logs_hash,
                logs_salt: logs_salt,
            }
        });
        
        profile.password_hash = hash;
        profile.salt = salt.clone();
        
        // Save profile
        self.save_profile(&profile)?;
        
        // Auto-unlock after setting password
        *self.current_profile.write().unwrap() = Some(profile);
        *self.is_unlocked.write().unwrap() = true;
        
        // Generate session key
        let session_key = self.encryption.derive_key(password, &salt)?;
        *self.session_key.write().unwrap() = Some(session_key);
        
        self.add_log(LogLevel::Security, "Profile", "Settings password changed");
        
        Ok(())
    }
    
    /// Set logs password
    pub fn set_logs_password(&self, password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        let (hash, salt) = self.encryption.hash_password(password)?;
        
        let mut profile = self.load_profile()?.ok_or("No profile found")?;
        profile.logs_password_hash = hash;
        profile.logs_salt = salt;
        
        self.save_profile(&profile)?;
        
        self.add_log(LogLevel::Security, "Profile", "Logs password changed");
        
        Ok(())
    }
    
    /// Verify the master password
    pub fn verify_master_password(&self, password: &str) -> Result<bool, String> {
        let profile = self.load_profile()?;
        
        if let Some(p) = profile {
            let is_valid = self.encryption.verify_password(password, &p.password_hash, &p.salt)?;
            
            if is_valid {
                self.add_log(LogLevel::Security, "Auth", "Settings password verified");
            } else {
                self.add_log(LogLevel::Security, "Auth", "Invalid settings password attempt");
            }
            
            Ok(is_valid)
        } else {
            Err("No profile found".to_string())
        }
    }
    
    /// Verify logs password
    pub fn verify_logs_password(&self, password: &str) -> Result<bool, String> {
        let profile = self.load_profile()?;
        
        if let Some(p) = profile {
            let is_valid = self.encryption.verify_password(password, &p.logs_password_hash, &p.logs_salt)?;
            
            if is_valid {
                self.add_log(LogLevel::Security, "Auth", "Logs password verified");
            } else {
                self.add_log(LogLevel::Security, "Auth", "Invalid logs password attempt");
            }
            
            Ok(is_valid)
        } else {
            Err("No profile found".to_string())
        }
    }
    
    /// Unlock settings with password
    pub fn unlock(&self, password: &str) -> Result<bool, String> {
        if self.verify_master_password(password)? {
            *self.is_unlocked.write().unwrap() = true;
            
            // Load profile
            if let Some(profile) = self.load_profile()? {
                let salt = profile.salt.clone();
                *self.current_profile.write().unwrap() = Some(profile);
                
                // Generate session key
                let session_key = self.encryption.derive_key(password, &salt)?;
                *self.session_key.write().unwrap() = Some(session_key);
            }
            
            self.add_log(LogLevel::Security, "Auth", "Settings unlocked");
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Unlock logs with logs password
    pub fn unlock_logs(&self, password: &str) -> Result<bool, String> {
        if self.verify_logs_password(password)? {
            *self.is_logs_unlocked.write().unwrap() = true;
            self.add_log(LogLevel::Security, "Auth", "Logs unlocked");
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Lock settings
    pub fn lock(&self) {
        *self.is_unlocked.write().unwrap() = false;
        *self.session_key.write().unwrap() = None;
        self.add_log(LogLevel::Security, "Auth", "Settings locked");
    }
    
    /// Lock logs
    pub fn lock_logs(&self) {
        *self.is_logs_unlocked.write().unwrap() = false;
        self.add_log(LogLevel::Security, "Auth", "Logs locked");
    }
    
    /// Add a log entry
    pub fn add_log(&self, level: LogLevel, category: &str, message: &str) {
        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level,
            category: category.to_string(),
            message: message.to_string(),
            details: None,
        };
        
        self.logs.write().unwrap().push(entry);
    }
    
    /// Add a log entry with details
    pub fn add_log_with_details(&self, level: LogLevel, category: &str, message: &str, details: &str) {
        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level,
            category: category.to_string(),
            message: message.to_string(),
            details: Some(details.to_string()),
        };
        
        self.logs.write().unwrap().push(entry);
    }
    
    /// Get all logs (requires logs to be unlocked)
    pub fn get_logs(&self) -> Result<Vec<LogEntry>, String> {
        if self.is_logs_locked() {
            return Err("Logs are locked. Enter logs password to view.".to_string());
        }
        
        Ok(self.logs.read().unwrap().clone())
    }
    
    /// Save profile to disk
    fn save_profile(&self, profile: &UserProfile) -> Result<(), String> {
        let profile_path = self.data_dir.join("profile.json");
        let json = serde_json::to_string_pretty(profile)
            .map_err(|e| e.to_string())?;
        
        std::fs::write(&profile_path, json)
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Load profile from disk
    fn load_profile(&self) -> Result<Option<UserProfile>, String> {
        let profile_path = self.data_dir.join("profile.json");
        
        if !profile_path.exists() {
            return Ok(None);
        }
        
        let json = std::fs::read_to_string(&profile_path)
            .map_err(|e| e.to_string())?;
        
        let profile: UserProfile = serde_json::from_str(&json)
            .map_err(|e| e.to_string())?;
        
        Ok(Some(profile))
    }
    
    /// Check if a profile exists
    pub fn has_profile(&self) -> bool {
        self.data_dir.join("profile.json").exists()
    }
    
    /// Get default passwords (for display in logs)
    pub fn get_default_passwords() -> (&'static str, &'static str) {
        (DEFAULT_SETTINGS_PASSWORD, DEFAULT_LOGS_PASSWORD)
    }
}
