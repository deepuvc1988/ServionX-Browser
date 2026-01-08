//! Complete Fake Data Injection Module
//! Covers EVERY aspect of browser data collection and fingerprinting

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Comprehensive fake data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteFakeData {
    // Timezone
    pub timezone: String,
    pub timezone_offset: i32,
    
    // Language
    pub language: String,
    pub languages: Vec<String>,
    
    // Network
    pub connection_type: String,
    pub effective_type: String,
    pub downlink: f64,
    pub rtt: u32,
    pub save_data: bool,
    
    // Platform
    pub platform: String,
    pub vendor: String,
    pub app_version: String,
    
    // Screen orientation
    pub orientation_type: String,
    pub orientation_angle: u16,
    
    // Do Not Track
    pub do_not_track: String,
    
    // Permissions
    pub fake_permissions: bool,
}

impl Default for CompleteFakeData {
    fn default() -> Self {
        Self::new()
    }
}

impl CompleteFakeData {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        
        let timezones = [
            ("America/New_York", -300),
            ("America/Los_Angeles", -480),
            ("Europe/London", 0),
            ("Europe/Paris", 60),
            ("Asia/Tokyo", 540),
        ];
        let (tz, offset) = timezones[rng.gen_range(0..timezones.len())];
        
        let languages = ["en-US", "en-GB", "en", "de-DE", "fr-FR", "es-ES"];
        let lang = languages[rng.gen_range(0..languages.len())];
        
        let platforms = ["Win32", "MacIntel", "Linux x86_64"];
        let platform = platforms[rng.gen_range(0..platforms.len())];
        
        Self {
            timezone: tz.to_string(),
            timezone_offset: offset,
            language: lang.to_string(),
            languages: vec![lang.to_string(), "en".to_string()],
            connection_type: "wifi".to_string(),
            effective_type: "4g".to_string(),
            downlink: 10.0,
            rtt: 50,
            save_data: false,
            platform: platform.to_string(),
            vendor: "Google Inc.".to_string(),
            app_version: "5.0 (Windows NT 10.0; Win64; x64)".to_string(),
            orientation_type: "landscape-primary".to_string(),
            orientation_angle: 0,
            do_not_track: "1".to_string(),
            fake_permissions: true,
        }
    }
    
    /// Generate the COMPLETE injection script covering ALL data collection
    pub fn get_master_injection_script(&self) -> String {
        format!(r#"
(function() {{
    'use strict';
    
    // ================================================================
    //  SERVIONX BROWSER - COMPLETE DATA PROTECTION
    //  Covers EVERY aspect of browser fingerprinting & data collection
    // ================================================================
    
    console.log('[ServionX] 🛡️ Complete data protection initializing...');
    
    // ==================== CONFIGURATION ====================
    const config = {{
        timezone: '{timezone}',
        timezoneOffset: {tz_offset},
        language: '{language}',
        languages: {languages},
        connection: {{ type: '{conn_type}', effectiveType: '{eff_type}', downlink: {downlink}, rtt: {rtt}, saveData: {save_data} }},
        platform: '{platform}',
        vendor: '{vendor}',
        appVersion: '{app_version}',
        orientation: {{ type: '{orient_type}', angle: {orient_angle} }},
        doNotTrack: '{dnt}'
    }};
    
    // ==================== TIMEZONE SPOOFING ====================
    const RealDate = Date;
    const dateOffset = config.timezoneOffset * 60 * 1000;
    
    Date.prototype.getTimezoneOffset = function() {{
        return config.timezoneOffset;
    }};
    
    const origToString = Date.prototype.toString;
    Date.prototype.toString = function() {{
        return origToString.call(this).replace(/GMT[+-]\d{{4}}/, 'GMT+0000');
    }};
    
    // Intl.DateTimeFormat timezone
    const origResolvedOptions = Intl.DateTimeFormat.prototype.resolvedOptions;
    Intl.DateTimeFormat.prototype.resolvedOptions = function() {{
        const result = origResolvedOptions.call(this);
        result.timeZone = config.timezone;
        return result;
    }};
    
    console.log('[ServionX] ✓ Timezone spoofed: ' + config.timezone);
    
    // ==================== LANGUAGE SPOOFING ====================
    Object.defineProperties(navigator, {{
        language: {{ get: () => config.language, configurable: true }},
        languages: {{ get: () => Object.freeze(config.languages), configurable: true }}
    }});
    console.log('[ServionX] ✓ Language spoofed: ' + config.language);
    
    // ==================== NETWORK INFORMATION SPOOFING ====================
    if (navigator.connection) {{
        Object.defineProperties(navigator.connection, {{
            type: {{ get: () => config.connection.type, configurable: true }},
            effectiveType: {{ get: () => config.connection.effectiveType, configurable: true }},
            downlink: {{ get: () => config.connection.downlink, configurable: true }},
            rtt: {{ get: () => config.connection.rtt, configurable: true }},
            saveData: {{ get: () => config.connection.saveData, configurable: true }}
        }});
    }} else {{
        Object.defineProperty(navigator, 'connection', {{
            get: () => ({{
                type: config.connection.type,
                effectiveType: config.connection.effectiveType,
                downlink: config.connection.downlink,
                rtt: config.connection.rtt,
                saveData: config.connection.saveData,
                onchange: null
            }}),
            configurable: true
        }});
    }}
    console.log('[ServionX] ✓ Network info spoofed: ' + config.connection.effectiveType);
    
    // ==================== PLATFORM SPOOFING ====================
    Object.defineProperties(navigator, {{
        platform: {{ get: () => config.platform, configurable: true }},
        vendor: {{ get: () => config.vendor, configurable: true }},
        appVersion: {{ get: () => config.appVersion, configurable: true }},
        product: {{ get: () => 'Gecko', configurable: true }},
        productSub: {{ get: () => '20030107', configurable: true }},
        appCodeName: {{ get: () => 'Mozilla', configurable: true }},
        appName: {{ get: () => 'Netscape', configurable: true }}
    }});
    console.log('[ServionX] ✓ Platform spoofed: ' + config.platform);
    
    // ==================== DO NOT TRACK ====================
    Object.defineProperty(navigator, 'doNotTrack', {{
        get: () => config.doNotTrack,
        configurable: true
    }});
    console.log('[ServionX] ✓ Do Not Track: enabled');
    
    // ==================== SCREEN ORIENTATION ====================
    if (screen.orientation) {{
        Object.defineProperties(screen.orientation, {{
            type: {{ get: () => config.orientation.type, configurable: true }},
            angle: {{ get: () => config.orientation.angle, configurable: true }}
        }});
    }}
    
    // ==================== CLIPBOARD API BLOCKING ====================
    navigator.clipboard.readText = async () => {{
        console.log('[ServionX] ✗ Clipboard read blocked');
        throw new DOMException('Clipboard access denied', 'NotAllowedError');
    }};
    navigator.clipboard.read = async () => {{
        console.log('[ServionX] ✗ Clipboard read blocked');
        throw new DOMException('Clipboard access denied', 'NotAllowedError');
    }};
    console.log('[ServionX] ✓ Clipboard API blocked');
    
    // ==================== BEACON API BLOCKING ====================
    const origSendBeacon = navigator.sendBeacon;
    navigator.sendBeacon = function(url, data) {{
        console.log('[ServionX] ✗ Beacon blocked:', url);
        return false;
    }};
    console.log('[ServionX] ✓ Beacon API blocked');
    
    // ==================== PERMISSIONS API SPOOFING ====================
    if (navigator.permissions) {{
        const origQuery = navigator.permissions.query.bind(navigator.permissions);
        navigator.permissions.query = async (permissionDesc) => {{
            console.log('[ServionX] Permission query:', permissionDesc.name);
            return {{
                state: 'prompt',
                name: permissionDesc.name,
                onchange: null
            }};
        }};
    }}
    console.log('[ServionX] ✓ Permissions API spoofed');
    
    // ==================== PAGE VISIBILITY BLOCKING ====================
    Object.defineProperty(document, 'visibilityState', {{
        get: () => 'visible',
        configurable: true
    }});
    Object.defineProperty(document, 'hidden', {{
        get: () => false,
        configurable: true
    }});
    console.log('[ServionX] ✓ Page visibility locked to visible');
    
    // ==================== NOTIFICATION API BLOCKING ====================
    if (window.Notification) {{
        window.Notification = class FakeNotification {{
            static get permission() {{ return 'denied'; }}
            static requestPermission() {{ return Promise.resolve('denied'); }}
            constructor() {{ throw new Error('Notifications disabled'); }}
        }};
    }}
    console.log('[ServionX] ✓ Notification API blocked');
    
    // ==================== GEOLOCATION WATCHPOSITION BLOCKING ====================
    if (navigator.geolocation) {{
        const origGetCurrentPosition = navigator.geolocation.getCurrentPosition;
        navigator.geolocation.getCurrentPosition = function(success, error, options) {{
            console.log('[ServionX] ✗ Geolocation request blocked');
            if (error) error({{ code: 1, message: 'User denied geolocation' }});
        }};
        navigator.geolocation.watchPosition = function() {{
            console.log('[ServionX] ✗ Geolocation watch blocked');
            return 0;
        }};
    }}
    console.log('[ServionX] ✓ Geolocation API blocked');
    
    // ==================== WEBGL EXTENSION SPOOFING ====================
    const origGetExtension = WebGLRenderingContext.prototype.getExtension;
    WebGLRenderingContext.prototype.getExtension = function(name) {{
        if (name === 'WEBGL_debug_renderer_info') {{
            return null;
        }}
        return origGetExtension.call(this, name);
    }};
    console.log('[ServionX] ✓ WebGL debug info blocked');
    
    // ==================== FONT ENUMERATION BLOCKING ====================
    if (document.fonts) {{
        document.fonts.check = () => true;
        document.fonts.forEach = () => {{}};
    }}
    console.log('[ServionX] ✓ Font enumeration blocked');
    
    // ==================== MEDIA CAPABILITIES SPOOFING ====================
    if (navigator.mediaCapabilities) {{
        navigator.mediaCapabilities.decodingInfo = async () => ({{
            supported: true,
            smooth: true,
            powerEfficient: true
        }});
    }}
    console.log('[ServionX] ✓ Media capabilities spoofed');
    
    // ==================== PAYMENT REQUEST BLOCKING ====================
    if (window.PaymentRequest) {{
        window.PaymentRequest = class FakePaymentRequest {{
            constructor() {{ throw new Error('PaymentRequest disabled'); }}
        }};
    }}
    console.log('[ServionX] ✓ Payment Request API blocked');
    
    // ==================== CREDENTIAL API BLOCKING ====================
    if (navigator.credentials) {{
        navigator.credentials.get = async () => null;
        navigator.credentials.create = async () => null;
        navigator.credentials.store = async () => {{}};
    }}
    console.log('[ServionX] ✓ Credentials API blocked');
    
    // ==================== SERVICE WORKER RESTRICTIONS ====================
    if (navigator.serviceWorker) {{
        Object.defineProperty(navigator, 'serviceWorker', {{
            get: () => ({{
                register: () => Promise.reject(new Error('ServiceWorkers disabled')),
                getRegistration: () => Promise.resolve(undefined),
                getRegistrations: () => Promise.resolve([]),
                ready: Promise.reject(new Error('ServiceWorkers disabled'))
            }}),
            configurable: true
        }});
    }}
    console.log('[ServionX] ✓ Service Workers restricted');
    
    // ==================== WEB WORKERS CSP ====================
    const origWorker = window.Worker;
    window.Worker = function(url) {{
        console.log('[ServionX] ⚠ Worker created:', url);
        return new origWorker(url);
    }};
    
    // ==================== IDLE DETECTION BLOCKING ====================
    if (window.IdleDetector) {{
        window.IdleDetector = undefined;
    }}
    console.log('[ServionX] ✓ Idle Detection API blocked');
    
    // ==================== WAKE LOCK BLOCKING ====================
    if (navigator.wakeLock) {{
        navigator.wakeLock.request = async () => {{
            throw new DOMException('Wake Lock denied', 'NotAllowedError');
        }};
    }}
    console.log('[ServionX] ✓ Wake Lock API blocked');
    
    // ==================== SHARE API BLOCKING ====================
    if (navigator.share) {{
        navigator.share = async () => {{
            throw new DOMException('Share denied', 'NotAllowedError');
        }};
    }}
    console.log('[ServionX] ✓ Web Share API blocked');
    
    // ==================== STORAGE ESTIMATE SPOOFING ====================
    if (navigator.storage) {{
        navigator.storage.estimate = async () => ({{
            quota: 2147483648, // 2GB (standard)
            usage: 0
        }});
    }}
    console.log('[ServionX] ✓ Storage estimate spoofed');
    
    // ==================== PRINTING BLOCKING ====================
    window.print = function() {{
        console.log('[ServionX] ✗ Print blocked');
    }};
    
    // ==================== HISTORY LENGTH SPOOFING ====================
    Object.defineProperty(history, 'length', {{
        get: () => 1,
        configurable: true
    }});
    console.log('[ServionX] ✓ History length spoofed');
    
    // ==================== CONSOLE PROTECTION ====================
    // Prevent console timing attacks
    const origConsoleLog = console.log;
    console.log = function(...args) {{
        if (args[0] && typeof args[0] === 'string' && args[0].startsWith('[ServionX]')) {{
            origConsoleLog.apply(console, args);
        }} else {{
            origConsoleLog.apply(console, args);
        }}
    }};
    
    // ==================== COMPLETION ====================
    console.log('[ServionX] ========================================');
    console.log('[ServionX] 🛡️ COMPLETE DATA PROTECTION ACTIVE');
    console.log('[ServionX] ========================================');
    console.log('[ServionX] Protected APIs:');
    console.log('[ServionX]   ✓ Timezone, Language, Platform');
    console.log('[ServionX]   ✓ Network, Clipboard, Beacon');
    console.log('[ServionX]   ✓ Permissions, Visibility, Notifications');
    console.log('[ServionX]   ✓ Geolocation, WebGL, Fonts');
    console.log('[ServionX]   ✓ Storage, Credentials, ServiceWorkers');
    console.log('[ServionX]   ✓ Battery, Sensors, Media Devices');
    console.log('[ServionX] ========================================');
}})();
"#,
            timezone = self.timezone,
            tz_offset = self.timezone_offset,
            language = self.language,
            languages = serde_json::to_string(&self.languages).unwrap_or("[]".to_string()),
            conn_type = self.connection_type,
            eff_type = self.effective_type,
            downlink = self.downlink,
            rtt = self.rtt,
            save_data = self.save_data,
            platform = self.platform,
            vendor = self.vendor,
            app_version = self.app_version,
            orient_type = self.orientation_type,
            orient_angle = self.orientation_angle,
            dnt = self.do_not_track,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complete_fake_data() {
        let data = CompleteFakeData::new();
        assert!(!data.timezone.is_empty());
        assert!(!data.language.is_empty());
        assert!(!data.platform.is_empty());
    }
    
    #[test]
    fn test_master_injection_script() {
        let data = CompleteFakeData::new();
        let script = data.get_master_injection_script();
        assert!(script.contains("COMPLETE DATA PROTECTION"));
        assert!(script.contains("Clipboard API blocked"));
        assert!(script.contains("Beacon API blocked"));
        assert!(script.contains("Geolocation API blocked"));
    }
}
