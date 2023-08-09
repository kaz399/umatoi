use crate::payload::ToPayload;

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

impl ToPayload<Vec<u8>> for MagnetFunction {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
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

impl ToPayload<Vec<u8>> for MagnetNotificationCondition {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
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

impl ToPayload<Vec<u8>> for PostureAngleNotificationCondition {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
