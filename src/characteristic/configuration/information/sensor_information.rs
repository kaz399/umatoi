use super::super::def::ConfigurationType;

/// Response to magnetic sensor settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-magnetic-sensor-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMagneticSensorData {
    pub result: bool,
}

impl ResponseMagneticSensorData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 3 {
            return None;
        }
        if byte_data[0] == ConfigurationType::MagneticSensor.response() {
            Some(Self {
                result: byte_data[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}
