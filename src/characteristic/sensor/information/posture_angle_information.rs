use super::super::def::common_def::CommandId;
use super::super::def::posture_angle_def::PostureDataType;
use crate::payload::ToPayload;

/// Posture angle information (euler)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-euler-angles>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PostureAngleEulerInformation {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
}

impl PostureAngleEulerInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 8 {
            return None;
        }
        if (byte_data[0] == CommandId::PostureAngle.response())
            && (byte_data[1] == u8::from(PostureDataType::Euler))
        {
            Some(PostureAngleEulerInformation {
                roll: i16::from_le_bytes([byte_data[2], byte_data[3]]),
                pitch: i16::from_le_bytes([byte_data[4], byte_data[5]]),
                yaw: i16::from_le_bytes([byte_data[6], byte_data[7]]),
            })
        } else {
            None
        }
    }
}

impl ToPayload<Vec<u8>> for PostureAngleEulerInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.roll.to_le_bytes().to_vec());
        payload.extend(self.pitch.to_le_bytes().to_vec());
        payload.extend(self.yaw.to_le_bytes().to_vec());
        payload
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

impl PostureAngleQuaternionsInformation {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 8 {
            return None;
        }
        if (byte_data[0] == CommandId::PostureAngle.response())
            && (byte_data[1] == u8::from(PostureDataType::Quaternions))
        {
            Some(PostureAngleQuaternionsInformation {
                w: i16::from_le_bytes([byte_data[2], byte_data[3]]),
                x: i16::from_le_bytes([byte_data[4], byte_data[5]]),
                y: i16::from_le_bytes([byte_data[6], byte_data[7]]),
                z: i16::from_le_bytes([byte_data[8], byte_data[9]]),
            })
        } else {
            None
        }
    }
}

impl ToPayload<Vec<u8>> for PostureAngleQuaternionsInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.w.to_le_bytes().to_vec());
        payload.extend(self.x.to_le_bytes().to_vec());
        payload.extend(self.y.to_le_bytes().to_vec());
        payload.extend(self.z.to_le_bytes().to_vec());
        payload
    }
}
