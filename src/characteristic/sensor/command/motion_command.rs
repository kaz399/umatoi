use super::super::def::common_def::CommandId;
use crate::payload::ToPayload;
use serde::Serialize;

/// Request motion sensor information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#requesting-motion-detection-information>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestMotionDetection {
    pub command: CommandId,
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

