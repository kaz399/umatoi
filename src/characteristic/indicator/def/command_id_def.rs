//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_light>

use crate::payload::ToPayload;

/// Command

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandId {
    TurningOnAndOff,
    RepeatedTuringOnAndOff,
    TurnOffAll,
    TurnOff,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::TurningOnAndOff => 0x03u8,
            CommandId::RepeatedTuringOnAndOff => 0x04u8,
            CommandId::TurnOffAll => 0x01u8,
            CommandId::TurnOff => 0x02u8,
        }
    }
}

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
