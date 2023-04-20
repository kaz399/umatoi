use super::super::def::ConfigurationType;
use serde::Serialize;

/// Enable motor speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#motor-speed-information-acquisition-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct EnableMotorSpeed {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub enable: bool,
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

