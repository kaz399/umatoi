use crate::payload::ToPayload;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandId {
    BleProtocolVersion,
    HorizontalDetectionThreshold,
    CollisionDetectionThreshold,
    DoubleTapDetectionTimeInterval,
    IdSensorNotification,
    IdSensorMissedNotification,
    MagneticSensor,
    MotorSpeed,
    PostureAngleDetection,
}

impl From<CommandId> for u8 {
    fn from(configuration_type: CommandId) -> u8 {
        match configuration_type {
            CommandId::BleProtocolVersion => 0x01u8,
            CommandId::HorizontalDetectionThreshold => 0x05u8,
            CommandId::CollisionDetectionThreshold => 0x06u8,
            CommandId::DoubleTapDetectionTimeInterval => 0x17u8,
            CommandId::IdSensorNotification => 0x18u8,
            CommandId::IdSensorMissedNotification => 0x19u8,
            CommandId::MagneticSensor => 0x1bu8,
            CommandId::MotorSpeed => 0x1cu8,
            CommandId::PostureAngleDetection => 0x1du8,
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
