// Security Commands
// Tauri commands for security operations

use tauri::State;
use crate::security::{ProfileManager, LogEntry, LogLevel};

/// Unlock settings with password
#[tauri::command]
pub fn unlock_settings(
    profile: State<ProfileManager>,
    password: String,
) -> Result<bool, String> {
    profile.unlock(&password)
}

/// Lock settings
#[tauri::command]
pub fn lock_settings(profile: State<ProfileManager>) {
    profile.lock();
}

/// Check if settings are locked
#[tauri::command]
pub fn is_settings_locked(profile: State<ProfileManager>) -> bool {
    profile.is_locked()
}

/// Set master password (change settings password)
#[tauri::command]
pub fn set_master_password(
    profile: State<ProfileManager>,
    password: String,
) -> Result<(), String> {
    profile.set_master_password(&password)
}

/// Verify master password
#[tauri::command]
pub fn verify_master_password(
    profile: State<ProfileManager>,
    password: String,
) -> Result<bool, String> {
    profile.verify_master_password(&password)
}

/// Unlock logs with logs password
#[tauri::command]
pub fn unlock_logs(
    profile: State<ProfileManager>,
    password: String,
) -> Result<bool, String> {
    profile.unlock_logs(&password)
}

/// Lock logs
#[tauri::command]
pub fn lock_logs(profile: State<ProfileManager>) {
    profile.lock_logs();
}

/// Check if logs are locked
#[tauri::command]
pub fn is_logs_locked(profile: State<ProfileManager>) -> bool {
    profile.is_logs_locked()
}

/// Set logs password
#[tauri::command]
pub fn set_logs_password(
    profile: State<ProfileManager>,
    password: String,
) -> Result<(), String> {
    profile.set_logs_password(&password)
}

/// Verify logs password
#[tauri::command]
pub fn verify_logs_password(
    profile: State<ProfileManager>,
    password: String,
) -> Result<bool, String> {
    profile.verify_logs_password(&password)
}

/// Get encrypted logs (requires logs to be unlocked)
#[tauri::command]
pub fn get_encrypted_logs(
    profile: State<ProfileManager>,
) -> Result<Vec<LogEntry>, String> {
    profile.get_logs()
}

/// Add a log entry
#[tauri::command]
pub fn add_log_entry(
    profile: State<ProfileManager>,
    level: String,
    category: String,
    message: String,
) {
    let log_level = match level.to_lowercase().as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warning" | "warn" => LogLevel::Warning,
        "error" => LogLevel::Error,
        "security" => LogLevel::Security,
        _ => LogLevel::Info,
    };
    
    profile.add_log(log_level, &category, &message);
}

/// Check if profile exists (first run check)
#[tauri::command]
pub fn has_profile(profile: State<ProfileManager>) -> bool {
    profile.has_profile()
}

// ========== LIVE SECURITY LOGS COMMANDS ==========

use super::live_logs::{LiveSecurityLogs, SecurityLog as LiveLog, LogStats};

/// Get all live security logs
#[tauri::command]
pub fn get_live_logs(logs: State<LiveSecurityLogs>) -> Vec<LiveLog> {
    logs.get_logs()
}

/// Get recent live security logs
#[tauri::command]
pub fn get_recent_live_logs(logs: State<LiveSecurityLogs>, count: usize) -> Vec<LiveLog> {
    logs.get_recent_logs(count)
}

/// Get security log statistics
#[tauri::command]
pub fn get_log_stats(logs: State<LiveSecurityLogs>) -> LogStats {
    logs.get_stats()
}

/// Clear all live logs
#[tauri::command]
pub fn clear_live_logs(logs: State<LiveSecurityLogs>) {
    logs.clear_logs();
}
