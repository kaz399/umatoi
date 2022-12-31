use serde::Serialize;
use serde::Serializer;

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

impl Serialize for ConfigurationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

impl ConfigurationType {
    pub fn response(self) -> u8 {
        u8::from(self) | 0x80u8
    }
}
