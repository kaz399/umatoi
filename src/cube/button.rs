//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_button>

use std::time;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Button {
    Unknown,
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone)]
pub struct ButtonInfo {
    pub time: time::Instant,
    pub button: Button,
}

impl Default for ButtonInfo {
    fn default() -> Self {
        Self {
            time: time::Instant::now(),
            button: Button::Unknown,
        }
    }
}
