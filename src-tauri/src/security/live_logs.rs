//! Live Security Logs Service
//! Captures real-time security events and provides them to the frontend

use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Maximum logs to keep in memory
const MAX_LOGS: usize = 500;

/// Security log entry type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogType {
    Tracker,
    Malware,
    Antivirus,
    Vulnerability,
    Download,
    Privacy,
    Network,
    Certificate,
}

/// Security log severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogSeverity {
    Info,
    Warning,
    Blocked,
    Safe,
    Error,
}

/// Security log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLog {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub log_type: LogType,
    pub severity: LogSeverity,
    pub message: String,
    pub details: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}

/// Live security logs service
pub struct LiveSecurityLogs {
    logs: Arc<RwLock<VecDeque<SecurityLog>>>,
    stats: Arc<RwLock<LogStats>>,
}

/// Log statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogStats {
    pub trackers_blocked: u64,
    pub malware_blocked: u64,
    pub downloads_scanned: u64,
    pub threats_found: u64,
    pub https_upgrades: u64,
    pub fingerprint_blocks: u64,
}

impl LiveSecurityLogs {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_LOGS))),
            stats: Arc::new(RwLock::new(LogStats::default())),
        }
    }
    
    /// Add a new log entry
    pub fn log(&self, log_type: LogType, severity: LogSeverity, message: &str, details: Option<&str>, url: Option<&str>) {
        let entry = SecurityLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            log_type: log_type.clone(),
            severity: severity.clone(),
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            domain: url.and_then(|u| extract_domain(u)),
        };
        
        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            match (&log_type, &severity) {
                (LogType::Tracker, LogSeverity::Blocked) => stats.trackers_blocked += 1,
                (LogType::Malware, LogSeverity::Blocked) => stats.malware_blocked += 1,
                (LogType::Download, LogSeverity::Safe) => stats.downloads_scanned += 1,
                (LogType::Download, LogSeverity::Blocked) => stats.threats_found += 1,
                (LogType::Network, LogSeverity::Info) => stats.https_upgrades += 1,
                (LogType::Privacy, LogSeverity::Blocked) => stats.fingerprint_blocks += 1,
                _ => {}
            }
        }
        
        // Add to log queue
        {
            let mut logs = self.logs.write().unwrap();
            if logs.len() >= MAX_LOGS {
                logs.pop_front();
            }
            logs.push_back(entry.clone());
        }
        
        log::info!("[SECURITY] {:?} - {}", severity, message);
    }
    
    /// Log tracker blocked
    pub fn log_tracker_blocked(&self, domain: &str, url: &str) {
        self.log(
            LogType::Tracker,
            LogSeverity::Blocked,
            &format!("Tracker blocked: {}", domain),
            Some(&format!("Request to tracking domain prevented")),
            Some(url),
        );
    }
    
    /// Log malware blocked
    pub fn log_malware_blocked(&self, reason: &str, url: &str) {
        self.log(
            LogType::Malware,
            LogSeverity::Blocked,
            &format!("Malware URL blocked: {}", reason),
            Some("URL matched malware database"),
            Some(url),
        );
    }
    
    /// Log download scanned
    pub fn log_download_scanned(&self, filename: &str, result: &str) {
        let severity = if result == "clean" { LogSeverity::Safe } else { LogSeverity::Warning };
        self.log(
            LogType::Download,
            severity,
            &format!("Download scanned: {}", filename),
            Some(&format!("Result: {}", result)),
            None,
        );
    }
    
    /// Log HTTPS upgrade
    pub fn log_https_upgrade(&self, url: &str) {
        self.log(
            LogType::Network,
            LogSeverity::Info,
            "HTTP upgraded to HTTPS",
            Some(&format!("Connection secured")),
            Some(url),
        );
    }
    
    /// Log fingerprint protection
    pub fn log_fingerprint_blocked(&self, api: &str) {
        self.log(
            LogType::Privacy,
            LogSeverity::Blocked,
            &format!("Fingerprinting blocked: {}", api),
            Some("API access spoofed/blocked"),
            None,
        );
    }
    
    /// Get all logs
    pub fn get_logs(&self) -> Vec<SecurityLog> {
        self.logs.read().unwrap().iter().cloned().collect()
    }
    
    /// Get recent logs (last N entries)
    pub fn get_recent_logs(&self, count: usize) -> Vec<SecurityLog> {
        let logs = self.logs.read().unwrap();
        logs.iter().rev().take(count).cloned().collect()
    }
    
    /// Get stats
    pub fn get_stats(&self) -> LogStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Clear all logs
    pub fn clear_logs(&self) {
        self.logs.write().unwrap().clear();
    }
}

impl Default for LiveSecurityLogs {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract domain from URL
fn extract_domain(url: &str) -> Option<String> {
    url.replace("http://", "")
        .replace("https://", "")
        .split('/')
        .next()
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_creation() {
        let logs = LiveSecurityLogs::new();
        logs.log_tracker_blocked("googleanalytics.com", "https://example.com/page");
        
        let entries = logs.get_logs();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].message.contains("googleanalytics.com"));
    }
    
    #[test]
    fn test_stats() {
        let logs = LiveSecurityLogs::new();
        logs.log_tracker_blocked("tracker1.com", "https://test.com");
        logs.log_tracker_blocked("tracker2.com", "https://test.com");
        
        let stats = logs.get_stats();
        assert_eq!(stats.trackers_blocked, 2);
    }
}
