// Fingerprint Generator
// Generates consistent fake browser fingerprints

use serde::{Deserialize, Serialize};
use rand::Rng;
use uuid::Uuid;

/// Represents a complete fake browser fingerprint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FakeFingerprint {
    pub session_id: String,
    
    // Hardware fingerprint
    pub hardware_concurrency: u32,
    pub device_memory: u32,
    
    // Canvas fingerprint noise
    pub canvas_noise_seed: u32,
    
    // WebGL fingerprint
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub webgl_version: String,
    
    // Audio fingerprint noise
    pub audio_noise_seed: u32,
    
    // Font list (subset of common fonts)
    pub installed_fonts: Vec<String>,
    
    // Plugin fingerprint
    pub plugins: Vec<FakePlugin>,
    
    // Touch support
    pub max_touch_points: u32,
    pub touch_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakePlugin {
    pub name: String,
    pub description: String,
    pub filename: String,
}

#[derive(Debug)]
pub struct FingerprintGenerator {
    webgl_configs: Vec<WebGLConfig>,
    font_sets: Vec<Vec<&'static str>>,
}

#[derive(Debug)]
struct WebGLConfig {
    vendor: &'static str,
    renderer: &'static str,
}

impl FingerprintGenerator {
    pub fn new() -> Self {
        Self {
            webgl_configs: vec![
                WebGLConfig {
                    vendor: "Google Inc. (NVIDIA)",
                    renderer: "ANGLE (NVIDIA, NVIDIA GeForce RTX 3080 Direct3D11 vs_5_0 ps_5_0, D3D11)",
                },
                WebGLConfig {
                    vendor: "Google Inc. (AMD)",
                    renderer: "ANGLE (AMD, AMD Radeon RX 6800 XT Direct3D11 vs_5_0 ps_5_0, D3D11)",
                },
                WebGLConfig {
                    vendor: "Google Inc. (Intel)",
                    renderer: "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)",
                },
                WebGLConfig {
                    vendor: "Apple Inc.",
                    renderer: "Apple M1 Pro",
                },
                WebGLConfig {
                    vendor: "Apple Inc.",
                    renderer: "Apple M2 Max",
                },
                WebGLConfig {
                    vendor: "Google Inc. (NVIDIA)",
                    renderer: "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
                },
                WebGLConfig {
                    vendor: "Mesa/X.org",
                    renderer: "llvmpipe (LLVM 15.0.6, 256 bits)",
                },
                WebGLConfig {
                    vendor: "Google Inc. (Qualcomm)",
                    renderer: "ANGLE (Qualcomm, Adreno (TM) 650, OpenGL ES 3.2)",
                },
            ],
            font_sets: vec![
                vec![
                    "Arial", "Arial Black", "Comic Sans MS", "Courier New", "Georgia",
                    "Impact", "Times New Roman", "Trebuchet MS", "Verdana", "Webdings",
                ],
                vec![
                    "Arial", "Calibri", "Cambria", "Consolas", "Courier New",
                    "Segoe UI", "Tahoma", "Times New Roman", "Verdana", "Wingdings",
                ],
                vec![
                    "Arial", "Helvetica", "Menlo", "Monaco", "San Francisco",
                    "SF Pro Display", "Times", "Courier", "Georgia", "Palatino",
                ],
                vec![
                    "Arial", "DejaVu Sans", "DejaVu Serif", "Liberation Sans",
                    "Liberation Serif", "Noto Sans", "Roboto", "Ubuntu", "Droid Sans",
                ],
            ],
        }
    }
    
    /// Generate a new fake fingerprint
    pub fn generate(&self) -> FakeFingerprint {
        let mut rng = rand::thread_rng();
        
        // Select random WebGL config
        let webgl_config = &self.webgl_configs[rng.gen_range(0..self.webgl_configs.len())];
        
        // Select random font set
        let font_set = &self.font_sets[rng.gen_range(0..self.font_sets.len())];
        
        // Generate hardware specs
        let hardware_concurrency = *[2, 4, 6, 8, 12, 16].choose(&mut rng).unwrap();
        let device_memory = *[2, 4, 8, 16, 32].choose(&mut rng).unwrap();
        
        // Generate touch support (mobile vs desktop)
        let is_mobile = rng.gen_bool(0.2); // 20% chance of mobile
        
        FakeFingerprint {
            session_id: Uuid::new_v4().to_string(),
            hardware_concurrency,
            device_memory,
            canvas_noise_seed: rng.gen_range(1..10000),
            webgl_vendor: webgl_config.vendor.to_string(),
            webgl_renderer: webgl_config.renderer.to_string(),
            webgl_version: "WebGL 2.0 (OpenGL ES 3.0 Chromium)".to_string(),
            audio_noise_seed: rng.gen_range(1..10000),
            installed_fonts: font_set.iter().map(|s| s.to_string()).collect(),
            plugins: if is_mobile {
                vec![]
            } else {
                self.generate_plugins(&mut rng)
            },
            max_touch_points: if is_mobile { rng.gen_range(1..11) } else { 0 },
            touch_support: is_mobile,
        }
    }
    
    fn generate_plugins(&self, rng: &mut impl Rng) -> Vec<FakePlugin> {
        let all_plugins = vec![
            FakePlugin {
                name: "PDF Viewer".to_string(),
                description: "Portable Document Format".to_string(),
                filename: "internal-pdf-viewer".to_string(),
            },
            FakePlugin {
                name: "Chrome PDF Viewer".to_string(),
                description: "Portable Document Format".to_string(),
                filename: "internal-pdf-viewer".to_string(),
            },
            FakePlugin {
                name: "Chromium PDF Viewer".to_string(),
                description: "Portable Document Format".to_string(),
                filename: "internal-pdf-viewer".to_string(),
            },
        ];
        
        // Randomly include 1-3 plugins
        let count = rng.gen_range(1..=all_plugins.len());
        all_plugins.into_iter().take(count).collect()
    }
}

impl Default for FingerprintGenerator {
    fn default() -> Self {
        Self::new()
    }
}

trait SliceRandom {
    fn choose<R: Rng>(&self, rng: &mut R) -> Option<&u32>;
}

impl SliceRandom for [u32] {
    fn choose<R: Rng>(&self, rng: &mut R) -> Option<&u32> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fingerprint_generation() {
        let gen = FingerprintGenerator::new();
        let fp1 = gen.generate();
        let fp2 = gen.generate();
        
        // Each fingerprint should have unique session ID
        assert_ne!(fp1.session_id, fp2.session_id);
        
        // Fingerprints should have valid hardware specs
        assert!(fp1.hardware_concurrency >= 2);
        assert!(fp1.device_memory >= 2);
    }
}
