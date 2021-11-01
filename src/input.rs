use macroquad::prelude::*;

pub fn check_keyboard_input() {
    if let "macos" = std::env::consts::OS {
        if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
            return; // -> quit on macOS
        }
    }
}
