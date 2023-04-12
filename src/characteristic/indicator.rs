//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_light>

use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::device_interface::CubeInterface;
use anyhow::Result;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;

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

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut color = serializer.serialize_struct("Color", 3)?;
        color.serialize_field("r", &self.r)?;
        color.serialize_field("g", &self.g)?;
        color.serialize_field("b", &self.b)?;
        color.end()
    }
}

/// Period

#[derive(Default, Debug, Copy, Clone)]
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

impl Serialize for Period {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut color = serializer.serialize_struct("Period", 1)?;
        color.serialize_field("period", &self.period)?;
        color.end()
    }
}

/// Indicator parameter

#[derive(Default, Debug, Copy, Clone)]
pub struct Params {
    pub duration: Period,
    pub color: Color,
}

impl Serialize for Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = bincode::serialize(&self.duration).unwrap();
        let color = bincode::serialize(&self.color).unwrap();
        let mut params = serializer.serialize_seq(Some(duration.len() + 2 + color.len()))?;
        for data in duration {
            params.serialize_element(&data)?;
        }
        params.serialize_element(&1u8)?;
        params.serialize_element(&1u8)?;
        for data in color {
            params.serialize_element(&data)?;
        }
        params.end()
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

impl Default for TurningOnAndOff {
    fn default() -> Self {
        Self {
            command: CommandId::TurningOnAndOff,
            id: 1,
            params: Params::default(),
        }
    }
}

impl Serialize for TurningOnAndOff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let params = bincode::serialize(&self.params).unwrap();
        let mut payload = serializer.serialize_seq(Some(params.len() + 1))?;
        payload.serialize_element(&self.command)?;
        for (i, data) in params.iter().enumerate() {
            if (i % 6) == 2 {
                payload.serialize_element(&self.id)?;
            } else {
                payload.serialize_element(&data)?;
            }
        }
        payload.end()
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

impl Serialize for RepeatedTuringOnAndOff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let params_list = bincode::serialize(&self.params_list).unwrap();
        let mut payload = serializer.serialize_seq(Some(params_list.len() + 3))?;
        payload.serialize_element(&self.command)?;
        payload.serialize_element(&self.repeat)?;
        payload.serialize_element(&self.params_list.len())?;
        for (i, data) in params_list.iter().enumerate() {
            if (i % 6) == 2 {
                payload.serialize_element(&self.id)?;
            } else {
                payload.serialize_element(&data)?;
            }
        }
        payload.end()
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

impl Serialize for TurnOffAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut payload = serializer.serialize_seq(Some(1))?;
        payload.serialize_element(&self.command)?;
        payload.end()
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

impl Serialize for TurnOff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut payload = serializer.serialize_seq(Some(2))?;
        payload.serialize_element(&self.command)?;
        payload.serialize_element(&self.id)?;
        payload.end()
    }
}

pub async fn write(interface: &dyn CubeInterface, bytes: &[u8]) -> Result<bool> {
    interface.write(CoreCubeUuid::LightCtrl.into(), bytes).await
}
