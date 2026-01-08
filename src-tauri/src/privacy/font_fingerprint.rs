// Font Fingerprint Protection
// Spoofs the list of available fonts to prevent fingerprinting

use rand::seq::SliceRandom;
use rand::Rng;

/// Common fonts that will be reported as available
const COMMON_FONTS: &[&str] = &[
    "Arial",
    "Arial Black",
    "Comic Sans MS",
    "Courier New",
    "Georgia",
    "Impact",
    "Times New Roman",
    "Trebuchet MS",
    "Verdana",
    "Helvetica",
    "Helvetica Neue",
    "Palatino",
    "Garamond",
    "Bookman",
    "Tahoma",
    "Lucida Console",
    "Monaco",
    "Consolas",
    "Liberation Sans",
    "Liberation Serif",
];

/// Generates a fake font list for fingerprint protection
pub struct FontFingerprint {
    fake_fonts: Vec<String>,
}

impl FontFingerprint {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        
        // Select random subset of common fonts (10-15 fonts)
        let num_fonts = rng.gen_range(10..=15);
        let mut fonts: Vec<String> = COMMON_FONTS.iter()
            .map(|s| s.to_string())
            .collect();
        fonts.shuffle(&mut rng);
        fonts.truncate(num_fonts);
        
        Self {
            fake_fonts: fonts,
        }
    }
    
    /// Get the list of fake fonts to report
    pub fn get_fonts(&self) -> &[String] {
        &self.fake_fonts
    }
    
    /// Generate JavaScript to inject for font spoofing
    pub fn get_injection_script(&self) -> String {
        let fonts_json = serde_json::to_string(&self.fake_fonts).unwrap_or_default();
        
        format!(r#"
// Font Fingerprint Protection
(function() {{
    'use strict';
    
    const FAKE_FONTS = {fonts};
    
    // Override document.fonts if available
    if (document.fonts && typeof document.fonts.check === 'function') {{
        const originalCheck = document.fonts.check.bind(document.fonts);
        document.fonts.check = function(font, text) {{
            // Extract font family name
            const match = font.match(/["']?([^"',]+)["']?/);
            if (match) {{
                const fontName = match[1].trim();
                return FAKE_FONTS.some(f => f.toLowerCase() === fontName.toLowerCase());
            }}
            return originalCheck(font, text);
        }};
    }}
    
    console.log('%c[ServionX] Font fingerprinting protection active', 'color: #22c55e;');
}})();
"#, fonts = fonts_json)
    }
}

impl Default for FontFingerprint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_font_generation() {
        let fp = FontFingerprint::new();
        assert!(fp.get_fonts().len() >= 10);
        assert!(fp.get_fonts().len() <= 15);
    }
}
