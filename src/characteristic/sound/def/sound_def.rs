//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_sound>

use crate::payload::ToPayload;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandId {
    PlaySoundEffect,
    PlayMidiNotes,
    StopSound,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::PlaySoundEffect => 0x02u8,
            CommandId::PlayMidiNotes => 0x03u8,
            CommandId::StopSound => 0x01u8,
        }
    }
}

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

