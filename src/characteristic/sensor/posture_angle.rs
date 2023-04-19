use super::def::CommandId;
use crate::payload::ToPayload;
use serde::Serialize;
use serde::Serializer;

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

/// Posture angle information (euler)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-euler-angles>

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PostureAngleEulerData {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
}

impl ToPayload<Vec<u8>> for PostureAngleEulerData {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl PostureAngleEulerData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 8 {
            return None;
        }
        if (byte_data[0] == CommandId::PostureAngle.response())
            && (byte_data[1] == u8::from(PostureDataType::Euler))
        {
            Some(PostureAngleEulerData {
                roll: i16::from_le_bytes([byte_data[2], byte_data[3]]),
                pitch: i16::from_le_bytes([byte_data[4], byte_data[5]]),
                yaw: i16::from_le_bytes([byte_data[6], byte_data[7]]),
            })
        } else {
            None
        }
    }
}

/// Posture angle information (quaternions)
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor#obtaining-posture-angle-information-notifications-in-quaternions>

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PostureAngleQuaternionsData {
    pub w: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl ToPayload<Vec<u8>> for PostureAngleQuaternionsData {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl PostureAngleQuaternionsData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 8 {
            return None;
        }
        if (byte_data[0] == CommandId::PostureAngle.response())
            && (byte_data[1] == u8::from(PostureDataType::Quaternions))
        {
            Some(PostureAngleQuaternionsData {
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

/// Posture data type

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PostureDataType {
    #[default]
    Euler,
    Quaternions,
}

impl From<PostureDataType> for u8 {
    fn from(posture_data_type: PostureDataType) -> u8 {
        match posture_data_type {
            PostureDataType::Euler => 1u8,
            PostureDataType::Quaternions => 2u8,
        }
    }
}

impl Serialize for PostureDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor_posture1() {
        _setup();

        let m = PostureAngleEulerData::default();
        println!("{:?}", m);
        let payload = m.to_payload();
        println!("{:?}", payload);
    }

    #[test]
    fn sensor_posture2() {
        _setup();

        let m = PostureAngleQuaternionsData::default();
        println!("{:?}", m);
        let payload = m.to_payload();
        println!("{:?}", payload);
    }
}
