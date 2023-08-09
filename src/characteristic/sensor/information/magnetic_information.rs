use super::super::def::common_def::CommandId;
use crate::payload::ToPayload;
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

impl MagneticSensorInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 6 {
            return None;
        }
        if byte_data[0] == CommandId::MagneticSensor.response() {
            Some(Self {
                state: byte_data[1],
                strength: byte_data[2],
                x: i8::from_le_bytes([byte_data[3]]),
                y: i8::from_le_bytes([byte_data[4]]),
                z: i8::from_le_bytes([byte_data[5]]),
            })
        } else {
            None
        }
    }
}

impl ToPayload<Vec<u8>> for MagneticSensorInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![self.state, self.strength];
        payload.extend(self.x.to_le_bytes().to_vec());
        payload.extend(self.y.to_le_bytes().to_vec());
        payload.extend(self.z.to_le_bytes().to_vec());
        payload
    }
}
