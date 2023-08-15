//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_light>

use crate::payload::ToPayload;

use super::super::def::indicator_def::CommandId;
use super::super::def::indicator_def::IndicatorParam;
use crate::integer_converter::u_to_u8;

/// TurningOnAndOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turning-the-indicator-on-and-off>

#[derive(Debug, Copy, Clone)]
pub struct TurningOnAndOff {
    pub command: CommandId,
    pub id: u8,
    pub param: IndicatorParam,
}

impl Default for TurningOnAndOff {
    fn default() -> Self {
        Self {
            command: CommandId::TurningOnAndOff,
            id: 1,
            param: IndicatorParam::default(),
        }
    }
}

impl ToPayload<Vec<u8>> for TurningOnAndOff {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![self.command.into()];
        payload.extend(self.param.to_payload());
        payload
    }
}

/// RepeatedTuringOnAndOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#repeated-turning-on-and-off-of-indicator>

#[derive(Debug, Clone)]
pub struct RepeatedTuringOnAndOff {
    pub command: CommandId,
    pub repeat: u8,
    pub id: u8,
    pub param_list: Vec<IndicatorParam>,
}

impl Default for RepeatedTuringOnAndOff {
    fn default() -> Self {
        Self {
            command: CommandId::RepeatedTuringOnAndOff,
            repeat: 1,
            id: 1,
            param_list: Vec::new(),
        }
    }
}

impl ToPayload<Vec<u8>> for RepeatedTuringOnAndOff {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![self.command.into(), self.repeat];
        payload.push(u_to_u8(self.param_list.len()));
        for param in self.param_list {
            payload.extend(param.to_payload());
        }
        payload
    }
}

/// TurnOffAll
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turn-off-all-indicators>

#[derive(Debug, Copy, Clone)]
pub struct TurnOffAll {
    pub command: CommandId,
}

impl Default for TurnOffAll {
    fn default() -> Self {
        Self {
            command: CommandId::TurnOffAll,
        }
    }
}

impl ToPayload<Vec<u8>> for TurnOffAll {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.command.into()];
        payload
    }
}

/// TurnOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turn-off-a-specific-indicator>

#[derive(Debug, Copy, Clone)]
pub struct TurnOff {
    pub command: CommandId,
    pub id: u8,
}

impl Default for TurnOff {
    fn default() -> Self {
        Self {
            command: CommandId::TurnOff,
            id: 1,
        }
    }
}

impl ToPayload<Vec<u8>> for TurnOff {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.command.into(), self.id];
        payload
    }
}
