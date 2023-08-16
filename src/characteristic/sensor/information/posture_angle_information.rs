use super::super::def::command_id_def::CommandId;
use super::super::def::posture_angle_def::PostureDataType;
use crate::payload::FromPayload;

/// Posture angle information (euler)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-euler-angles>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PostureAngleEulerInformation {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
}

impl FromPayload<&[u8]> for PostureAngleEulerInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 8 {
            return None;
        }
        if (payload[0] == CommandId::PostureAngle.response())
            && (payload[1] == u8::from(PostureDataType::Euler))
        {
            Some(PostureAngleEulerInformation {
                roll: i16::from_le_bytes([payload[2], payload[3]]),
                pitch: i16::from_le_bytes([payload[4], payload[5]]),
                yaw: i16::from_le_bytes([payload[6], payload[7]]),
            })
        } else {
            None
        }
    }
}

/// Posture angle information (quaternions)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-quaternions>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PostureAngleQuaternionsInformation {
    pub w: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl FromPayload<&[u8]> for PostureAngleQuaternionsInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 8 {
            return None;
        }
        if (payload[0] == CommandId::PostureAngle.response())
            && (payload[1] == u8::from(PostureDataType::Quaternions))
        {
            Some(PostureAngleQuaternionsInformation {
                w: i16::from_le_bytes([payload[2], payload[3]]),
                x: i16::from_le_bytes([payload[4], payload[5]]),
                y: i16::from_le_bytes([payload[6], payload[7]]),
                z: i16::from_le_bytes([payload[8], payload[9]]),
            })
        } else {
            None
        }
    }
}
