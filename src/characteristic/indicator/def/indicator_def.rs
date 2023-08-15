//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_light>

pub use crate::characteristic::motor::def::common_def::Period;
use crate::payload::ToPayload;

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

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Indicator color

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
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
        let payload: Vec<u8> = vec![self.r, self.g, self.b];
        payload
    }
}

/// Indicator parameter

#[derive(Default, Debug, Copy, Clone)]
pub struct IndicatorParam {
    pub duration: Period,
    pub id: u8,
    pub color: Color,
}

impl ToPayload<Vec<u8>> for IndicatorParam {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.duration.to_payload());
        payload.push(1u8);
        payload.push(self.id);
        payload.extend(self.color.to_payload());
        payload
    }
}
