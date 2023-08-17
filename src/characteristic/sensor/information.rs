mod magnetic;
mod motion;
mod posture_angle;

use crate::characteristic::sensor::def::CommandId;
use crate::payload::FromPayload;

/// Sensor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SensorInformation {
    MotionDetection(motion::MotionDetectionInformation),
    PostureAngleEuler(posture_angle::PostureAngleEulerInformation),
    PostureAngleQuaternion(posture_angle::PostureAngleQuaternionsInformation),
    MagneticSensor(magnetic::MagneticSensorInformation),
}

impl FromPayload<&[u8]> for SensorInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.is_empty() {
            return None;
        }
        if let Some(response_data) = motion::MotionDetectionInformation::from_payload(payload) {
            return Some(SensorInformation::MotionDetection(response_data));
        }
        if let Some(response_data) = magnetic::MagneticSensorInformation::from_payload(payload) {
            return Some(SensorInformation::MagneticSensor(response_data));
        }
        if let Some(response_data) =
            posture_angle::PostureAngleEulerInformation::from_payload(payload)
        {
            return Some(SensorInformation::PostureAngleEuler(response_data));
        }
        if let Some(response_data) =
            posture_angle::PostureAngleQuaternionsInformation::from_payload(payload)
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
