use serde::Serialize;

use super::super::def::common_def::ConfigurationType;


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
