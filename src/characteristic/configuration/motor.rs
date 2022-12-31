use super::def::ConfigurationType;
use serde::Serialize;

/// Enable motor speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#motor-speed-information-acquisition-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct EnableMotorSpeed {
    configuration_type: ConfigurationType,
    _reserved: u8,
    enable: bool,
}

impl EnableMotorSpeed {
    pub fn new(enable: bool) -> Self {
        Self {
            configuration_type: ConfigurationType::MotorSpeed,
            _reserved: 0,
            enable,
        }
    }
}

/// Response to motor speed information\
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-motor-speed-information-acquisition-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseEnableMotorSpeedData {
    result: bool,
}

impl ResponseEnableMotorSpeedData {
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
