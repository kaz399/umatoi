use super::super::def::command_id::CommandId;
use super::super::def::motion::Posture;
use crate::payload::FromPayload;

/// Motion detection information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#obtaining-motion-detection-information>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotionDetectionInformation {
    pub horizontal: bool,
    pub collision: bool,
    pub double_tap: bool,
    pub posture: Posture,
    pub shake: u8,
}

impl FromPayload<&[u8]> for MotionDetectionInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 6 {
            return None;
        }
        if payload[0] == CommandId::Motion.response() {
            Some(Self {
                horizontal: payload[1] != 0,
                collision: payload[2] != 0,
                double_tap: payload[3] != 0,
                posture: Posture::from(payload[4]),
                shake: payload[5],
            })
        } else {
            None
        }
    }
}
