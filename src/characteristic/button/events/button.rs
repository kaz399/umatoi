//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_button>

use crate::payload::FromPayload;
use super::super::def::button_state::ButtonState;
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

impl FromPayload<&[u8]> for ButtonInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 2 {
            return None;
        }
        match payload[0] {
            0x01u8 => Some(ButtonInformation {
                time: time::Instant::now(),
                state: ButtonState::from(payload[1]),
            }),
            _ => None,
        }
    }
}
