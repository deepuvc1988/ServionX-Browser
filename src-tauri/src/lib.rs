// ServionX Browser - Core Library
// The most advanced and secure browser

pub mod privacy;
pub mod metadata;
pub mod input;
pub mod security;
pub mod tools;
pub mod browser;
pub mod settings;
pub mod downloads;

use tauri::Manager;

// Re-export commonly used types
pub use privacy::PrivacyEngine;
pub use security::ProfileManager;
pub use browser::WebViewManager;
pub use settings::SettingsManager;
pub use downloads::{SmartDownloader, VideoGrabber};

/// Initialize and run the ServionX Browser
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging - don't panic if it fails
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();
    
    log::info!("Starting ServionX Browser v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            log::info!("Setting up ServionX Browser...");
            
            // Initialize privacy engine
            let privacy_engine = privacy::PrivacyEngine::new();
            app.manage(privacy_engine);
            log::info!("Privacy engine initialized");
            
            // Initialize tracker blocker
            let tracker_blocker = privacy::TrackerBlocker::new();
            app.manage(tracker_blocker);
            log::info!("Tracker blocker initialized ({} domains blocked)", 
                privacy::tracker_blocker::TRACKER_DOMAINS.len());
            
            // Initialize HTTPS enforcer
            let https_enforcer = privacy::HttpsEnforcer::new();
            app.manage(https_enforcer);
            log::info!("HTTPS enforcer initialized");
            
            // Initialize font fingerprint protection
            let font_fingerprint = privacy::FontFingerprint::new();
            log::info!("Font fingerprint protection initialized ({} fonts spoofed)", 
                font_fingerprint.get_fonts().len());
            app.manage(font_fingerprint);
            
            // Initialize referrer control
            let referrer_control = privacy::ReferrerControl::new();
            app.manage(referrer_control);
            log::info!("Referrer control initialized (NoReferrer mode)");
            
            // Initialize malware blocker
            let malware_blocker = privacy::MalwareBlocker::new();
            app.manage(malware_blocker);
            log::info!("Malware/phishing blocker initialized");
            
            // Initialize fingerprinting detector
            let fingerprint_detector = privacy::FingerprintingDetector::new();
            app.manage(fingerprint_detector);
            log::info!("Fingerprinting detector initialized");
            
            // Initialize storage partitioner
            let storage_partitioner = privacy::StoragePartitioner::new();
            app.manage(storage_partitioner);
            log::info!("Storage partitioner initialized");
            
            // Initialize blocklist manager (smart ad blocking)
            let blocklist_manager = privacy::BlocklistManager::new();
            log::info!("Blocklist manager initialized (ad/tracker blocking)");
            app.manage(blocklist_manager);
            
            // Initialize profile manager - handle errors gracefully
            match security::ProfileManager::new() {
                Ok(profile_manager) => {
                    app.manage(profile_manager);
                    log::info!("Profile manager initialized");
                }
                Err(e) => {
                    log::error!("Failed to initialize profile manager: {}", e);
                }
            }
            
            // Initialize metadata stripper
            let metadata_stripper = metadata::MetadataStripper::new();
            app.manage(metadata_stripper);
            log::info!("Metadata stripper initialized");
            
            // Initialize settings manager
            match settings::SettingsManager::new() {
                Ok(settings_manager) => {
                    app.manage(settings_manager);
                    log::info!("Settings manager initialized");
                }
                Err(e) => {
                    log::error!("Failed to initialize settings manager: {}", e);
                }
            }
            
            // Initialize upload protection (fake file metadata)
            let upload_protection = metadata::FakeFileMetadata::new();
            app.manage(upload_protection);
            log::info!("Upload protection initialized (EXIF/document metadata spoofing)");
            
            // Initialize vulnerability scanner
            let vuln_scanner = security::VulnerabilityScanner::new();
            app.manage(vuln_scanner);
            log::info!("Website vulnerability scanner initialized");
            
            // Initialize download manager with antivirus
            let download_manager = security::DownloadManager::new();
            app.manage(download_manager);
            log::info!("Download manager with file scanning initialized");
            
            // Initialize Tor manager
            let tor_manager = security::TorManager::new();
            app.manage(tor_manager);
            log::info!("Tor network manager initialized (disabled by default)");
            
            // Initialize advanced fingerprint protection
            let advanced_fp = privacy::AdvancedFingerprintProtection::new();
            app.manage(advanced_fp);
            log::info!("Advanced fingerprint protection initialized (screen/battery/sensors)");
            
            // Initialize network security
            let network_security = security::NetworkSecurity::new();
            app.manage(network_security);
            log::info!("Network security initialized (DoH/cookie hardening)");
            
            // Initialize complete fake data (comprehensive data injection)
            let complete_fake_data = privacy::CompleteFakeData::new();
            app.manage(complete_fake_data);
            log::info!("Complete fake data injection initialized (30+ APIs protected)");
            
            // Initialize ULTIMATE privacy protection (50+ APIs)
            let ultimate_protection = privacy::UltimatePrivacyProtection::new();
            app.manage(ultimate_protection);
            log::info!("ULTIMATE privacy protection initialized (50+ APIs)");
            
            // Initialize additional protection (WebAssembly, ResizeObserver, etc.)
            let additional_protection = privacy::AdditionalProtection::new();
            app.manage(additional_protection);
            log::info!("Additional protection initialized (WebAssembly/ResizeObserver)");
            
            // Initialize webview manager
            let webview_manager = browser::WebViewManager::new();
            app.manage(webview_manager);
            log::info!("WebView manager initialized");
            
            // Initialize live security logs
            let live_logs = security::LiveSecurityLogs::new();
            app.manage(live_logs);
            log::info!("Live security logs initialized");
            
            // Initialize download manager
            let downloader = downloads::SmartDownloader::new();
            log::info!("Download manager initialized: {}", downloader.get_download_dir().display());
            app.manage(downloader);
            
            // Initialize video grabber
            let video_grabber = downloads::VideoGrabber::new();
            app.manage(video_grabber);
            log::info!("Video grabber initialized (IDM-like video detection)");
            
            log::info!("╔══════════════════════════════════════════════════╗");
            log::info!("║                                                  ║");
            log::info!("║    ServionX Browser - MAXIMUM SECURITY MODE      ║");
            log::info!("║    The Most Secure Browser in the World          ║");
            log::info!("║                                                  ║");
            log::info!("╠══════════════════════════════════════════════════╣");
            log::info!("║  ✓ Canvas/WebGL/Audio Fingerprinting Protected   ║");
            log::info!("║  ✓ WebRTC STUN/TURN IP Leak Blocked              ║");
            log::info!("║  ✓ 50+ APIs Spoofed/Blocked                      ║");
            log::info!("║  ✓ Math/CSS/Keyboard Fingerprinting Protected    ║");
            log::info!("║  ✓ Battery/Bluetooth/USB/Sensor Blocked          ║");
            log::info!("║  ✓ DNS over HTTPS (Cloudflare)                   ║");
            log::info!("║  ✓ Cookie Hardening (SameSite/Secure)            ║");
            log::info!("║  ✓ SHA256 Download Scanning                      ║");
            log::info!("║  ✓ Tor Network Ready                             ║");
            log::info!("║  ✓ Malware/Phishing Protection                   ║");
            log::info!("╚══════════════════════════════════════════════════╝");
            log::info!("ServionX Browser initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Privacy commands
            privacy::commands::get_fake_fingerprint,
            privacy::commands::get_fake_geolocation,
            privacy::commands::get_fake_user_agent,
            privacy::commands::get_fake_ip_headers,
            privacy::commands::regenerate_identity,
            privacy::commands::get_injection_scripts,
            privacy::commands::check_whitelist,
            privacy::commands::add_to_whitelist,
            privacy::commands::remove_from_whitelist,
            privacy::commands::get_whitelist,
            privacy::commands::get_complete_injection_scripts,
            
            // Security commands
            security::commands::unlock_settings,
            security::commands::lock_settings,
            security::commands::is_settings_locked,
            security::commands::set_master_password,
            security::commands::verify_master_password,
            security::commands::unlock_logs,
            security::commands::lock_logs,
            security::commands::is_logs_locked,
            security::commands::set_logs_password,
            security::commands::verify_logs_password,
            security::commands::has_profile,
            security::commands::get_encrypted_logs,
            security::commands::add_log_entry,
            security::commands::get_live_logs,
            security::commands::get_recent_live_logs,
            security::commands::get_log_stats,
            security::commands::clear_live_logs,
            
            // Settings commands
            settings::commands::get_all_settings,
            settings::commands::update_all_settings,
            settings::commands::set_setting,
            settings::commands::get_setting,
            settings::commands::toggle_setting,
            settings::commands::reset_settings,
            
            // Metadata commands
            metadata::commands::strip_file_metadata,
            metadata::commands::get_file_metadata,
            
            // Input commands
            input::commands::get_virtual_keyboard_layout,
            input::commands::process_virtual_key,
            
            // Browser commands
            browser::commands::create_browser_tab,
            browser::commands::navigate_tab,
            browser::commands::close_browser_tab,
            browser::commands::get_browser_tabs,
            browser::commands::update_browser_tab,
            
            // Download commands
            downloads::commands::start_download,
            downloads::commands::get_downloads,
            downloads::commands::get_download,
            downloads::commands::pause_download,
            downloads::commands::resume_download,
            downloads::commands::cancel_download,
            downloads::commands::clear_completed_downloads,
            downloads::commands::get_download_directory,
            downloads::commands::execute_download,
            
            // Video grabber commands
            downloads::commands::get_detected_videos,
            downloads::commands::get_all_detected_videos,
            downloads::commands::get_video_detection_script,
            downloads::commands::report_detected_media,
            downloads::commands::download_video,
            
            // Tools commands
            tools::commands::ssh_connect,
            tools::commands::ssh_disconnect,
            tools::commands::ssh_execute,
            tools::commands::sftp_list_directory,
            tools::commands::sftp_download,
            tools::commands::sftp_upload,
            tools::commands::network_ping,
            tools::commands::network_port_scan,
            tools::commands::network_dns_lookup,
            tools::commands::http_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ServionX Browser");
}
