use super::def::ConfigurationType;
use serde::{Serialize, Serializer};

/// Id sensor notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#identification-sensor-id-notification-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetIdSensorNotification {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub minimum_interval: u8,
    pub condition: IdSensorNotificationCondition,
}

impl SetIdSensorNotification {
    pub fn new(minimum_interval: u8, condition: IdSensorNotificationCondition) -> Self {
        Self {
            configuration_type: ConfigurationType::BleProtocolVersion,
            _reserved: 0,
            minimum_interval,
            condition,
        }
    }
}

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

/// Response to
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-identification-sensor-id-notification-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseIdSensorNotificationData {
    pub result: bool,
}

impl ResponseIdSensorNotificationData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data[0] == ConfigurationType::IdSensorNotification.response() {
            Some(Self {
                result: byte_data[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}

/// Id sensor missed notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#identification-sensor-id-missed-notification-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetIdSensorMissedNotification {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub sensitivity: u8,
}

impl SetIdSensorMissedNotification {
    pub fn new(sensitivity: u8) -> Self {
        Self {
            configuration_type: ConfigurationType::IdSensorMissedNotification,
            _reserved: 0,
            sensitivity,
        }
    }
}

/// Response to Id sensor missed notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-identification-sensor-id-missed-notification-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseIdSensorMissedNotificationData {
    pub result: bool,
}

impl ResponseIdSensorMissedNotificationData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data[0] == ConfigurationType::IdSensorMissedNotification.response() {
            Some(Self {
                result: byte_data[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}
