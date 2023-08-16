use crate::payload::ToPayload;

/// Command

#[derive(Debug, Copy, Clone)]
pub enum CommandId {
    Run,
    Period,
    TargetPosition,
    MultiTargetPositions,
    Acceleration,
    ObtainSpeed,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::Run => 1u8,
            CommandId::Period => 2u8,
            CommandId::TargetPosition => 3u8,
            CommandId::MultiTargetPositions => 4u8,
            CommandId::Acceleration => 5u8,
            CommandId::ObtainSpeed => 0x60u8,
        }
    }
}

impl CommandId {
    pub fn response(self) -> u8 {
        u8::from(self) | 0x80u8
    }
}

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
