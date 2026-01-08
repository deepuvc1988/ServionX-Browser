//! Download Commands
//! Tauri commands for download and video grabbing functionality

use tauri::State;
use super::{SmartDownloader, Download, VideoGrabber, DetectedMedia};

// ========== DOWNLOAD COMMANDS ==========

/// Start a new download
#[tauri::command]
pub fn start_download(
    downloader: State<SmartDownloader>,
    url: String,
    filename: Option<String>,
) -> Result<Download, String> {
    downloader.start_download(&url, filename.as_deref())
}

/// Get all downloads
#[tauri::command]
pub fn get_downloads(downloader: State<SmartDownloader>) -> Vec<Download> {
    downloader.get_downloads()
}

/// Get a specific download
#[tauri::command]
pub fn get_download(downloader: State<SmartDownloader>, id: String) -> Option<Download> {
    downloader.get_download(&id)
}

/// Pause a download
#[tauri::command]
pub fn pause_download(downloader: State<SmartDownloader>, id: String) {
    downloader.pause_download(&id);
}

/// Resume a download
#[tauri::command]
pub fn resume_download(downloader: State<SmartDownloader>, id: String) {
    downloader.resume_download(&id);
}

/// Cancel a download
#[tauri::command]
pub fn cancel_download(downloader: State<SmartDownloader>, id: String) {
    downloader.cancel_download(&id);
}

/// Clear completed downloads
#[tauri::command]
pub fn clear_completed_downloads(downloader: State<SmartDownloader>) {
    downloader.clear_completed();
}

/// Get download directory
#[tauri::command]
pub fn get_download_directory(downloader: State<SmartDownloader>) -> String {
    downloader.get_download_dir().to_string_lossy().to_string()
}

// ========== VIDEO GRABBER COMMANDS ==========

/// Get detected videos for current page
#[tauri::command]
pub fn get_detected_videos(grabber: State<VideoGrabber>, page_url: String) -> Vec<DetectedMedia> {
    grabber.get_media_for_page(&page_url)
}

/// Get all detected videos
#[tauri::command]
pub fn get_all_detected_videos(grabber: State<VideoGrabber>) -> Vec<DetectedMedia> {
    grabber.get_all_media()
}

/// Get video detection script for injection
#[tauri::command]
pub fn get_video_detection_script() -> String {
    VideoGrabber::get_detection_script()
}

/// Report detected media from frontend
#[tauri::command]
pub fn report_detected_media(
    grabber: State<VideoGrabber>,
    media_type: String,
    url: String,
    page_url: String,
    title: Option<String>,
    duration: Option<u64>,
) {
    let media_type = match media_type.to_lowercase().as_str() {
        "hls" => super::MediaType::HLS,
        "dash" => super::MediaType::DASH,
        "audio" => super::MediaType::Audio,
        "direct" => super::MediaType::Direct,
        _ => super::MediaType::Video,
    };
    
    let media = DetectedMedia {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        url,
        page_url,
        media_type,
        duration,
        size: None,
        qualities: vec![],
        detected_at: chrono::Utc::now().timestamp(),
        thumbnail: None,
    };
    
    grabber.add_media(media);
}

/// Download detected video
#[tauri::command]
pub async fn download_video(
    downloader: State<'_, SmartDownloader>,
    url: String,
    filename: Option<String>,
) -> Result<Download, String> {
    // For direct videos, just start a normal download
    let download = downloader.start_download(&url, filename.as_deref())?;
    
    // TODO: For HLS/DASH, we need to:
    // 1. Fetch the playlist
    // 2. Parse segments
    // 3. Download all segments
    // 4. Merge with FFmpeg
    
    Ok(download)
}

/// Execute actual HTTP download
#[tauri::command]
pub async fn execute_download(
    downloader: State<'_, SmartDownloader>,
    id: String,
) -> Result<(), String> {
    let download = downloader.get_download(&id)
        .ok_or_else(|| "Download not found".to_string())?;
    
    log::info!("Starting download: {} -> {}", download.url, download.save_path.display());
    
    // Use reqwest to download
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client.get(&download.url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    // Check status
    if !response.status().is_success() {
        downloader.fail_download(&id, &format!("HTTP {}", response.status()));
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let total_size = response.content_length();
    downloader.update_progress(&id, 0, total_size, 0);
    
    // Download entire content
    let bytes = response.bytes().await.map_err(|e| format!("Download failed: {}", e))?;
    
    // Write to file
    tokio::fs::write(&download.save_path, &bytes)
        .await
        .map_err(|e| format!("Write failed: {}", e))?;
    
    // Update final progress
    downloader.update_progress(&id, bytes.len() as u64, Some(bytes.len() as u64), 0);
    downloader.complete_download(&id);
    
    log::info!("Download completed: {} ({} bytes)", download.filename, bytes.len());
    
    Ok(())
}
