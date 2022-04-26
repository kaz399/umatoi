//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_sensor>

use serde::Serialize;
use serde::Serializer;

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

impl Serialize for CommandId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}
