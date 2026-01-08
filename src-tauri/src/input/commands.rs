// Input Commands
// Tauri commands for virtual keyboard

use serde::{Deserialize, Serialize};
use crate::input::{VirtualKeyboard, KeyboardLayout, KeyResult};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Global virtual keyboard instance
static KEYBOARD: Lazy<Mutex<VirtualKeyboard>> = Lazy::new(|| {
    Mutex::new(VirtualKeyboard::new())
});

/// Get virtual keyboard layout
#[tauri::command]
pub fn get_virtual_keyboard_layout(shuffled: bool) -> KeyboardLayout {
    let kb = KEYBOARD.lock().unwrap();
    if shuffled {
        kb.get_shuffled_layout()
    } else {
        kb.get_standard_layout()
    }
}

/// Process a virtual key press
#[tauri::command]
pub fn process_virtual_key(key: String, is_shift: bool, is_caps: bool) -> KeyResult {
    let mut kb = KEYBOARD.lock().unwrap();
    kb.process_key(&key, is_shift, is_caps)
}
