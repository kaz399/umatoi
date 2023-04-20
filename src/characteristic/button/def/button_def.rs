//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_button>

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

