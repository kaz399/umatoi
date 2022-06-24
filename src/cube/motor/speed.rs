use super::def::CommandId;
use serde::Serialize;

/// Motor Speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#obtaining-motor-speed-information>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMotorSpeedInformation {
    pub left: u8,
    pub right: u8,
}

impl ResponseMotorSpeedInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 3 {
            return None;
        }
        if byte_data[0] == CommandId::ObtainSpeed.response() {
            Some( Self {
                left: byte_data[1],
                right: byte_data[2],
            })
        } else {
            None
        }
    }
}
