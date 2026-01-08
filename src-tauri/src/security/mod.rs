// Security Module
// User profiles, encryption, secure storage, and advanced security features

pub mod profile;
pub mod encryption;
pub mod vulnerability_scanner;
pub mod download_manager;
pub mod tor_manager;
pub mod network_security;
pub mod certificate_transparency;
pub mod live_logs;
pub mod commands;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub use profile::ProfileManager;
pub use encryption::Encryption;
pub use vulnerability_scanner::VulnerabilityScanner;
pub use download_manager::DownloadManager;
pub use tor_manager::TorManager;
pub use network_security::NetworkSecurity;
pub use certificate_transparency::CertificateTransparency;
pub use live_logs::{LiveSecurityLogs, SecurityLog, LogType, LogSeverity, LogStats};

/// Encrypted log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Security,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Security => write!(f, "SECURITY"),
        }
    }
}
