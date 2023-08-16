use super::super::def::CommandId;
use crate::payload::ToPayload;

/// Enable motor speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#motor-speed-information-acquisition-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EnableMotorSpeed {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub enable: bool,
}

impl EnableMotorSpeed {
    pub fn new(enable: bool) -> Self {
        Self {
            configuration_type: CommandId::MotorSpeed,
            _reserved: 0,
            enable,
        }
    }
}

impl ToPayload<Vec<u8>> for EnableMotorSpeed {
    fn to_payload(self) -> Vec<u8> {
        let enable: u8 = if self.enable { 1 } else { 0 };
        let payload: Vec<u8> = vec![self.configuration_type.into(), self._reserved, enable];
        payload
    }
}
