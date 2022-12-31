use super::def::CommandId;
use crate::payload::ToPayload;
use serde::Serialize;
use serde::Serializer;

/// Request motion sensor information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#requesting-motion-detection-information>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestMotionDetection {
    command: CommandId,
}

impl Default for RequestMotionDetection {
    fn default() -> Self {
        Self {
            command: CommandId::Motion,
        }
    }
}

impl ToPayload<Vec<u8>> for RequestMotionDetection {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Motion detection information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#obtaining-motion-detection-information>

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotionDetectionData {
    pub horizontal: bool,
    pub collision: bool,
    pub double_tap: bool,
    pub posture: Posture,
    pub shake: u8,
}

impl MotionDetectionData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 6 {
            return None;
        }
        if byte_data[0] == CommandId::Motion.response() {
            Some(Self {
                horizontal: byte_data[1] != 0,
                collision: byte_data[2] != 0,
                double_tap: byte_data[3] != 0,
                posture: Posture::from(byte_data[4]),
                shake: byte_data[5],
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Posture {
    Unknown,
    Top,
    Bottom,
    Rear,
    Front,
    Right,
    Left,
}

impl From<Posture> for u8 {
    fn from(posture: Posture) -> u8 {
        match posture {
            Posture::Unknown => 0u8,
            Posture::Top => 1u8,
            Posture::Bottom => 2u8,
            Posture::Rear => 3u8,
            Posture::Front => 4u8,
            Posture::Right => 5u8,
            Posture::Left => 6u8,
        }
    }
}

impl From<u8> for Posture {
    fn from(code: u8) -> Posture {
        match code {
            1u8 => Posture::Top,
            2u8 => Posture::Bottom,
            3u8 => Posture::Rear,
            4u8 => Posture::Front,
            5u8 => Posture::Right,
            6u8 => Posture::Left,
            _ => Posture::Unknown,
        }
    }
}

impl Default for Posture {
    fn default() -> Self {
        Posture::Unknown
    }
}

impl Serialize for Posture {
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
    fn sensor_motion1() {
        _setup();

        let m = MotionDetectionData::default();
        println!("{:?}", m);
    }
}
