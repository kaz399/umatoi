mod magnetic_information;
mod motion_information;
mod posture_angle_information;

use crate::characteristic::sensor::def::CommandId;
use crate::characteristic::sensor::def::PostureDataType;
use crate::payload::ToPayload;

/// Sensor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SensorInformation {
    MotionDetection(motion_information::MotionDetectionInformation),
    PostureAngleEuler(posture_angle_information::PostureAngleEulerInformation),
    PostureAngleQuaternion(posture_angle_information::PostureAngleQuaternionsInformation),
    MagneticSensor(magnetic_information::MagneticSensorInformation),
}

impl SensorInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = motion_information::MotionDetectionInformation::new(byte_data)
        {
            return Some(SensorInformation::MotionDetection(response_data));
        }
        if let Some(response_data) = magnetic_information::MagneticSensorInformation::new(byte_data)
        {
            return Some(SensorInformation::MagneticSensor(response_data));
        }
        if let Some(response_data) =
            posture_angle_information::PostureAngleEulerInformation::new(byte_data)
        {
            return Some(SensorInformation::PostureAngleEuler(response_data));
        }
        if let Some(response_data) =
            posture_angle_information::PostureAngleQuaternionsInformation::new(byte_data)
        {
            return Some(SensorInformation::PostureAngleQuaternion(response_data));
        }
        None
    }
}

impl From<SensorInformation> for u8 {
    fn from(response_type: SensorInformation) -> u8 {
        match response_type {
            SensorInformation::MotionDetection(_) => CommandId::Motion.response(),
            SensorInformation::PostureAngleEuler(_) => CommandId::PostureAngle.response(),
            SensorInformation::PostureAngleQuaternion(_) => CommandId::PostureAngle.response(),
            SensorInformation::MagneticSensor(_) => CommandId::MagneticSensor.response(),
        }
    }
}

impl ToPayload<Vec<u8>> for SensorInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            SensorInformation::MotionDetection(st) => {
                payload.extend(st.to_payload());
            }
            SensorInformation::PostureAngleEuler(st) => {
                payload.push(PostureDataType::Euler.into());
                payload.extend(st.to_payload());
            }
            SensorInformation::PostureAngleQuaternion(st) => {
                payload.push(PostureDataType::Quaternions.into());
                payload.extend(st.to_payload());
            }
            SensorInformation::MagneticSensor(st) => {
                payload.extend(st.to_payload());
            }
        }

        payload
    }
}
