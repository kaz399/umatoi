use super::def::CommandId;
use crate::payload::ToPayload;
use serde::Serialize;
use std::i8;
use std::u8;

/// Request posture angle information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#requesting-posture-angle-detection>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestMagneticSensor {
    command: CommandId,
}

impl Default for RequestMagneticSensor {
    fn default() -> Self {
        Self {
            command: CommandId::MagneticSensor,
        }
    }
}

impl ToPayload<Vec<u8>> for RequestMagneticSensor {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Posture angle information (euler)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-euler-angles>

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MagneticSensorData {
    pub state: u8,
    pub strength: u8,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl ToPayload<Vec<u8>> for MagneticSensorData {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_magnetic1() {
        _setup();

        let m = MagneticSensorData::default();
        println!("{:?}", m);
        let payload = m.to_payload();
        println!("{:?}", payload);
    }
}
