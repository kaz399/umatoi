use super::super::def::common_def::CommandId;
use crate::payload::ToPayload;
use serde::Serialize;
use std::u8;

/// Request posture angle information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#requesting-posture-angle-detection>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestMagneticSensor {
    pub command: CommandId,
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

#[cfg(test)]
mod test {
    use crate::characteristic::sensor::MagneticSensorInformation;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_magnetic1() {
        _setup();

        let m = MagneticSensorInformation::default();
        println!("{:?}", m);
    }
}
