use super::super::def::common_def::CommandId;
use crate::characteristic::sensor::def::posture_angle_def::PostureDataType;
use crate::payload::ToPayload;
use serde::Serialize;

/// Request posture angle information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#requesting-posture-angle-detection>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
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
        bincode::serialize(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::characteristic::sensor::{
        PostureAngleEulerInformation, PostureAngleQuaternionsInformation,
    };
    use crate::payload::ToPayload;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_posture1() {
        _setup();

        let m = PostureAngleEulerInformation::default();
        println!("{:?}", m);
        let payload = m.to_payload();
        println!("{:?}", payload);
    }

    #[test]
    fn sensor_posture2() {
        _setup();

        let m = PostureAngleQuaternionsInformation::default();
        println!("{:?}", m);
        let payload = m.to_payload();
        println!("{:?}", payload);
    }
}
