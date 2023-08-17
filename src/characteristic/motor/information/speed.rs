use crate::characteristic::motor::def::CommandId;
use crate::payload::FromPayload;

/// Motor Speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#obtaining-motor-speed-information>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotorSpeedInformation {
    pub left: u8,
    pub right: u8,
}

impl FromPayload<&[u8]> for MotorSpeedInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 3 {
            return None;
        }
        if payload[0] == CommandId::ObtainSpeed.response() {
            Some(Self {
                left: payload[1],
                right: payload[2],
            })
        } else {
            None
        }
    }
}
