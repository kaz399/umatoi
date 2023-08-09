use super::super::def::common_def::ConfigurationType;

/// Response to motor speed information\
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-motor-speed-information-acquisition-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseEnableMotorSpeedData {
    pub result: bool,
}

impl ResponseEnableMotorSpeedData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() < 3 {
            return None;
        }
        if byte_data[0] == ConfigurationType::MotorSpeed.response() {
            Some(Self {
                result: byte_data[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}
