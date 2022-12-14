//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_button>

use std::time;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Unknown(u8),
    Pressed,
    Released,
}

impl From<ButtonState> for u8 {
    fn from(state: ButtonState) -> Self {
        match state {
            ButtonState::Pressed => 0x80u8,
            ButtonState::Released => 0x00u8,
            ButtonState::Unknown(x) => x,
        }
    }
}

impl From<u8> for ButtonState {
    fn from(code: u8) -> ButtonState {
        match code {
            0x80u8 => ButtonState::Pressed,
            0x00u8 => ButtonState::Released,
            x => ButtonState::Unknown(x),
        }
    }
}


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
            0x01u8 => Some(ButtonInformation { time: time::Instant::now(), state: ButtonState::from(byte_data[1]) }),
            _ => None,
        }
    }
}


