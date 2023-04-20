use crate::characteristic::motor::def::CommandId;
use crate::payload::ToPayload;

/// Motor Speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#obtaining-motor-speed-information>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotorSpeedInformation {
    pub left: u8,
    pub right: u8,
}

impl MotorSpeedInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 3 {
            return None;
        }
        if byte_data[0] == CommandId::ObtainSpeed.response() {
            Some(Self {
                left: byte_data[1],
                right: byte_data[2],
            })
        } else {
            None
        }
    }
}

impl ToPayload<Vec<u8>> for MotorSpeedInformation {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.left, self.right];
        payload
    }
}
