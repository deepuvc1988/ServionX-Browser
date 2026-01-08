//! Video Grabber - IDM-like video download capability
//! Detects and downloads videos from streaming platforms

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

/// Detected media type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Video,
    Audio,
    HLS,      // .m3u8 streams
    DASH,     // .mpd streams
    Direct,   // Direct video file
}

/// Quality level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityLevel {
    pub resolution: String,      // e.g., "1080p", "720p"
    pub bitrate: Option<u64>,    // bits per second
    pub url: String,
    pub media_type: MediaType,
}

/// Detected video/media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedMedia {
    pub id: String,
    pub title: Option<String>,
    pub url: String,
    pub page_url: String,
    pub media_type: MediaType,
    pub duration: Option<u64>,   // seconds
    pub size: Option<u64>,       // bytes
    pub qualities: Vec<QualityLevel>,
    pub detected_at: i64,
    pub thumbnail: Option<String>,
}

/// Video grabber service
pub struct VideoGrabber {
    detected_media: Arc<RwLock<HashMap<String, DetectedMedia>>>,
    active_page: RwLock<Option<String>>,
}

impl VideoGrabber {
    pub fn new() -> Self {
        Self {
            detected_media: Arc::new(RwLock::new(HashMap::new())),
            active_page: RwLock::new(None),
        }
    }
    
    /// Set the active page URL
    pub fn set_active_page(&self, url: &str) {
        *self.active_page.write().unwrap() = Some(url.to_string());
    }
    
    /// Add detected media
    pub fn add_media(&self, media: DetectedMedia) {
        let id = media.id.clone();
        self.detected_media.write().unwrap().insert(id.clone(), media);
        log::info!("Video detected: {}", id);
    }
    
    /// Get all detected media for current page
    pub fn get_media_for_page(&self, page_url: &str) -> Vec<DetectedMedia> {
        self.detected_media
            .read()
            .unwrap()
            .values()
            .filter(|m| m.page_url == page_url)
            .cloned()
            .collect()
    }
    
    /// Get all detected media
    pub fn get_all_media(&self) -> Vec<DetectedMedia> {
        self.detected_media.read().unwrap().values().cloned().collect()
    }
    
    /// Clear old media (older than 1 hour)
    pub fn cleanup_old(&self) {
        let cutoff = chrono::Utc::now().timestamp() - 3600;
        let mut media = self.detected_media.write().unwrap();
        media.retain(|_, m| m.detected_at > cutoff);
    }
    
    /// Generate JavaScript to inject for video detection
    pub fn get_detection_script() -> String {
        r#"
(function() {
    'use strict';
    
    console.log('[ServionX VideoGrabber] Initializing...');
    
    const detected = new Set();
    
    // Report detected media to backend
    function reportMedia(media) {
        if (detected.has(media.url)) return;
        detected.add(media.url);
        
        console.log('[ServionX VideoGrabber] Detected:', media);
        
        // Send to backend via custom event
        window.__SERVIONX_DETECTED_MEDIA__ = window.__SERVIONX_DETECTED_MEDIA__ || [];
        window.__SERVIONX_DETECTED_MEDIA__.push(media);
        
        // Dispatch event for Tauri to catch
        window.dispatchEvent(new CustomEvent('servionx-media-detected', { 
            detail: media 
        }));
    }
    
    // Detect video/audio elements
    function scanMediaElements() {
        // Video elements
        document.querySelectorAll('video').forEach(video => {
            if (video.src && video.src.startsWith('http')) {
                reportMedia({
                    type: 'video',
                    url: video.src,
                    duration: video.duration || null,
                    width: video.videoWidth,
                    height: video.videoHeight,
                    pageUrl: window.location.href
                });
            }
            
            // Check source elements
            video.querySelectorAll('source').forEach(source => {
                if (source.src && source.src.startsWith('http')) {
                    reportMedia({
                        type: 'video',
                        url: source.src,
                        mediaType: source.type || 'video/mp4',
                        pageUrl: window.location.href
                    });
                }
            });
        });
        
        // Audio elements
        document.querySelectorAll('audio').forEach(audio => {
            if (audio.src && audio.src.startsWith('http')) {
                reportMedia({
                    type: 'audio',
                    url: audio.src,
                    duration: audio.duration || null,
                    pageUrl: window.location.href
                });
            }
        });
    }
    
    // Intercept XHR to detect m3u8/mpd streams
    const originalXHR = window.XMLHttpRequest;
    window.XMLHttpRequest = function() {
        const xhr = new originalXHR();
        const originalOpen = xhr.open;
        
        xhr.open = function(method, url) {
            if (typeof url === 'string') {
                const urlLower = url.toLowerCase();
                
                // HLS streams
                if (urlLower.includes('.m3u8')) {
                    reportMedia({
                        type: 'hls',
                        url: url,
                        pageUrl: window.location.href
                    });
                }
                
                // DASH streams
                if (urlLower.includes('.mpd')) {
                    reportMedia({
                        type: 'dash',
                        url: url,
                        pageUrl: window.location.href
                    });
                }
                
                // Direct video files
                if (urlLower.match(/\.(mp4|webm|mkv|avi|mov|flv)(\?|$)/)) {
                    reportMedia({
                        type: 'direct',
                        url: url,
                        pageUrl: window.location.href
                    });
                }
            }
            
            return originalOpen.apply(this, arguments);
        };
        
        return xhr;
    };
    
    // Intercept fetch to detect streams
    const originalFetch = window.fetch;
    window.fetch = function(input, init) {
        const url = typeof input === 'string' ? input : input.url;
        
        if (url) {
            const urlLower = url.toLowerCase();
            
            if (urlLower.includes('.m3u8')) {
                reportMedia({ type: 'hls', url: url, pageUrl: window.location.href });
            }
            if (urlLower.includes('.mpd')) {
                reportMedia({ type: 'dash', url: url, pageUrl: window.location.href });
            }
            if (urlLower.match(/\.(mp4|webm|mkv|avi|mov|flv)(\?|$)/)) {
                reportMedia({ type: 'direct', url: url, pageUrl: window.location.href });
            }
        }
        
        return originalFetch.apply(this, arguments);
    };
    
    // Scan on load and periodically
    scanMediaElements();
    setInterval(scanMediaElements, 3000);
    
    // Watch for new video elements
    const observer = new MutationObserver((mutations) => {
        mutations.forEach(mutation => {
            mutation.addedNodes.forEach(node => {
                if (node.nodeName === 'VIDEO' || node.nodeName === 'AUDIO') {
                    scanMediaElements();
                }
                if (node.querySelectorAll) {
                    const media = node.querySelectorAll('video, audio');
                    if (media.length > 0) {
                        scanMediaElements();
                    }
                }
            });
        });
    });
    
    observer.observe(document.body, { childList: true, subtree: true });
    
    console.log('[ServionX VideoGrabber] Ready - monitoring for videos');
})();
"#.to_string()
    }
}

impl Default for VideoGrabber {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse M3U8 playlist content
pub fn parse_m3u8(content: &str) -> Vec<QualityLevel> {
    let mut qualities = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for i in 0..lines.len() {
        let line = lines[i];
        
        // Look for #EXT-X-STREAM-INF
        if line.starts_with("#EXT-X-STREAM-INF:") {
            let info = line.trim_start_matches("#EXT-X-STREAM-INF:");
            
            // Extract resolution
            let resolution = if let Some(pos) = info.find("RESOLUTION=") {
                let start = pos + 11;
                let end = info[start..].find(',').map(|p| start + p).unwrap_or(info.len());
                info[start..end].to_string()
            } else {
                "Unknown".to_string()
            };
            
            // Extract bandwidth
            let bitrate = if let Some(pos) = info.find("BANDWIDTH=") {
                let start = pos + 10;
                let end = info[start..].find(',').map(|p| start + p).unwrap_or(info.len());
                info[start..end].parse().ok()
            } else {
                None
            };
            
            // Next line is the URL
            if i + 1 < lines.len() {
                let url = lines[i + 1].trim();
                if !url.starts_with('#') {
                    qualities.push(QualityLevel {
                        resolution: format_resolution(&resolution),
                        bitrate,
                        url: url.to_string(),
                        media_type: MediaType::HLS,
                    });
                }
            }
        }
    }
    
    qualities
}

/// Format resolution string (e.g., "1920x1080" -> "1080p")
fn format_resolution(res: &str) -> String {
    if let Some(pos) = res.find('x') {
        let height = &res[pos + 1..];
        match height.parse::<u32>() {
            Ok(h) if h >= 2160 => "4K".to_string(),
            Ok(h) if h >= 1440 => "1440p".to_string(),
            Ok(h) if h >= 1080 => "1080p".to_string(),
            Ok(h) if h >= 720 => "720p".to_string(),
            Ok(h) if h >= 480 => "480p".to_string(),
            Ok(h) if h >= 360 => "360p".to_string(),
            Ok(h) => format!("{}p", h),
            Err(_) => res.to_string(),
        }
    } else {
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_resolution() {
        assert_eq!(format_resolution("1920x1080"), "1080p");
        assert_eq!(format_resolution("1280x720"), "720p");
        assert_eq!(format_resolution("3840x2160"), "4K");
    }
}
