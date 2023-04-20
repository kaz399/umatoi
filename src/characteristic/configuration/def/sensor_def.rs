use serde::Serialize;
use serde::Serializer;

/// Magnet function

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MagnetFunction {
    Disable,
    MagnetState,
    MagnetForce,
}

impl From<MagnetFunction> for u8 {
    fn from(function_type: MagnetFunction) -> u8 {
        match function_type {
            MagnetFunction::Disable => 0x00u8,
            MagnetFunction::MagnetState => 0x01u8,
            MagnetFunction::MagnetForce => 0x02u8,
        }
    }
}

impl Serialize for MagnetFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Magnet condition

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MagnetNotificationCondition {
    Always,
    AtChange,
}

impl From<MagnetNotificationCondition> for u8 {
    fn from(function_type: MagnetNotificationCondition) -> u8 {
        match function_type {
            MagnetNotificationCondition::Always => 0x00u8,
            MagnetNotificationCondition::AtChange => 0x01u8,
        }
    }
}

impl Serialize for MagnetNotificationCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Posture notify condition

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PostureAngleNotificationCondition {
    Always,
    AtChange,
}

impl From<PostureAngleNotificationCondition> for u8 {
    fn from(condition: PostureAngleNotificationCondition) -> u8 {
        match condition {
            PostureAngleNotificationCondition::Always => 0x00u8,
            PostureAngleNotificationCondition::AtChange => 0x01u8,
        }
    }
}
