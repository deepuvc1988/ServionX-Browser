// Download Manager with Antivirus Scanning
// Manages downloads and scans files for threats

use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Download status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Scanning,
    Safe,
    Dangerous,
    Failed,
    Cancelled,
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanResult {
    Clean,
    Suspicious { reason: String },
    Malware { threat_name: String },
    Unknown,
}

/// Download item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub size: Option<u64>,
    pub downloaded: u64,
    pub status: DownloadStatus,
    pub scan_result: Option<ScanResult>,
    pub file_hash: Option<String>,
    pub started_at: i64,
    pub completed_at: Option<i64>,
}

/// Known malicious file hashes (sample - would be updated from threat intel feeds)
const MALWARE_HASHES: &[&str] = &[
    // Example malware hashes (EICAR test file)
    "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f",
    // Add more known malware hashes here
];

/// Dangerous file extensions
const DANGEROUS_EXTENSIONS: &[&str] = &[
    ".exe", ".msi", ".bat", ".cmd", ".ps1", ".vbs", ".js",
    ".jar", ".scr", ".pif", ".dll", ".com", ".hta",
    ".wsf", ".cpl", ".msc", ".gadget",
];

/// Suspicious patterns in filenames
const SUSPICIOUS_PATTERNS: &[&str] = &[
    "crack", "keygen", "patch", "loader", "activator",
    "warez", "serial", "hack", "cheat", "trainer",
];

/// Download manager with scanning
pub struct DownloadManager {
    downloads: RwLock<HashMap<String, DownloadItem>>,
    malware_hashes: RwLock<Vec<String>>,
    scan_enabled: RwLock<bool>,
    total_scanned: RwLock<u64>,
    threats_blocked: RwLock<u64>,
}

impl DownloadManager {
    pub fn new() -> Self {
        let mut hashes = Vec::new();
        for hash in MALWARE_HASHES {
            hashes.push(hash.to_string());
        }
        
        Self {
            downloads: RwLock::new(HashMap::new()),
            malware_hashes: RwLock::new(hashes),
            scan_enabled: RwLock::new(true),
            total_scanned: RwLock::new(0),
            threats_blocked: RwLock::new(0),
        }
    }
    
    /// Add a new download
    pub fn add_download(&self, url: &str, filename: &str) -> DownloadItem {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        let download = DownloadItem {
            id: id.clone(),
            url: url.to_string(),
            filename: filename.to_string(),
            size: None,
            downloaded: 0,
            status: DownloadStatus::Pending,
            scan_result: None,
            file_hash: None,
            started_at: now,
            completed_at: None,
        };
        
        self.downloads.write().unwrap().insert(id, download.clone());
        download
    }
    
    /// Pre-scan a URL before downloading
    pub fn prescan_url(&self, url: &str, filename: &str) -> ScanResult {
        let url_lower = url.to_lowercase();
        let filename_lower = filename.to_lowercase();
        
        // Check for dangerous extensions
        for ext in DANGEROUS_EXTENSIONS {
            if filename_lower.ends_with(ext) {
                log::warn!("Dangerous file extension detected: {}", ext);
                return ScanResult::Suspicious {
                    reason: format!("Potentially dangerous file type: {}", ext),
                };
            }
        }
        
        // Check for suspicious patterns
        for pattern in SUSPICIOUS_PATTERNS {
            if filename_lower.contains(pattern) || url_lower.contains(pattern) {
                log::warn!("Suspicious pattern detected: {}", pattern);
                return ScanResult::Suspicious {
                    reason: format!("Suspicious filename pattern: {}", pattern),
                };
            }
        }
        
        // Check URL against known bad sources
        let bad_tlds = [".tk", ".ml", ".ga", ".cf", ".gq"];
        for tld in bad_tlds {
            if url_lower.contains(tld) {
                return ScanResult::Suspicious {
                    reason: format!("High-risk domain TLD: {}", tld),
                };
            }
        }
        
        ScanResult::Clean
    }
    
    /// Scan file content by hash
    pub fn scan_file_hash(&self, file_hash: &str) -> ScanResult {
        *self.total_scanned.write().unwrap() += 1;
        
        let hashes = self.malware_hashes.read().unwrap();
        
        for known_hash in hashes.iter() {
            if file_hash.to_lowercase() == known_hash.to_lowercase() {
                *self.threats_blocked.write().unwrap() += 1;
                log::warn!("Known malware hash detected: {}", file_hash);
                return ScanResult::Malware {
                    threat_name: "Known Malware".to_string(),
                };
            }
        }
        
        ScanResult::Clean
    }
    
    /// Calculate SHA256 hash of data
    pub fn calculate_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    /// Add malware hash to database
    pub fn add_malware_hash(&self, hash: &str) {
        self.malware_hashes.write().unwrap().push(hash.to_lowercase());
    }
    
    /// Update download status
    pub fn update_download(&self, id: &str, status: DownloadStatus, scan_result: Option<ScanResult>) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.status = status;
            if scan_result.is_some() {
                download.scan_result = scan_result;
            }
            if status == DownloadStatus::Safe || status == DownloadStatus::Dangerous {
                download.completed_at = Some(chrono::Utc::now().timestamp());
            }
        }
    }
    
    /// Get all downloads
    pub fn get_downloads(&self) -> Vec<DownloadItem> {
        self.downloads.read().unwrap().values().cloned().collect()
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> DownloadStats {
        let downloads = self.downloads.read().unwrap();
        
        DownloadStats {
            total_downloads: downloads.len(),
            total_scanned: *self.total_scanned.read().unwrap(),
            threats_blocked: *self.threats_blocked.read().unwrap(),
            pending: downloads.values().filter(|d| d.status == DownloadStatus::Pending).count(),
            in_progress: downloads.values().filter(|d| d.status == DownloadStatus::Downloading).count(),
        }
    }
    
    /// Enable/disable scanning
    pub fn set_scan_enabled(&self, enabled: bool) {
        *self.scan_enabled.write().unwrap() = enabled;
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Download statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    pub total_downloads: usize,
    pub total_scanned: u64,
    pub threats_blocked: u64,
    pub pending: usize,
    pub in_progress: usize,
}

/// Threat intelligence feeds
pub const THREAT_INTEL_FEEDS: &[(&str, &str)] = &[
    // URLHaus - Malware URLs
    ("urlhaus", "https://urlhaus.abuse.ch/downloads/csv_recent/"),
    // Abuse.ch Malware Bazaar - File hashes
    ("malwarebazaar", "https://bazaar.abuse.ch/export/txt/sha256/recent/"),
    // PhishTank
    ("phishtank", "https://data.phishtank.com/data/online-valid.csv"),
];
