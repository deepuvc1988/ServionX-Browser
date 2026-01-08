// Input Module
// Secure virtual keyboard and input handling

pub mod commands;

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::Rng;

/// Virtual keyboard layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardLayout {
    pub rows: Vec<Vec<KeyInfo>>,
    pub layout_id: String,
    pub is_shuffled: bool,
}

/// Information about a single key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub key: String,
    pub display: String,
    pub key_type: KeyType,
    pub width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyType {
    Character,
    Shift,
    Backspace,
    Enter,
    Space,
    Tab,
    CapsLock,
    Number,
    Symbol,
}

/// Creates and manages virtual keyboards
pub struct VirtualKeyboard {
    is_shift_active: bool,
    is_caps_active: bool,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        Self {
            is_shift_active: false,
            is_caps_active: false,
        }
    }
    
    /// Get a standard QWERTY keyboard layout
    pub fn get_standard_layout(&self) -> KeyboardLayout {
        KeyboardLayout {
            rows: vec![
                // Number row
                vec![
                    self.key("1", "1", KeyType::Number, 1.0),
                    self.key("2", "2", KeyType::Number, 1.0),
                    self.key("3", "3", KeyType::Number, 1.0),
                    self.key("4", "4", KeyType::Number, 1.0),
                    self.key("5", "5", KeyType::Number, 1.0),
                    self.key("6", "6", KeyType::Number, 1.0),
                    self.key("7", "7", KeyType::Number, 1.0),
                    self.key("8", "8", KeyType::Number, 1.0),
                    self.key("9", "9", KeyType::Number, 1.0),
                    self.key("0", "0", KeyType::Number, 1.0),
                    self.key("Backspace", "⌫", KeyType::Backspace, 1.5),
                ],
                // Top row
                vec![
                    self.key("q", "Q", KeyType::Character, 1.0),
                    self.key("w", "W", KeyType::Character, 1.0),
                    self.key("e", "E", KeyType::Character, 1.0),
                    self.key("r", "R", KeyType::Character, 1.0),
                    self.key("t", "T", KeyType::Character, 1.0),
                    self.key("y", "Y", KeyType::Character, 1.0),
                    self.key("u", "U", KeyType::Character, 1.0),
                    self.key("i", "I", KeyType::Character, 1.0),
                    self.key("o", "O", KeyType::Character, 1.0),
                    self.key("p", "P", KeyType::Character, 1.0),
                ],
                // Middle row
                vec![
                    self.key("a", "A", KeyType::Character, 1.0),
                    self.key("s", "S", KeyType::Character, 1.0),
                    self.key("d", "D", KeyType::Character, 1.0),
                    self.key("f", "F", KeyType::Character, 1.0),
                    self.key("g", "G", KeyType::Character, 1.0),
                    self.key("h", "H", KeyType::Character, 1.0),
                    self.key("j", "J", KeyType::Character, 1.0),
                    self.key("k", "K", KeyType::Character, 1.0),
                    self.key("l", "L", KeyType::Character, 1.0),
                    self.key("Enter", "↵", KeyType::Enter, 1.5),
                ],
                // Bottom row
                vec![
                    self.key("Shift", "⇧", KeyType::Shift, 1.5),
                    self.key("z", "Z", KeyType::Character, 1.0),
                    self.key("x", "X", KeyType::Character, 1.0),
                    self.key("c", "C", KeyType::Character, 1.0),
                    self.key("v", "V", KeyType::Character, 1.0),
                    self.key("b", "B", KeyType::Character, 1.0),
                    self.key("n", "N", KeyType::Character, 1.0),
                    self.key("m", "M", KeyType::Character, 1.0),
                    self.key("Shift", "⇧", KeyType::Shift, 1.5),
                ],
                // Space row
                vec![
                    self.key("@", "@", KeyType::Symbol, 1.0),
                    self.key("#", "#", KeyType::Symbol, 1.0),
                    self.key("Space", "Space", KeyType::Space, 5.0),
                    self.key(".", ".", KeyType::Symbol, 1.0),
                    self.key("-", "-", KeyType::Symbol, 1.0),
                ],
            ],
            layout_id: uuid::Uuid::new_v4().to_string(),
            is_shuffled: false,
        }
    }
    
    /// Get a shuffled keyboard layout (anti-keylogger/screenshot)
    pub fn get_shuffled_layout(&self) -> KeyboardLayout {
        let mut rng = rand::thread_rng();
        let mut layout = self.get_standard_layout();
        
        // Shuffle character keys within each row
        for row in &mut layout.rows {
            let mut char_keys: Vec<_> = row.iter()
                .filter(|k| k.key_type == KeyType::Character)
                .cloned()
                .collect();
            
            char_keys.shuffle(&mut rng);
            
            let mut char_idx = 0;
            for key in row.iter_mut() {
                if key.key_type == KeyType::Character && char_idx < char_keys.len() {
                    *key = char_keys[char_idx].clone();
                    char_idx += 1;
                }
            }
        }
        
        layout.layout_id = uuid::Uuid::new_v4().to_string();
        layout.is_shuffled = true;
        layout
    }
    
    /// Process a key press and return the resulting character
    pub fn process_key(&mut self, key: &str, is_shift: bool, is_caps: bool) -> KeyResult {
        match key {
            "Shift" => {
                self.is_shift_active = !self.is_shift_active;
                KeyResult::ModifierToggled { modifier: "Shift".to_string(), active: self.is_shift_active }
            }
            "CapsLock" => {
                self.is_caps_active = !self.is_caps_active;
                KeyResult::ModifierToggled { modifier: "CapsLock".to_string(), active: self.is_caps_active }
            }
            "Backspace" => KeyResult::Backspace,
            "Enter" => KeyResult::Enter,
            "Space" => KeyResult::Character(' '),
            "Tab" => KeyResult::Tab,
            _ => {
                let c = key.chars().next().unwrap_or(' ');
                let output = if is_shift || is_caps || self.is_shift_active || self.is_caps_active {
                    c.to_uppercase().next().unwrap_or(c)
                } else {
                    c.to_lowercase().next().unwrap_or(c)
                };
                
                // Reset shift after character
                if self.is_shift_active {
                    self.is_shift_active = false;
                }
                
                KeyResult::Character(output)
            }
        }
    }
    
    fn key(&self, key: &str, display: &str, key_type: KeyType, width: f32) -> KeyInfo {
        KeyInfo {
            key: key.to_string(),
            display: display.to_string(),
            key_type,
            width,
        }
    }
}

/// Result of processing a key press
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KeyResult {
    Character(char),
    Backspace,
    Enter,
    Tab,
    ModifierToggled { modifier: String, active: bool },
}

impl Default for VirtualKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_standard_layout() {
        let kb = VirtualKeyboard::new();
        let layout = kb.get_standard_layout();
        
        assert_eq!(layout.rows.len(), 5);
        assert!(!layout.is_shuffled);
    }
    
    #[test]
    fn test_shuffled_layout() {
        let kb = VirtualKeyboard::new();
        let layout = kb.get_shuffled_layout();
        
        assert!(layout.is_shuffled);
    }
    
    #[test]
    fn test_key_processing() {
        let mut kb = VirtualKeyboard::new();
        
        if let KeyResult::Character(c) = kb.process_key("a", false, false) {
            assert_eq!(c, 'a');
        }
        
        if let KeyResult::Character(c) = kb.process_key("a", true, false) {
            assert_eq!(c, 'A');
        }
    }
}
