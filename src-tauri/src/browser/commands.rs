// Browser Commands
// Tauri commands for browser tab management

use tauri::{AppHandle, State};
use super::{WebViewManager, BrowserTab};

/// Create a new browser tab
#[tauri::command]
pub async fn create_browser_tab(
    app: AppHandle,
    manager: State<'_, WebViewManager>,
    tab_id: String,
    url: String,
) -> Result<BrowserTab, String> {
    manager.create_tab(&app, &tab_id, &url)
}

/// Navigate to URL in a tab
#[tauri::command]
pub async fn navigate_tab(
    app: AppHandle,
    manager: State<'_, WebViewManager>,
    tab_id: String,
    url: String,
) -> Result<(), String> {
    manager.navigate(&app, &tab_id, &url)
}

/// Close a browser tab
#[tauri::command]
pub async fn close_browser_tab(
    app: AppHandle,
    manager: State<'_, WebViewManager>,
    tab_id: String,
) -> Result<(), String> {
    manager.close_tab(&app, &tab_id)
}

/// Get all tabs
#[tauri::command]
pub fn get_browser_tabs(manager: State<'_, WebViewManager>) -> Vec<BrowserTab> {
    manager.get_tabs()
}

/// Update tab info (called from webview)
#[tauri::command]
pub fn update_browser_tab(
    manager: State<'_, WebViewManager>,
    tab_id: String,
    title: Option<String>,
    is_loading: Option<bool>,
) {
    manager.update_tab(&tab_id, title, is_loading);
}
