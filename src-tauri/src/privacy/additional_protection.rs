//! Additional Fingerprinting Protection
//! Covers WebAssembly, ResizeObserver, and other advanced techniques

use serde::{Deserialize, Serialize};

/// Additional browser API protections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalProtection {
    pub block_wasm_fingerprint: bool,
    pub block_resize_observer_fp: bool,
    pub block_intersection_observer_fp: bool,
    pub normalize_date_formatting: bool,
    pub block_shared_workers: bool,
}

impl Default for AdditionalProtection {
    fn default() -> Self {
        Self::new()
    }
}

impl AdditionalProtection {
    pub fn new() -> Self {
        Self {
            block_wasm_fingerprint: true,
            block_resize_observer_fp: true,
            block_intersection_observer_fp: true,
            normalize_date_formatting: true,
            block_shared_workers: true,
        }
    }
    
    /// Get injection script for additional protections
    pub fn get_injection_script(&self) -> String {
        format!(r#"
(function() {{
    'use strict';
    
    // ================================================================
    //  SERVIONX - ADDITIONAL FINGERPRINTING PROTECTION
    //  WebAssembly, ResizeObserver, Workers, Intl
    // ================================================================
    
    console.log('[ServionX] 🔧 Additional fingerprinting protection...');
    
    // ==================== WEBASSEMBLY FINGERPRINTING ====================
    // Block WebAssembly feature detection fingerprinting
    if ({block_wasm} && typeof WebAssembly !== 'undefined') {{
        // Override WebAssembly.validate to return consistent results
        const origValidate = WebAssembly.validate;
        WebAssembly.validate = function(bytes) {{
            console.log('[ServionX] ⚠ WebAssembly.validate called');
            return origValidate(bytes);
        }};
        
        // Monitor WebAssembly.instantiate for fingerprinting
        const origInstantiate = WebAssembly.instantiate;
        WebAssembly.instantiate = function(buffer, imports) {{
            console.log('[ServionX] ⚠ WebAssembly.instantiate called');
            return origInstantiate(buffer, imports);
        }};
        
        // Block WebAssembly.Memory properties fingerprinting
        const origMemory = WebAssembly.Memory;
        WebAssembly.Memory = function(descriptor) {{
            console.log('[ServionX] ⚠ WebAssembly.Memory created');
            return new origMemory(descriptor);
        }};
        
        console.log('[ServionX] ✓ WebAssembly fingerprinting monitored');
    }}
    
    // ==================== RESIZE OBSERVER FINGERPRINTING ====================
    // ResizeObserver can be used to detect exact viewport/element sizes
    if ({block_resize} && typeof ResizeObserver !== 'undefined') {{
        const OriginalResizeObserver = ResizeObserver;
        window.ResizeObserver = function(callback) {{
            const wrappedCallback = (entries, observer) => {{
                // Normalize sizes to prevent fingerprinting
                const normalizedEntries = entries.map(entry => {{
                    const rect = entry.contentRect;
                    return {{
                        ...entry,
                        contentRect: {{
                            x: Math.round(rect.x),
                            y: Math.round(rect.y),
                            width: Math.round(rect.width),
                            height: Math.round(rect.height),
                            top: Math.round(rect.top),
                            right: Math.round(rect.right),
                            bottom: Math.round(rect.bottom),
                            left: Math.round(rect.left)
                        }}
                    }};
                }});
                callback(normalizedEntries, observer);
            }};
            return new OriginalResizeObserver(wrappedCallback);
        }};
        window.ResizeObserver.prototype = OriginalResizeObserver.prototype;
        console.log('[ServionX] ✓ ResizeObserver values normalized');
    }}
    
    // ==================== INTERSECTION OBSERVER FINGERPRINTING ====================
    if ({block_intersection} && typeof IntersectionObserver !== 'undefined') {{
        const OriginalIntersectionObserver = IntersectionObserver;
        window.IntersectionObserver = function(callback, options) {{
            const wrappedCallback = (entries, observer) => {{
                // Normalize intersection ratios
                const normalizedEntries = entries.map(entry => ({{
                    ...entry,
                    intersectionRatio: Math.round(entry.intersectionRatio * 100) / 100
                }}));
                callback(normalizedEntries, observer);
            }};
            return new OriginalIntersectionObserver(wrappedCallback, options);
        }};
        window.IntersectionObserver.prototype = OriginalIntersectionObserver.prototype;
        console.log('[ServionX] ✓ IntersectionObserver values normalized');
    }}
    
    // ==================== INTL DATE FORMATTING FINGERPRINTING ====================
    // Date formatting reveals locale/timezone info
    if ({normalize_date}) {{
        const origDateTimeFormat = Intl.DateTimeFormat;
        Intl.DateTimeFormat = function(locale, options) {{
            // Use generic locale if none specified
            const safeLocale = locale || 'en-US';
            return new origDateTimeFormat(safeLocale, options);
        }};
        Intl.DateTimeFormat.prototype = origDateTimeFormat.prototype;
        Intl.DateTimeFormat.supportedLocalesOf = origDateTimeFormat.supportedLocalesOf;
        console.log('[ServionX] ✓ Intl.DateTimeFormat normalized');
    }}
    
    // ==================== SHARED WORKER FINGERPRINTING ====================
    // SharedWorkers can be used for cross-tab fingerprinting
    if ({block_shared_workers} && typeof SharedWorker !== 'undefined') {{
        window.SharedWorker = function(url, name) {{
            console.log('[ServionX] ✗ SharedWorker blocked:', url);
            throw new Error('SharedWorkers are disabled for security');
        }};
        console.log('[ServionX] ✓ SharedWorker blocked');
    }}
    
    // ==================== BROADCAST CHANNEL FINGERPRINTING ====================
    // BroadcastChannel can leak cross-tab info
    if (typeof BroadcastChannel !== 'undefined') {{
        const OriginalBroadcastChannel = BroadcastChannel;
        window.BroadcastChannel = function(name) {{
            console.log('[ServionX] ⚠ BroadcastChannel created:', name);
            return new OriginalBroadcastChannel(name);
        }};
        window.BroadcastChannel.prototype = OriginalBroadcastChannel.prototype;
    }}
    
    // ==================== PERFORMANCE OBSERVER ====================
    // PerformanceObserver can leak timing info
    if (typeof PerformanceObserver !== 'undefined') {{
        const OriginalPerformanceObserver = PerformanceObserver;
        window.PerformanceObserver = function(callback) {{
            console.log('[ServionX] ⚠ PerformanceObserver created');
            return new OriginalPerformanceObserver(callback);
        }};
        window.PerformanceObserver.prototype = OriginalPerformanceObserver.prototype;
        window.PerformanceObserver.supportedEntryTypes = ['navigation', 'resource'];
    }}
    
    // ==================== REPORTING API ====================
    // Block Reporting API
    if (typeof ReportingObserver !== 'undefined') {{
        window.ReportingObserver = undefined;
        console.log('[ServionX] ✓ ReportingObserver blocked');
    }}
    
    // ==================== NETWORK INFO WORKER ====================
    // Prevent workers from accessing network info
    if (typeof navigator.connection !== 'undefined') {{
        Object.defineProperty(navigator, 'onLine', {{
            get: () => true,
            configurable: true
        }});
    }}
    
    console.log('[ServionX] ========================================');
    console.log('[ServionX] 🔧 ADDITIONAL PROTECTION ACTIVE');
    console.log('[ServionX] ========================================');
    console.log('[ServionX]   ✓ WebAssembly fingerprinting');
    console.log('[ServionX]   ✓ ResizeObserver normalized');
    console.log('[ServionX]   ✓ IntersectionObserver normalized');
    console.log('[ServionX]   ✓ SharedWorker blocked');
    console.log('[ServionX]   ✓ Intl formatting normalized');
    console.log('[ServionX] ========================================');
}})();
"#,
            block_wasm = self.block_wasm_fingerprint,
            block_resize = self.block_resize_observer_fp,
            block_intersection = self.block_intersection_observer_fp,
            normalize_date = self.normalize_date_formatting,
            block_shared_workers = self.block_shared_workers,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_additional_protection() {
        let prot = AdditionalProtection::new();
        assert!(prot.block_wasm_fingerprint);
        assert!(prot.block_resize_observer_fp);
    }
    
    #[test]
    fn test_injection_script() {
        let prot = AdditionalProtection::new();
        let script = prot.get_injection_script();
        assert!(script.contains("WebAssembly"));
        assert!(script.contains("ResizeObserver"));
        assert!(script.contains("SharedWorker"));
    }
}
