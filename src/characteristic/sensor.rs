pub mod def;
pub mod magnetic;
pub mod motion;
pub mod posture_angle;

use def::CommandId;
use magnetic::MagneticSensorData;
use motion::MotionDetectionData;
use posture_angle::{PostureAngleEulerData, PostureAngleQuaternionsData, PostureDataType};
use crate::payload::ToPayload;

/// Sensor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SensorResponse {
    MotionDetection(MotionDetectionData),
    PostureAngleEuler(PostureAngleEulerData),
    PostureAngleQuaternion(PostureAngleQuaternionsData),
    MagneticSensor(MagneticSensorData),
}

impl SensorResponse {
    pub fn new(byte_data: &[u8]) -> Option<SensorResponse> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = MotionDetectionData::new(byte_data) {
            return Some(SensorResponse::MotionDetection(response_data));
        }
        if let Some(response_data) = MagneticSensorData::new(byte_data) {
            return Some(SensorResponse::MagneticSensor(response_data));
        }
        if let Some(response_data) = PostureAngleEulerData::new(byte_data) {
            return Some(SensorResponse::PostureAngleEuler(response_data));
        }
        if let Some(response_data) = PostureAngleQuaternionsData::new(byte_data) {
            return Some(SensorResponse::PostureAngleQuaternion(response_data));
        }
        None
    }
}

impl From<SensorResponse> for u8 {
    fn from(response_type: SensorResponse) -> u8 {
        match response_type {
            SensorResponse::MotionDetection(_) => CommandId::Motion.response(),
            SensorResponse::PostureAngleEuler(_) => CommandId::PostureAngle.response(),
            SensorResponse::PostureAngleQuaternion(_) => CommandId::PostureAngle.response(),
            SensorResponse::MagneticSensor(_) => CommandId::MagneticSensor.response(),
        }
    }
}

impl ToPayload<Vec<u8>> for SensorResponse {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            SensorResponse::MotionDetection(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            SensorResponse::PostureAngleEuler(st) => {
                payload.push(PostureDataType::Euler.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            SensorResponse::PostureAngleQuaternion(st) => {
                payload.push(PostureDataType::Quaternions.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            SensorResponse::MagneticSensor(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
        }

        payload
    }
}

#[cfg(test)]
mod test {
    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor() {
        _setup();
    }
}
