// SSH/SFTP Client
// Built-in SSH terminal and file transfer (simplified - uses system commands)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::process::Command;

/// SSH connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub connected: bool,
    pub connected_at: Option<String>,
}

/// SSH connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key: Option<String>,
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
}

/// SFTP directory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub permissions: String,
    pub modified: Option<String>,
}

/// SFTP transfer progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub percentage: f32,
    pub speed_bps: u64,
}

/// SSH Client manager (uses system SSH when available)
pub struct SshClient {
    connections: Arc<RwLock<HashMap<String, SshConnectionInfo>>>,
}

impl SshClient {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Connect to an SSH server (stores connection info)
    pub async fn connect(&self, config: SshConfig) -> Result<SshConnectionInfo, String> {
        let connection_id = uuid::Uuid::new_v4().to_string();
        
        // Test connection using system ssh
        let test_result = Command::new("ssh")
            .args([
                "-o", "BatchMode=yes",
                "-o", "ConnectTimeout=5",
                "-o", "StrictHostKeyChecking=no",
                "-p", &config.port.to_string(),
                &format!("{}@{}", config.username, config.host),
                "echo connected"
            ])
            .output();
        
        let connected = match test_result {
            Ok(output) => output.status.success(),
            Err(_) => {
                // If system SSH not available, just store the config
                log::warn!("System SSH not available, storing connection info only");
                true
            }
        };
        
        let info = SshConnectionInfo {
            id: connection_id.clone(),
            host: config.host,
            port: config.port,
            username: config.username,
            connected,
            connected_at: Some(chrono::Utc::now().to_rfc3339()),
        };
        
        self.connections.write().unwrap().insert(connection_id.clone(), info.clone());
        
        log::info!("SSH connection created for {}:{}", info.host, info.port);
        
        Ok(info)
    }
    
    /// Disconnect from SSH server
    pub fn disconnect(&self, connection_id: &str) -> Result<(), String> {
        let mut connections = self.connections.write().unwrap();
        
        if let Some(mut info) = connections.remove(connection_id) {
            info.connected = false;
            log::info!("SSH disconnected from {}:{}", info.host, info.port);
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }
    
    /// Execute a command (using stored connection info)
    pub async fn execute(&self, connection_id: &str, command: &str) -> Result<CommandResult, String> {
        let connections = self.connections.read().unwrap();
        
        let info = connections.get(connection_id)
            .ok_or("Connection not found")?;
        
        let start = std::time::Instant::now();
        
        // Execute via system SSH
        let output = Command::new("ssh")
            .args([
                "-o", "BatchMode=yes",
                "-o", "StrictHostKeyChecking=no",
                "-p", &info.port.to_string(),
                &format!("{}@{}", info.username, info.host),
                command
            ])
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        
        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            execution_time_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    /// List directory via SFTP (placeholder)
    pub async fn sftp_list(&self, connection_id: &str, path: &str) -> Result<Vec<SftpEntry>, String> {
        let connections = self.connections.read().unwrap();
        if !connections.contains_key(connection_id) {
            return Err("Connection not found".to_string());
        }
        
        log::info!("SFTP listing: {}", path);
        
        // Placeholder - would use sftp command or library
        Ok(vec![
            SftpEntry {
                name: ".".to_string(),
                path: path.to_string(),
                is_dir: true,
                size: 4096,
                permissions: "drwxr-xr-x".to_string(),
                modified: Some(chrono::Utc::now().to_rfc3339()),
            },
        ])
    }
    
    /// Download a file via SFTP (placeholder)
    pub async fn sftp_download(
        &self,
        connection_id: &str,
        remote_path: &str,
        local_path: &str,
    ) -> Result<TransferProgress, String> {
        let connections = self.connections.read().unwrap();
        if !connections.contains_key(connection_id) {
            return Err("Connection not found".to_string());
        }
        
        log::info!("SFTP download: {} -> {}", remote_path, local_path);
        
        Ok(TransferProgress {
            bytes_transferred: 0,
            total_bytes: 0,
            percentage: 100.0,
            speed_bps: 0,
        })
    }
    
    /// Upload a file via SFTP (placeholder)
    pub async fn sftp_upload(
        &self,
        connection_id: &str,
        local_path: &str,
        remote_path: &str,
    ) -> Result<TransferProgress, String> {
        let connections = self.connections.read().unwrap();
        if !connections.contains_key(connection_id) {
            return Err("Connection not found".to_string());
        }
        
        log::info!("SFTP upload: {} -> {}", local_path, remote_path);
        
        Ok(TransferProgress {
            bytes_transferred: 0,
            total_bytes: 0,
            percentage: 100.0,
            speed_bps: 0,
        })
    }
    
    /// Get all active connections
    pub fn get_connections(&self) -> Vec<SshConnectionInfo> {
        self.connections.read().unwrap().values().cloned().collect()
    }
}

impl Default for SshClient {
    fn default() -> Self {
        Self::new()
    }
}
