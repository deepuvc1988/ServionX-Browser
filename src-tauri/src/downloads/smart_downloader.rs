//! Smart Download Manager
//! Downloads files with progress tracking, scanning, and resume support

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

/// Download state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DownloadState {
    Pending,
    Downloading,
    Paused,
    Scanning,
    Completed,
    Failed,
    Cancelled,
}

/// Download item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub save_path: PathBuf,
    pub size_total: Option<u64>,
    pub size_downloaded: u64,
    pub state: DownloadState,
    pub speed_bps: u64,
    pub error: Option<String>,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    pub content_type: Option<String>,
    pub is_resumable: bool,
}

impl Download {
    pub fn progress(&self) -> f64 {
        match self.size_total {
            Some(total) if total > 0 => (self.size_downloaded as f64 / total as f64) * 100.0,
            _ => 0.0,
        }
    }
}

/// Smart download manager
pub struct SmartDownloader {
    downloads: Arc<RwLock<HashMap<String, Download>>>,
    download_dir: PathBuf,
    max_concurrent: usize,
    active_count: Arc<RwLock<usize>>,
}

impl SmartDownloader {
    pub fn new() -> Self {
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ServionX Downloads");
        
        // Create download directory
        let _ = std::fs::create_dir_all(&download_dir);
        
        Self {
            downloads: Arc::new(RwLock::new(HashMap::new())),
            download_dir,
            max_concurrent: 5,
            active_count: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Start a new download
    pub fn start_download(&self, url: &str, filename: Option<&str>) -> Result<Download, String> {
        let id = uuid::Uuid::new_v4().to_string();
        
        // Extract filename from URL if not provided
        let filename = filename
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                url.split('/').last()
                    .unwrap_or("download")
                    .split('?').next()
                    .unwrap_or("download")
                    .to_string()
            });
        
        let save_path = self.download_dir.join(&filename);
        
        let download = Download {
            id: id.clone(),
            url: url.to_string(),
            filename,
            save_path,
            size_total: None,
            size_downloaded: 0,
            state: DownloadState::Pending,
            speed_bps: 0,
            error: None,
            created_at: chrono::Utc::now().timestamp(),
            completed_at: None,
            content_type: None,
            is_resumable: false,
        };
        
        self.downloads.write().unwrap().insert(id.clone(), download.clone());
        
        log::info!("Download queued: {} -> {}", url, download.save_path.display());
        
        Ok(download)
    }
    
    /// Get all downloads
    pub fn get_downloads(&self) -> Vec<Download> {
        self.downloads.read().unwrap().values().cloned().collect()
    }
    
    /// Get a specific download
    pub fn get_download(&self, id: &str) -> Option<Download> {
        self.downloads.read().unwrap().get(id).cloned()
    }
    
    /// Update download progress
    pub fn update_progress(&self, id: &str, downloaded: u64, total: Option<u64>, speed: u64) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.size_downloaded = downloaded;
            if total.is_some() {
                download.size_total = total;
            }
            download.speed_bps = speed;
            download.state = DownloadState::Downloading;
        }
    }
    
    /// Complete download
    pub fn complete_download(&self, id: &str) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.state = DownloadState::Completed;
            download.completed_at = Some(chrono::Utc::now().timestamp());
            log::info!("Download completed: {}", download.filename);
        }
    }
    
    /// Fail download
    pub fn fail_download(&self, id: &str, error: &str) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.state = DownloadState::Failed;
            download.error = Some(error.to_string());
            log::error!("Download failed: {} - {}", download.filename, error);
        }
    }
    
    /// Cancel download
    pub fn cancel_download(&self, id: &str) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.state = DownloadState::Cancelled;
        }
    }
    
    /// Pause download
    pub fn pause_download(&self, id: &str) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            download.state = DownloadState::Paused;
        }
    }
    
    /// Resume download
    pub fn resume_download(&self, id: &str) {
        if let Some(download) = self.downloads.write().unwrap().get_mut(id) {
            if download.state == DownloadState::Paused {
                download.state = DownloadState::Pending;
            }
        }
    }
    
    /// Get download directory
    pub fn get_download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
    
    /// Remove completed downloads from list
    pub fn clear_completed(&self) {
        let mut downloads = self.downloads.write().unwrap();
        downloads.retain(|_, d| d.state != DownloadState::Completed);
    }
}

impl Default for SmartDownloader {
    fn default() -> Self {
        Self::new()
    }
}

/// Format bytes to human readable
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format speed to human readable
pub fn format_speed(bps: u64) -> String {
    format!("{}/s", format_bytes(bps))
}
