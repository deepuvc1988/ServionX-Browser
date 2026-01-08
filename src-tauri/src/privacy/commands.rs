// Privacy Commands
// Tauri commands for privacy functionality

use tauri::State;
use crate::privacy::{
    PrivacyEngine, FakeIdentity,
    fingerprint::FakeFingerprint,
    geolocation::FakeGeolocation,
    user_agent::FakeUserAgent,
    ip_privacy::FakeIpHeaders,
};

/// Get the current fake fingerprint
#[tauri::command]
pub fn get_fake_fingerprint(privacy: State<PrivacyEngine>) -> FakeFingerprint {
    privacy.get_identity().fingerprint
}

/// Get the current fake geolocation
#[tauri::command]
pub fn get_fake_geolocation(privacy: State<PrivacyEngine>) -> FakeGeolocation {
    privacy.get_identity().geolocation
}

/// Get the current fake user agent
#[tauri::command]
pub fn get_fake_user_agent(privacy: State<PrivacyEngine>) -> FakeUserAgent {
    privacy.get_identity().user_agent
}

/// Get the current fake IP headers
#[tauri::command]
pub fn get_fake_ip_headers(privacy: State<PrivacyEngine>) -> FakeIpHeaders {
    privacy.get_identity().ip_headers
}

/// Regenerate a new fake identity
#[tauri::command]
pub fn regenerate_identity(privacy: State<PrivacyEngine>) -> FakeIdentity {
    privacy.regenerate_identity()
}

/// Get JavaScript injection scripts for privacy protection
#[tauri::command]
pub fn get_injection_scripts(privacy: State<PrivacyEngine>) -> String {
    privacy.get_injection_scripts()
}

/// Check if a domain is whitelisted
#[tauri::command]
pub fn check_whitelist(privacy: State<PrivacyEngine>, domain: String) -> bool {
    privacy.is_whitelisted(&domain)
}

/// Add a domain to the whitelist
#[tauri::command]
pub fn add_to_whitelist(privacy: State<PrivacyEngine>, domain: String) {
    privacy.add_to_whitelist(&domain);
}

/// Remove a domain from the whitelist
#[tauri::command]
pub fn remove_from_whitelist(privacy: State<PrivacyEngine>, domain: String) {
    privacy.remove_from_whitelist(&domain);
}

/// Get all whitelisted domains
#[tauri::command]
pub fn get_whitelist(privacy: State<PrivacyEngine>) -> Vec<String> {
    privacy.get_whitelist()
}

/// Get COMPLETE injection scripts (ALL protection modules combined)
#[tauri::command]
pub fn get_complete_injection_scripts(
    privacy: State<PrivacyEngine>,
    advanced_fp: State<crate::privacy::AdvancedFingerprintProtection>,
    network_security: State<crate::security::NetworkSecurity>,
    complete_fake: State<crate::privacy::CompleteFakeData>,
    ultimate: State<crate::privacy::UltimatePrivacyProtection>,
    upload_protection: State<crate::metadata::FakeFileMetadata>,
) -> String {
    // Combine all injection scripts
    let mut combined = String::new();
    
    // 1. Base identity spoofing (fingerprint, geolocation, user agent)
    combined.push_str(&privacy.get_injection_scripts());
    combined.push('\n');
    
    // 2. Advanced fingerprint protection (screen, battery, sensors, timing)
    combined.push_str(&advanced_fp.get_injection_script());
    combined.push('\n');
    
    // 3. Network security (cookies, mixed content, referrer)
    combined.push_str(&network_security.get_injection_script());
    combined.push('\n');
    
    // 4. Complete fake data (timezone, language, network, clipboard, etc.)
    combined.push_str(&complete_fake.get_master_injection_script());
    combined.push('\n');
    
    // 5. Ultimate protection (WebRTC, Math, CSS, Keyboard, Audio, Canvas deep protection)
    combined.push_str(&ultimate.get_ultimate_injection_script());
    combined.push('\n');
    
    // 6. Upload protection (FormData, Fetch, XHR, drag-drop, paste, FileReader)
    combined.push_str(&upload_protection.get_upload_protection_script());
    
    log::info!("Combined injection script generated ({} bytes) - 6 protection layers", combined.len());
    combined
}
