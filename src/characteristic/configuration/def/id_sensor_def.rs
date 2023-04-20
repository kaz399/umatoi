use serde::{Serialize, Serializer};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IdSensorNotificationCondition {
    Always,
    AtChange,
    Sleep300WithNoChange,
}

impl From<IdSensorNotificationCondition> for u8 {
    fn from(condition: IdSensorNotificationCondition) -> u8 {
        match condition {
            IdSensorNotificationCondition::Always => 0x00u8,
            IdSensorNotificationCondition::AtChange => 0x01u8,
            IdSensorNotificationCondition::Sleep300WithNoChange => 0xffu8,
        }
    }
}

impl Serialize for IdSensorNotificationCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

