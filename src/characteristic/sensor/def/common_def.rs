//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_sensor>

use crate::payload::ToPayload;

/// Command
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandId {
    Motion,
    PostureAngle,
    MagneticSensor,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::Motion => 0x81u8,
            CommandId::PostureAngle => 0x83u8,
            CommandId::MagneticSensor => 0x82u8,
        }
    }
}

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

impl CommandId {
    pub fn response(self) -> u8 {
        u8::from(self) & 0x7f
    }
}
