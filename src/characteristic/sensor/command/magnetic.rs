use super::super::def::command_id::CommandId;
use crate::payload::ToPayload;
use std::u8;

/// Request posture angle information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#requesting-posture-angle-detection>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        let payload: Vec<u8> = vec![self.command.into()];
        payload
    }
}

#[cfg(test)]
mod test {
    use crate::characteristic::sensor::SensorInformation;
    use crate::payload::FromPayload;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_magnetic1() {
        _setup();

        let payload: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
        let m = SensorInformation::from_payload(&payload);

        println!("{:?}", m);
    }
}
