use super::super::def::common_def::ConfigurationType;
use super::super::def::id_sensor_def::IdSensorNotificationCondition;
use serde::Serialize;

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

