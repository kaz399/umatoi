use super::super::def::command_id::CommandId;
use crate::payload::FromPayload;

/// Response to motor speed information\
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-motor-speed-information-acquisition-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseEnableMotorSpeedData {
    pub result: bool,
}

impl FromPayload<&[u8]> for ResponseEnableMotorSpeedData {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 3 {
            return None;
        }
        if payload[0] == CommandId::MotorSpeed.response() {
            Some(Self {
                result: payload[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}
