// Tools Commands
// Tauri commands for SSH, SFTP, and network tools

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

use crate::tools::ssh::{SshClient, SshConfig, SshConnectionInfo, CommandResult, SftpEntry, TransferProgress};
use crate::tools::network::{NetworkTools, PingResult, PortScanResult, DnsResult, HttpResult};

// Global instances
static SSH_CLIENT: Lazy<Arc<Mutex<SshClient>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SshClient::new()))
});

static NETWORK_TOOLS: Lazy<NetworkTools> = Lazy::new(|| NetworkTools::new());

/// Connect to SSH server
#[tauri::command]
pub async fn ssh_connect(
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    private_key: Option<String>,
) -> Result<SshConnectionInfo, String> {
    let client = SSH_CLIENT.lock().await;
    
    let config = SshConfig {
        host,
        port,
        username,
        password,
        private_key,
    };
    
    client.connect(config).await
}

/// Disconnect from SSH server
#[tauri::command]
pub async fn ssh_disconnect(connection_id: String) -> Result<(), String> {
    let client = SSH_CLIENT.lock().await;
    client.disconnect(&connection_id)
}

/// Execute SSH command
#[tauri::command]
pub async fn ssh_execute(connection_id: String, command: String) -> Result<CommandResult, String> {
    let client = SSH_CLIENT.lock().await;
    client.execute(&connection_id, &command).await
}

/// List SFTP directory
#[tauri::command]
pub async fn sftp_list_directory(connection_id: String, path: String) -> Result<Vec<SftpEntry>, String> {
    let client = SSH_CLIENT.lock().await;
    client.sftp_list(&connection_id, &path).await
}

/// Download file via SFTP
#[tauri::command]
pub async fn sftp_download(
    connection_id: String,
    remote_path: String,
    local_path: String,
) -> Result<TransferProgress, String> {
    let client = SSH_CLIENT.lock().await;
    client.sftp_download(&connection_id, &remote_path, &local_path).await
}

/// Upload file via SFTP
#[tauri::command]
pub async fn sftp_upload(
    connection_id: String,
    local_path: String,
    remote_path: String,
) -> Result<TransferProgress, String> {
    let client = SSH_CLIENT.lock().await;
    client.sftp_upload(&connection_id, &local_path, &remote_path).await
}

/// Ping a host
#[tauri::command]
pub async fn network_ping(host: String, count: Option<u32>) -> PingResult {
    NETWORK_TOOLS.ping(&host, count.unwrap_or(4)).await
}

/// Scan ports
#[tauri::command]
pub fn network_port_scan(host: String, ports: Vec<u16>, timeout_ms: Option<u64>) -> Vec<PortScanResult> {
    NETWORK_TOOLS.scan_ports(&host, &ports, timeout_ms.unwrap_or(1000))
}

/// DNS lookup
#[tauri::command]
pub async fn network_dns_lookup(domain: String) -> Result<DnsResult, String> {
    NETWORK_TOOLS.dns_lookup(&domain).await
}

/// HTTP request
#[tauri::command]
pub async fn http_request(
    url: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<HttpResult, String> {
    NETWORK_TOOLS.http_request(&url, &method, headers, body).await
}
