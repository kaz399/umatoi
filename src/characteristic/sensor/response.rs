use super::def::CommandId;
use super::magnetic::MagneticSensorData;
use super::motion::MotionDetectionData;
use super::posture_angle::{PostureAngleEulerData, PostureAngleQuaternionsData, PostureDataType};
use crate::payload::ToPayload;

/// Sensor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Response {
    MotionDetection(MotionDetectionData),
    PostureAngleEuler(PostureAngleEulerData),
    PostureAngleQuaternion(PostureAngleQuaternionsData),
    MagneticSensor(MagneticSensorData),
}

impl Response {
    pub fn new(byte_data: &[u8]) -> Option<Response> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = MotionDetectionData::new(byte_data) {
            return Some(Response::MotionDetection(response_data));
        }
        if let Some(response_data) = MagneticSensorData::new(byte_data) {
            return Some(Response::MagneticSensor(response_data));
        }
        if let Some(response_data) = PostureAngleEulerData::new(byte_data) {
            return Some(Response::PostureAngleEuler(response_data));
        }
        if let Some(response_data) = PostureAngleQuaternionsData::new(byte_data) {
            return Some(Response::PostureAngleQuaternion(response_data));
        }
        None
    }
}

impl From<Response> for u8 {
    fn from(response_type: Response) -> u8 {
        match response_type {
            Response::MotionDetection(_) => CommandId::Motion.response(),
            Response::PostureAngleEuler(_) => CommandId::PostureAngle.response(),
            Response::PostureAngleQuaternion(_) => CommandId::PostureAngle.response(),
            Response::MagneticSensor(_) => CommandId::MagneticSensor.response(),
        }
    }
}

impl ToPayload<Vec<u8>> for Response {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            Response::MotionDetection(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::PostureAngleEuler(st) => {
                payload.push(PostureDataType::Euler.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::PostureAngleQuaternion(st) => {
                payload.push(PostureDataType::Quaternions.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MagneticSensor(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
        }

        payload
    }
}
