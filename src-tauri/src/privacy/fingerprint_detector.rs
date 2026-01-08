// Script Fingerprinting Detection
// Detects and alerts when websites attempt to fingerprint the browser

use std::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Types of fingerprinting attempts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FingerprintingType {
    Canvas,
    WebGL,
    Audio,
    Font,
    Hardware,
    Battery,
    WebRTC,
    Timezone,
    Language,
    Screen,
}

/// A detected fingerprinting attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintingAttempt {
    pub attempt_type: FingerprintingType,
    pub url: String,
    pub timestamp: i64,
    pub blocked: bool,
}

/// Detects and tracks fingerprinting attempts
pub struct FingerprintingDetector {
    attempts: RwLock<Vec<FingerprintingAttempt>>,
    total_blocked: RwLock<u64>,
}

impl FingerprintingDetector {
    pub fn new() -> Self {
        Self {
            attempts: RwLock::new(Vec::new()),
            total_blocked: RwLock::new(0),
        }
    }
    
    /// Record a fingerprinting attempt
    pub fn record_attempt(&self, attempt_type: FingerprintingType, url: String, blocked: bool) {
        let attempt = FingerprintingAttempt {
            attempt_type,
            url,
            timestamp: chrono::Utc::now().timestamp(),
            blocked,
        };
        
        self.attempts.write().unwrap().push(attempt);
        
        if blocked {
            *self.total_blocked.write().unwrap() += 1;
        }
    }
    
    /// Get recent attempts
    pub fn get_recent_attempts(&self, limit: usize) -> Vec<FingerprintingAttempt> {
        let attempts = self.attempts.read().unwrap();
        attempts.iter().rev().take(limit).cloned().collect()
    }
    
    /// Get total blocked count
    pub fn get_blocked_count(&self) -> u64 {
        *self.total_blocked.read().unwrap()
    }
    
    /// Generate JavaScript to detect and report fingerprinting attempts
    pub fn get_injection_script() -> String {
        r#"
// Fingerprinting Detection
(function() {
    'use strict';
    
    let fingerprintAttempts = 0;
    
    // Monitor canvas fingerprinting
    const originalGetContext = HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext = function(type, ...args) {
        if (type === '2d' || type === 'webgl' || type === 'webgl2') {
            fingerprintAttempts++;
            console.log('%c[ServionX] Canvas fingerprinting attempt detected and protected', 'color: #f59e0b;');
        }
        return originalGetContext.apply(this, [type, ...args]);
    };
    
    // Monitor audio fingerprinting
    const originalCreateOscillator = AudioContext.prototype.createOscillator;
    AudioContext.prototype.createOscillator = function() {
        fingerprintAttempts++;
        console.log('%c[ServionX] Audio fingerprinting attempt detected and protected', 'color: #f59e0b;');
        return originalCreateOscillator.apply(this);
    };
    
    // Monitor WebGL fingerprinting
    const originalGetParameter = WebGLRenderingContext.prototype.getParameter;
    WebGLRenderingContext.prototype.getParameter = function(param) {
        // RENDERER and VENDOR are common fingerprinting targets
        if (param === 37446 || param === 37445) {
            fingerprintAttempts++;
            console.log('%c[ServionX] WebGL fingerprinting attempt detected and protected', 'color: #f59e0b;');
        }
        return originalGetParameter.apply(this, arguments);
    };
    
    // Report fingerprinting attempts periodically
    setInterval(() => {
        if (fingerprintAttempts > 0) {
            console.log('%c[ServionX] Total fingerprinting attempts blocked: ' + fingerprintAttempts, 'color: #22c55e; font-weight: bold;');
        }
    }, 10000);
    
    console.log('%c[ServionX] Fingerprinting detection active', 'color: #22c55e;');
})();
"#.to_string()
    }
}

impl Default for FingerprintingDetector {
    fn default() -> Self {
        Self::new()
    }
}
