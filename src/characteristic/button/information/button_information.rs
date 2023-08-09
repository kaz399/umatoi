//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_button>

use super::super::def::button_def::ButtonState;
use std::time;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ButtonInformation {
    pub time: time::Instant,
    pub state: ButtonState,
}

impl Default for ButtonInformation {
    fn default() -> Self {
        Self {
            time: time::Instant::now(),
            state: ButtonState::Unknown(0xffu8),
        }
    }
}

impl ButtonInformation {
    pub fn new(byte_data: &[u8]) -> Option<ButtonInformation> {
        if byte_data.len() < 2 {
            return None;
        }
        match byte_data[0] {
            0x01u8 => Some(ButtonInformation {
                time: time::Instant::now(),
                state: ButtonState::from(byte_data[1]),
            }),
            _ => None,
        }
    }
}
