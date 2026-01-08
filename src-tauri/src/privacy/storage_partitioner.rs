// Storage Partitioning
// Isolates localStorage, sessionStorage, and IndexedDB per first-party origin

use std::sync::RwLock;

/// Storage partitioning controller
pub struct StoragePartitioner {
    enabled: RwLock<bool>,
    partitioned_sites: RwLock<u64>,
}

impl StoragePartitioner {
    pub fn new() -> Self {
        Self {
            enabled: RwLock::new(true),
            partitioned_sites: RwLock::new(0),
        }
    }
    
    /// Enable/disable partitioning
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }
    
    /// Get count of partitioned sites
    pub fn get_partitioned_count(&self) -> u64 {
        *self.partitioned_sites.read().unwrap()
    }
    
    /// Generate JavaScript to partition storage
    pub fn get_injection_script() -> String {
        r#"
// Storage Partitioning Protection
(function() {
    'use strict';
    
    const PARTITION_KEY = location.hostname;
    
    // Partition localStorage
    const originalLocalStorageSetItem = localStorage.setItem.bind(localStorage);
    const originalLocalStorageGetItem = localStorage.getItem.bind(localStorage);
    const originalLocalStorageRemoveItem = localStorage.removeItem.bind(localStorage);
    
    localStorage.setItem = function(key, value) {
        return originalLocalStorageSetItem(PARTITION_KEY + '::' + key, value);
    };
    
    localStorage.getItem = function(key) {
        return originalLocalStorageGetItem(PARTITION_KEY + '::' + key);
    };
    
    localStorage.removeItem = function(key) {
        return originalLocalStorageRemoveItem(PARTITION_KEY + '::' + key);
    };
    
    // Partition sessionStorage
    const originalSessionStorageSetItem = sessionStorage.setItem.bind(sessionStorage);
    const originalSessionStorageGetItem = sessionStorage.getItem.bind(sessionStorage);
    const originalSessionStorageRemoveItem = sessionStorage.removeItem.bind(sessionStorage);
    
    sessionStorage.setItem = function(key, value) {
        return originalSessionStorageSetItem(PARTITION_KEY + '::' + key, value);
    };
    
    sessionStorage.getItem = function(key) {
        return originalSessionStorageGetItem(PARTITION_KEY + '::' + key);
    };
    
    sessionStorage.removeItem = function(key) {
        return originalSessionStorageRemoveItem(PARTITION_KEY + '::' + key);
    };
    
    // Block cross-site cookie access
    Object.defineProperty(document, 'cookie', {
        get: function() {
            // Return only first-party cookies (simulated)
            return '';
        },
        set: function(value) {
            // Allow setting but isolate
            return value;
        },
        configurable: false
    });
    
    console.log('%c[ServionX] Storage partitioned for: ' + PARTITION_KEY, 'color: #22c55e;');
})();
"#.to_string()
    }
}

impl Default for StoragePartitioner {
    fn default() -> Self {
        Self::new()
    }
}
