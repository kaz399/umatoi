use crate::payload::ToPayload;

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

/// Id sensor notification condition

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

impl ToPayload<Vec<u8>> for IdSensorNotificationCondition {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
