use super::super::def::CommandId;
use crate::payload::FromPayload;

/// Response to magnetic sensor settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-magnetic-sensor-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMagneticSensorData {
    pub result: bool,
}

impl FromPayload<&[u8]> for ResponseMagneticSensorData {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 3 {
            return None;
        }
        if payload[0] == CommandId::MagneticSensor.response() {
            Some(Self {
                result: payload[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}
