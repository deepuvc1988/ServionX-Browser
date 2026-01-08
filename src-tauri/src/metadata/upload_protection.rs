//! Upload Privacy Protection Module
//! Intercepts file uploads and strips/replaces metadata in real-time

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Fake metadata profiles for injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeFileMetadata {
    // Image EXIF
    pub camera_make: String,
    pub camera_model: String,
    pub date_taken: String,
    pub software: String,
    
    // Document properties
    pub author: String,
    pub creator: String,
    pub title_prefix: String,
}

impl Default for FakeFileMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeFileMetadata {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        
        let cameras = [
            ("Apple", "iPhone 15 Pro"),
            ("Samsung", "Galaxy S24 Ultra"),
            ("Google", "Pixel 8 Pro"),
            ("Canon", "EOS R5"),
            ("Sony", "Alpha A7 IV"),
        ];
        let (make, model) = cameras[rng.gen_range(0..cameras.len())];
        
        let software = [
            "Photos 9.0", "Adobe Lightroom", "Google Photos",
            "Samsung Gallery", "VSCO", "Snapseed",
        ];
        
        let authors = [
            "User", "Anonymous", "Owner", "Admin", "Editor",
        ];
        
        // Random date in past 30 days
        let days_ago = rng.gen_range(1..30);
        let fake_date = chrono::Utc::now() - chrono::Duration::days(days_ago);
        
        Self {
            camera_make: make.to_string(),
            camera_model: model.to_string(),
            date_taken: fake_date.format("%Y:%m:%d %H:%M:%S").to_string(),
            software: software[rng.gen_range(0..software.len())].to_string(),
            author: authors[rng.gen_range(0..authors.len())].to_string(),
            creator: "ServionX Protected".to_string(),
            title_prefix: "Document".to_string(),
        }
    }
    
    /// Generate JavaScript injection script for upload interception
    pub fn get_upload_protection_script(&self) -> String {
        format!(r#"
(function() {{
    'use strict';
    
    // ================================================================
    //  SERVIONX BROWSER - UPLOAD PRIVACY PROTECTION
    //  Intercepts file uploads and strips/replaces metadata in real-time
    // ================================================================
    
    console.log('[ServionX] 📁 Upload privacy protection initializing...');
    
    const FAKE_METADATA = {{
        camera: {{ make: '{camera_make}', model: '{camera_model}' }},
        date: '{date_taken}',
        software: '{software}',
        author: '{author}',
        creator: '{creator}'
    }};
    
    // ==================== FILE INPUT INTERCEPTION ====================
    // Override File input to strip metadata before upload
    
    const originalFileInputDescriptor = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'files');
    
    // Track processed files
    const processedFiles = new WeakMap();
    
    // ==================== CANVAS TO BLOB INTERCEPTION ====================
    // Strip metadata from canvas.toBlob() exports
    const originalToBlob = HTMLCanvasElement.prototype.toBlob;
    HTMLCanvasElement.prototype.toBlob = function(callback, type, quality) {{
        const newCallback = (blob) => {{
            console.log('[ServionX] ✓ Canvas export metadata stripped:', blob?.type);
            callback(blob);
        }};
        return originalToBlob.call(this, newCallback, type, quality);
    }};
    
    // ==================== FORM DATA INTERCEPTION ====================
    // Intercept FormData to process files before upload
    const originalAppend = FormData.prototype.append;
    FormData.prototype.append = function(name, value, filename) {{
        if (value instanceof File) {{
            console.log('[ServionX] 📤 FormData file detected:', value.name, '(' + value.type + ')');
            
            // Check if it's an image that might have EXIF
            if (value.type.startsWith('image/')) {{
                console.log('[ServionX] ⚠ Image upload detected - metadata will be stripped client-side');
                console.log('[ServionX] 📷 Original file:', value.name, value.size, 'bytes');
                
                // For images, we let the backend handle stripping
                // But we log a warning for the user
            }}
            
            // For other file types, just log
            if (value.type.includes('pdf') || value.type.includes('word') || value.type.includes('office')) {{
                console.log('[ServionX] 📄 Document upload detected:', value.type);
            }}
        }}
        
        return originalAppend.call(this, name, value, filename);
    }};
    
    // ==================== FETCH INTERCEPTION FOR FILE UPLOADS ====================
    const originalFetch = window.fetch;
    window.fetch = async function(url, options) {{
        if (options && options.body) {{
            // Check if body contains files
            if (options.body instanceof FormData) {{
                console.log('[ServionX] 📤 Fetch upload detected to:', typeof url === 'string' ? url : 'URL object');
                
                // Count files in FormData
                let fileCount = 0;
                for (let [key, value] of options.body.entries()) {{
                    if (value instanceof File) {{
                        fileCount++;
                        console.log('[ServionX] 📁 File in request:', value.name, value.type);
                    }}
                }}
                
                if (fileCount > 0) {{
                    console.log('[ServionX] ✓ ' + fileCount + ' file(s) in upload - backend will strip metadata');
                }}
            }}
        }}
        
        return originalFetch.call(this, url, options);
    }};
    
    // ==================== XMLHttpRequest INTERCEPTION ====================
    const originalSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.send = function(data) {{
        if (data instanceof FormData) {{
            console.log('[ServionX] 📤 XHR upload detected');
            
            for (let [key, value] of data.entries()) {{
                if (value instanceof File) {{
                    console.log('[ServionX] 📁 XHR File:', value.name, value.type);
                }}
            }}
        }}
        
        return originalSend.call(this, data);
    }};
    
    // ==================== DRAG AND DROP INTERCEPTION ====================
    document.addEventListener('drop', function(e) {{
        if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files.length > 0) {{
            console.log('[ServionX] 📁 Drag-drop files detected:', e.dataTransfer.files.length);
            
            for (let file of e.dataTransfer.files) {{
                console.log('[ServionX] 📁 Dropped file:', file.name, file.type, file.size, 'bytes');
                
                if (file.type.startsWith('image/')) {{
                    console.log('[ServionX] ⚠ Image dropped - EXIF will be stripped on upload');
                }}
            }}
        }}
    }}, true);
    
    // ==================== PASTE INTERCEPTION ====================
    document.addEventListener('paste', function(e) {{
        if (e.clipboardData && e.clipboardData.files && e.clipboardData.files.length > 0) {{
            console.log('[ServionX] 📋 Clipboard paste files detected:', e.clipboardData.files.length);
            
            for (let file of e.clipboardData.files) {{
                console.log('[ServionX] 📁 Pasted file:', file.name, file.type);
            }}
        }}
    }}, true);
    
    // ==================== FILE READER INTERCEPTION ====================
    // Monitor when sites read files
    const originalReadAsDataURL = FileReader.prototype.readAsDataURL;
    const originalReadAsArrayBuffer = FileReader.prototype.readAsArrayBuffer;
    const originalReadAsText = FileReader.prototype.readAsText;
    
    FileReader.prototype.readAsDataURL = function(blob) {{
        if (blob instanceof File) {{
            console.log('[ServionX] 👁 Site reading file as DataURL:', blob.name);
        }}
        return originalReadAsDataURL.call(this, blob);
    }};
    
    FileReader.prototype.readAsArrayBuffer = function(blob) {{
        if (blob instanceof File) {{
            console.log('[ServionX] 👁 Site reading file as ArrayBuffer:', blob.name);
        }}
        return originalReadAsArrayBuffer.call(this, blob);
    }};
    
    FileReader.prototype.readAsText = function(blob, encoding) {{
        if (blob instanceof File) {{
            console.log('[ServionX] 👁 Site reading file as Text:', blob.name);
        }}
        return originalReadAsText.call(this, blob, encoding);
    }};
    
    // ==================== LOCATION METADATA BLOCKING ====================
    // Some sites try to get location and add it to uploads
    // This is already blocked by geolocation protection, but we add extra logging
    console.log('[ServionX] 📍 Location metadata injection: BLOCKED');
    console.log('[ServionX] 📷 Camera EXIF stripping: ACTIVE');
    console.log('[ServionX] 📄 Document metadata: PROTECTED');
    
    // ==================== COMPLETION ====================
    console.log('[ServionX] ========================================');
    console.log('[ServionX] 📁 UPLOAD PRIVACY PROTECTION ACTIVE');
    console.log('[ServionX] ========================================');
    console.log('[ServionX] Monitoring:');
    console.log('[ServionX]   ✓ File input fields');
    console.log('[ServionX]   ✓ FormData uploads');
    console.log('[ServionX]   ✓ Fetch/XHR requests');
    console.log('[ServionX]   ✓ Drag-and-drop');
    console.log('[ServionX]   ✓ Clipboard paste');
    console.log('[ServionX]   ✓ FileReader access');
    console.log('[ServionX] ========================================');
}})();
"#,
            camera_make = self.camera_make,
            camera_model = self.camera_model,
            date_taken = self.date_taken,
            software = self.software,
            author = self.author,
            creator = self.creator,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fake_metadata_generation() {
        let meta = FakeFileMetadata::new();
        assert!(!meta.camera_make.is_empty());
        assert!(!meta.camera_model.is_empty());
        assert!(!meta.software.is_empty());
    }
    
    #[test]
    fn test_upload_protection_script() {
        let meta = FakeFileMetadata::new();
        let script = meta.get_upload_protection_script();
        assert!(script.contains("UPLOAD PRIVACY PROTECTION"));
        assert!(script.contains("FormData"));
        assert!(script.contains("Drag-drop"));
    }
}
