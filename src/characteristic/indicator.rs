//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_light>

use crate::payload::ToPayload;
use serde::Serialize;
use serde::Serializer;
use std::cmp::min;

/// Command
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandId {
    TurningOnAndOff,
    RepeatedTuringOnAndOff,
    TurnOffAll,
    TurnOff,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::TurningOnAndOff => 0x03u8,
            CommandId::RepeatedTuringOnAndOff => 0x04u8,
            CommandId::TurnOffAll => 0x01u8,
            CommandId::TurnOff => 0x02u8,
        }
    }
}

impl Serialize for CommandId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Indicator color

#[derive(Serialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<&[u8]> for Color {
    fn from(color: &[u8]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self {
            r: ((color >> 24) & 0xff) as u8,
            g: ((color >> 16) & 0xff) as u8,
            b: ((color >> 8) & 0xff) as u8,
        }
    }
}

impl ToPayload<Vec<u8>> for Color {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Period

#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct Period {
    pub period: u8,
}

impl From<Period> for u8 {
    fn from(running_period: Period) -> u8 {
        running_period.period
    }
}

impl Period {
    pub fn forever() -> Self {
        Self { period: 0 }
    }

    pub fn from_millis(period: u64) -> Self {
        let period = period / 10;
        match period {
            1..=255 => Period {
                period: period as u8,
            },
            _ => Period { period: 0 },
        }
    }
}

impl ToPayload<Vec<u8>> for Period {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Indicator parameter

#[derive(Default, Debug, Copy, Clone)]
pub struct Params {
    pub duration: Period,
    pub color: Color,
}

impl ToPayload<Vec<u8>> for Params {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.duration.to_payload();
        payload.push(1);
        payload.push(1);
        payload.extend(&self.color.to_payload());
        payload
    }
}

/// TurningOnAndOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turning-the-indicator-on-and-off>

#[derive(Debug, Copy, Clone)]
pub struct TurningOnAndOff {
    pub command: CommandId,
    pub id: u8,
    pub params: Params,
}

impl ToPayload<Vec<u8>> for TurningOnAndOff {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = bincode::serialize(&self.command).unwrap();
        payload.extend(&self.params.to_payload());
        payload[3] = self.id;
        payload
    }
}

impl Default for TurningOnAndOff {
    fn default() -> Self {
        Self {
            command: CommandId::TurningOnAndOff,
            id: 0x01,
            params: Params::default(),
        }
    }
}

/// RepeatedTuringOnAndOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#repeated-turning-on-and-off-of-indicator>

#[derive(Debug, Clone)]
pub struct RepeatedTuringOnAndOff {
    pub command: CommandId,
    pub repeat: u8,
    pub id: u8,
    pub params_list: Vec<Params>,
}

impl ToPayload<Vec<u8>> for RepeatedTuringOnAndOff {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = bincode::serialize(&self.command).unwrap();
        payload.push(self.repeat);
        let num_of_params = min(self.params_list.len(), u8::MAX.into()) as u8;
        payload.push(num_of_params);
        for params in &self.params_list {
            let mut params_payload = params.to_payload();
            params_payload[2] = self.id;
            payload.extend(params_payload);
        }
        payload
    }
}

impl Default for RepeatedTuringOnAndOff {
    fn default() -> Self {
        Self {
            command: CommandId::RepeatedTuringOnAndOff,
            repeat: 1,
            id: 1,
            params_list: Vec::new(),
        }
    }
}

/// TurnOffAll
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turn-off-all-indicators>

#[derive(Debug, Copy, Clone)]
pub struct TurnOffAll {
    command: CommandId,
}

impl ToPayload<Vec<u8>> for TurnOffAll {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self.command).unwrap()
    }
}

impl Default for TurnOffAll {
    fn default() -> Self {
        Self {
            command: CommandId::TurnOffAll,
        }
    }
}

/// TurnOff
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_light#turn-off-a-specific-indicator>

#[derive(Debug, Copy, Clone)]
pub struct TurnOff {
    command: CommandId,
    pub id: u8,
}

impl ToPayload<Vec<u8>> for TurnOff {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = bincode::serialize(&self.command).unwrap();
        payload.push(1);
        payload.push(self.id);
        payload
    }
}

impl Default for TurnOff {
    fn default() -> Self {
        Self {
            command: CommandId::TurnOff,
            id: 1,
        }
    }
}
