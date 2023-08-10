use super::super::def::common_def::ConfigurationType;
use crate::payload::FromPayload;

/// Response to
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-identification-sensor-id-notification-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseIdSensorNotificationData {
    pub result: bool,
}

impl FromPayload<&[u8]> for ResponseIdSensorNotificationData {
    fn from_payload(payload: &[u8]) -> Option<Self> where Self: Sized {
        if payload[0] == ConfigurationType::IdSensorNotification.response() {
            Some(Self {
                result: payload[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}

/// Response to Id sensor missed notification settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-identification-sensor-id-missed-notification-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseIdSensorMissedNotificationData {
    pub result: bool,
}

impl FromPayload<&[u8]> for ResponseIdSensorMissedNotificationData {
    fn from_payload(payload: &[u8]) -> Option<Self> where Self: Sized {
        if payload[0] == ConfigurationType::IdSensorMissedNotification.response() {
            Some(Self {
                result: payload[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}

