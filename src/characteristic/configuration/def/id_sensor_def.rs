use crate::payload::ToPayload;

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
