use super::super::def::common_def::CommandId;
use super::super::def::motion_def::Posture;
use serde::Serialize;

/// Motion detection information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#obtaining-motion-detection-information>

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotionDetectionInformation {
    pub horizontal: bool,
    pub collision: bool,
    pub double_tap: bool,
    pub posture: Posture,
    pub shake: u8,
}

impl MotionDetectionInformation {
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

