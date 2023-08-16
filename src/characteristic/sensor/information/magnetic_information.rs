use super::super::def::command_id_def::CommandId;
use crate::payload::FromPayload;
use std::i8;
use std::u8;

/// Posture angle information (euler)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-euler-angles>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MagneticSensorInformation {
    pub state: u8,
    pub strength: u8,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl FromPayload<&[u8]> for MagneticSensorInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 6 {
            return None;
        }
        if payload[0] == CommandId::MagneticSensor.response() {
            Some(Self {
                state: payload[1],
                strength: payload[2],
                x: i8::from_le_bytes([payload[3]]),
                y: i8::from_le_bytes([payload[4]]),
                z: i8::from_le_bytes([payload[5]]),
            })
        } else {
            None
        }
    }
}
