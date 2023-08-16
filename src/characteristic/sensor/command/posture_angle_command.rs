use super::super::def::command_id::CommandId;
use crate::characteristic::sensor::def::posture_angle::PostureDataType;
use crate::payload::ToPayload;

/// Request posture angle information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#requesting-posture-angle-detection>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestPostureAngleDetection {
    pub command: CommandId,
    pub data_type: PostureDataType,
}

impl Default for RequestPostureAngleDetection {
    fn default() -> Self {
        Self {
            command: CommandId::PostureAngle,
            data_type: PostureDataType::Euler,
        }
    }
}

impl ToPayload<Vec<u8>> for RequestPostureAngleDetection {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.command.to_payload());
        payload.extend(self.data_type.to_payload());
        payload
    }
}

#[cfg(test)]
mod test {
    use crate::characteristic::sensor::information::SensorInformation;
    use crate::payload::FromPayload;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_posture1() {
        _setup();

        let payload: [u8; 8] = [0x03, 0x01, 0xb4, 0x00, 0x00, 0x00, 0x4e, 0xff];
        let m = SensorInformation::from_payload(&payload);

        println!("{:?}", m);
    }

    #[test]
    fn sensor_posture2() {
        _setup();

        let payload: [u8; 10] = [0x03, 0x02, 0x00, 0x00, 0x10, 0x27, 0x00, 0x00, 0x01, 0x00];
        let m = SensorInformation::from_payload(&payload);

        println!("{:?}", m);
    }
}
