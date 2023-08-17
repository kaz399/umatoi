use super::super::def::command_id::CommandId;
use crate::payload::ToPayload;

/// Request motion sensor information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_sensor#requesting-motion-detection-information>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        let payload: Vec<u8> = vec![self.command.into()];
        payload
    }
}
