//! Advanced Fingerprint Protection Module
//! Provides comprehensive protection against browser fingerprinting techniques

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Advanced fingerprint protection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFingerprintProtection {
    // Screen spoofing
    pub spoof_screen: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub color_depth: u8,
    pub pixel_ratio: f64,
    
    // Hardware spoofing
    pub spoof_hardware: bool,
    pub fake_cores: u8,
    pub fake_memory: u8, // GB
    pub fake_max_touch_points: u8,
    
    // API blocking
    pub block_battery_api: bool,
    pub block_bluetooth_api: bool,
    pub block_usb_api: bool,
    pub block_sensor_api: bool,
    pub block_gamepad_api: bool,
    pub block_speech_api: bool,
    pub block_media_devices: bool,
    
    // Timing protection
    pub reduce_timer_precision: bool,
    pub timer_precision_ms: f64, // Reduce to 100ms for privacy
}

impl Default for AdvancedFingerprintProtection {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedFingerprintProtection {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        
        // Common screen resolutions for spoofing
        let screens = [
            (1920, 1080), (1366, 768), (1536, 864), (1440, 900),
            (1280, 720), (1600, 900), (2560, 1440), (1280, 800),
        ];
        let (width, height) = screens[rng.gen_range(0..screens.len())];
        
        Self {
            spoof_screen: true,
            screen_width: width,
            screen_height: height,
            color_depth: 24,
            pixel_ratio: 1.0,
            
            spoof_hardware: true,
            fake_cores: [4, 8, 12, 16][rng.gen_range(0..4)],
            fake_memory: [4, 8, 16, 32][rng.gen_range(0..4)],
            fake_max_touch_points: 0, // Desktop appearance
            
            block_battery_api: true,
            block_bluetooth_api: true,
            block_usb_api: true,
            block_sensor_api: true,
            block_gamepad_api: true,
            block_speech_api: true,
            block_media_devices: true,
            
            reduce_timer_precision: true,
            timer_precision_ms: 100.0,
        }
    }
    
    /// Generate comprehensive fingerprint protection JavaScript
    pub fn get_injection_script(&self) -> String {
        format!(r#"
(function() {{
    'use strict';
    
    // ============================================
    // ADVANCED FINGERPRINT PROTECTION - ServionX
    // ============================================
    
    const config = {{
        screen: {{ width: {screen_w}, height: {screen_h}, colorDepth: {color_depth}, pixelRatio: {pixel_ratio} }},
        hardware: {{ cores: {cores}, memory: {memory}, touchPoints: {touch_points} }},
        blockBattery: {block_battery},
        blockBluetooth: {block_bluetooth},
        blockUsb: {block_usb},
        blockSensors: {block_sensors},
        blockGamepad: {block_gamepad},
        blockSpeech: {block_speech},
        blockMediaDevices: {block_media},
        reduceTiming: {reduce_timing},
        timingPrecision: {timing_precision}
    }};
    
    // === SCREEN SPOOFING ===
    Object.defineProperties(screen, {{
        width: {{ get: () => config.screen.width, configurable: true }},
        height: {{ get: () => config.screen.height, configurable: true }},
        availWidth: {{ get: () => config.screen.width, configurable: true }},
        availHeight: {{ get: () => config.screen.height - 40, configurable: true }},
        colorDepth: {{ get: () => config.screen.colorDepth, configurable: true }},
        pixelDepth: {{ get: () => config.screen.colorDepth, configurable: true }}
    }});
    
    Object.defineProperty(window, 'devicePixelRatio', {{
        get: () => config.screen.pixelRatio,
        configurable: true
    }});
    
    Object.defineProperty(window, 'innerWidth', {{
        get: () => config.screen.width,
        configurable: true
    }});
    
    Object.defineProperty(window, 'innerHeight', {{
        get: () => config.screen.height - 100,
        configurable: true
    }});
    
    // === HARDWARE SPOOFING ===
    Object.defineProperty(navigator, 'hardwareConcurrency', {{
        get: () => config.hardware.cores,
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'deviceMemory', {{
        get: () => config.hardware.memory,
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'maxTouchPoints', {{
        get: () => config.hardware.touchPoints,
        configurable: true
    }});
    
    // === BATTERY API BLOCKING ===
    if (config.blockBattery) {{
        delete navigator.getBattery;
        Object.defineProperty(navigator, 'getBattery', {{
            value: undefined,
            writable: false,
            configurable: false
        }});
        console.log('[ServionX] Battery API blocked');
    }}
    
    // === BLUETOOTH API BLOCKING ===
    if (config.blockBluetooth && navigator.bluetooth) {{
        Object.defineProperty(navigator, 'bluetooth', {{
            value: undefined,
            writable: false,
            configurable: false
        }});
        console.log('[ServionX] Bluetooth API blocked');
    }}
    
    // === USB API BLOCKING ===
    if (config.blockUsb && navigator.usb) {{
        Object.defineProperty(navigator, 'usb', {{
            value: undefined,
            writable: false,
            configurable: false
        }});
        console.log('[ServionX] USB API blocked');
    }}
    
    // === SENSOR APIs BLOCKING ===
    if (config.blockSensors) {{
        // Block accelerometer, gyroscope, magnetometer, etc.
        const sensorAPIs = [
            'Accelerometer', 'Gyroscope', 'Magnetometer', 
            'AbsoluteOrientationSensor', 'RelativeOrientationSensor',
            'LinearAccelerationSensor', 'GravitySensor', 'AmbientLightSensor'
        ];
        sensorAPIs.forEach(api => {{
            if (window[api]) {{
                window[api] = undefined;
            }}
        }});
        console.log('[ServionX] Sensor APIs blocked');
    }}
    
    // === GAMEPAD API SPOOFING ===
    if (config.blockGamepad) {{
        navigator.getGamepads = () => [];
        window.addEventListener('gamepadconnected', e => e.stopPropagation(), true);
        console.log('[ServionX] Gamepad API blocked');
    }}
    
    // === SPEECH SYNTHESIS BLOCKING ===
    if (config.blockSpeech && window.speechSynthesis) {{
        window.speechSynthesis.getVoices = () => [];
        console.log('[ServionX] Speech synthesis voices blocked');
    }}
    
    // === MEDIA DEVICES BLOCKING ===
    if (config.blockMediaDevices && navigator.mediaDevices) {{
        const originalEnumerateDevices = navigator.mediaDevices.enumerateDevices;
        navigator.mediaDevices.enumerateDevices = async () => {{
            console.log('[ServionX] Media devices enumeration blocked');
            return [];
        }};
    }}
    
    // === TIMING ATTACK PROTECTION ===
    if (config.reduceTiming) {{
        const originalNow = performance.now.bind(performance);
        performance.now = () => {{
            return Math.floor(originalNow() / config.timingPrecision) * config.timingPrecision;
        }};
        
        const originalDateNow = Date.now;
        Date.now = () => {{
            return Math.floor(originalDateNow() / config.timingPrecision) * config.timingPrecision;
        }};
        console.log('[ServionX] Timer precision reduced to ' + config.timingPrecision + 'ms');
    }}
    
    // === KEYBOARD/MOUSE FINGERPRINT PROTECTION ===
    // Normalize keyboard event timing
    let lastKeyTime = 0;
    document.addEventListener('keydown', (e) => {{
        const now = Date.now();
        if (now - lastKeyTime < 50) {{
            e.stopPropagation();
        }}
        lastKeyTime = now;
    }}, true);
    
    // === CLIENT RECTS PROTECTION ===
    const originalGetBoundingClientRect = Element.prototype.getBoundingClientRect;
    Element.prototype.getBoundingClientRect = function() {{
        const rect = originalGetBoundingClientRect.call(this);
        // Add small random noise to prevent fingerprinting
        const noise = 0.0001;
        return {{
            x: rect.x + (Math.random() * noise),
            y: rect.y + (Math.random() * noise),
            width: rect.width,
            height: rect.height,
            top: rect.top + (Math.random() * noise),
            right: rect.right + (Math.random() * noise),
            bottom: rect.bottom + (Math.random() * noise),
            left: rect.left + (Math.random() * noise)
        }};
    }};
    
    // === PLUGIN SPOOFING ===
    Object.defineProperty(navigator, 'plugins', {{
        get: () => {{
            return {{
                length: 5,
                item: () => null,
                namedItem: () => null,
                refresh: () => {{}},
                [Symbol.iterator]: function* () {{}}
            }};
        }},
        configurable: true
    }});
    
    // === MIME TYPES SPOOFING ===
    Object.defineProperty(navigator, 'mimeTypes', {{
        get: () => {{
            return {{
                length: 4,
                item: () => null,
                namedItem: () => null,
                [Symbol.iterator]: function* () {{}}
            }};
        }},
        configurable: true
    }});
    
    console.log('[ServionX] Advanced fingerprint protection active');
    console.log('[ServionX] Screen: ' + config.screen.width + 'x' + config.screen.height);
    console.log('[ServionX] CPU cores: ' + config.hardware.cores + ', Memory: ' + config.hardware.memory + 'GB');
}})();
"#,
            screen_w = self.screen_width,
            screen_h = self.screen_height,
            color_depth = self.color_depth,
            pixel_ratio = self.pixel_ratio,
            cores = self.fake_cores,
            memory = self.fake_memory,
            touch_points = self.fake_max_touch_points,
            block_battery = self.block_battery_api,
            block_bluetooth = self.block_bluetooth_api,
            block_usb = self.block_usb_api,
            block_sensors = self.block_sensor_api,
            block_gamepad = self.block_gamepad_api,
            block_speech = self.block_speech_api,
            block_media = self.block_media_devices,
            reduce_timing = self.reduce_timer_precision,
            timing_precision = self.timer_precision_ms,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_protection_defaults() {
        let protection = AdvancedFingerprintProtection::new();
        assert!(protection.block_battery_api);
        assert!(protection.block_bluetooth_api);
        assert!(protection.block_sensor_api);
        assert!(protection.reduce_timer_precision);
    }
    
    #[test]
    fn test_injection_script_generation() {
        let protection = AdvancedFingerprintProtection::new();
        let script = protection.get_injection_script();
        assert!(script.contains("Battery API blocked"));
        assert!(script.contains("Bluetooth API blocked"));
        assert!(script.contains("Sensor APIs blocked"));
    }
}
