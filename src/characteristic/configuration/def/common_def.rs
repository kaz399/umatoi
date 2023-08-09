use crate::payload::ToPayload;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConfigurationType {
    BleProtocolVersion,
    HorizontalDetectionThreshold,
    CollisionDetectionTheshold,
    DoubleTapDetectionTimeinterval,
    IdSensorNotification,
    IdSensorMissedNotification,
    MagneticSensor,
    MotorSpeed,
    PostureAngleDetection,
}

impl From<ConfigurationType> for u8 {
    fn from(configuration_type: ConfigurationType) -> u8 {
        match configuration_type {
            ConfigurationType::BleProtocolVersion => 0x01u8,
            ConfigurationType::HorizontalDetectionThreshold => 0x05u8,
            ConfigurationType::CollisionDetectionTheshold => 0x06u8,
            ConfigurationType::DoubleTapDetectionTimeinterval => 0x17u8,
            ConfigurationType::IdSensorNotification => 0x18u8,
            ConfigurationType::IdSensorMissedNotification => 0x19u8,
            ConfigurationType::MagneticSensor => 0x1bu8,
            ConfigurationType::MotorSpeed => 0x1cu8,
            ConfigurationType::PostureAngleDetection => 0x1du8,
        }
    }
}

impl ConfigurationType {
    pub fn response(self) -> u8 {
        u8::from(self) | 0x80u8
    }
}

impl ToPayload<Vec<u8>> for ConfigurationType {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
