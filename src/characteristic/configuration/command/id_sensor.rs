use super::super::def::command_id::CommandId;
use super::super::def::condition::IdSensorNotificationCondition;
use crate::payload::ToPayload;

/// Id sensor notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#identification-sensor-id-notification-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetIdSensorNotification {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub minimum_interval: u8,
    pub condition: IdSensorNotificationCondition,
}

impl SetIdSensorNotification {
    pub fn new(minimum_interval: u8, condition: IdSensorNotificationCondition) -> Self {
        Self {
            configuration_type: CommandId::BleProtocolVersion,
            _reserved: 0,
            minimum_interval,
            condition,
        }
    }
}

impl ToPayload<Vec<u8>> for SetIdSensorNotification {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.configuration_type.to_payload());
        payload.push(self._reserved);
        payload.push(self.minimum_interval);
        payload.extend(self.condition.to_payload());
        payload
    }
}

/// Id sensor missed notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#identification-sensor-id-missed-notification-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetIdSensorMissedNotification {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub sensitivity: u8,
}

impl SetIdSensorMissedNotification {
    pub fn new(sensitivity: u8) -> Self {
        Self {
            configuration_type: CommandId::IdSensorMissedNotification,
            _reserved: 0,
            sensitivity,
        }
    }
}

impl ToPayload<Vec<u8>> for SetIdSensorMissedNotification {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.configuration_type.to_payload());
        payload.push(self._reserved);
        payload.push(self.sensitivity);
        payload
    }
}
