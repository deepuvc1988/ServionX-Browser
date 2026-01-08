//! Settings Tauri Commands

use tauri::State;
use super::{SettingsManager, BrowserSettings};

/// Get all browser settings
#[tauri::command]
pub fn get_all_settings(settings: State<SettingsManager>) -> BrowserSettings {
    settings.get_settings()
}

/// Update all browser settings
#[tauri::command]
pub fn update_all_settings(
    settings: State<SettingsManager>,
    new_settings: BrowserSettings,
) -> Result<(), String> {
    settings.update_settings(new_settings)
}

/// Set a single setting by key
#[tauri::command]
pub fn set_setting(
    settings: State<SettingsManager>,
    key: String,
    value: bool,
) -> Result<(), String> {
    settings.set_setting(&key, value)
}

/// Get a single setting by key
#[tauri::command]
pub fn get_setting(
    settings: State<SettingsManager>,
    key: String,
) -> Result<bool, String> {
    settings.get_setting(&key)
}

/// Toggle a setting (flip its value)
#[tauri::command]
pub fn toggle_setting(
    settings: State<SettingsManager>,
    key: String,
) -> Result<bool, String> {
    let current = settings.get_setting(&key)?;
    let new_value = !current;
    settings.set_setting(&key, new_value)?;
    Ok(new_value)
}

/// Reset all settings to defaults
#[tauri::command]
pub fn reset_settings(settings: State<SettingsManager>) -> Result<BrowserSettings, String> {
    let defaults = BrowserSettings::default();
    settings.update_settings(defaults.clone())?;
    Ok(defaults)
}
