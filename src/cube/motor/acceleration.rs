use crate::payload::ToPayload;
use serde::ser::Serializer;
use serde::Serialize;

use super::def::{CommandId, Period};

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-acceleration-specified>

#[derive(Serialize, Debug, Copy, Clone)]
struct MotorControlAccleration {
    command: CommandId,
    id: u8,
    acceleration: Acceleration,
    angle_velocity: AngleVelocity,
    moving_direction: MovingDirection,
    priority: Priority,
    period: Period,
}

impl ToPayload<u8> for MotorControlAccleration {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Acceleration

#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
pub struct Acceleration {
    translational_speed: u8,
    acceleration: u8,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self {
            translational_speed: 0,
            acceleration: 0,
        }
    }
}

/// Angle velocity

#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
pub struct AngleVelocity {
    rotation_velocity: u16,
    rotation_direction: RotationDirection,
}

impl Default for AngleVelocity {
    fn default() -> Self {
        Self {
            rotation_velocity: 0,
            rotation_direction: RotationDirection::Positive,
        }
    }
}

/// Rotation direction

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationDirection {
    Positive,
    Negative,
}

impl From<RotationDirection> for u8 {
    fn from(rotation_direction: RotationDirection) -> u8 {
        match rotation_direction {
            RotationDirection::Positive => 0u8,
            RotationDirection::Negative => 1u8,
        }
    }
}

impl Serialize for RotationDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Moving direction

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovingDirection {
    Forward,
    Backward,
}

impl From<MovingDirection> for u8 {
    fn from(moving_direction: MovingDirection) -> u8 {
        match moving_direction {
            MovingDirection::Forward => 0u8,
            MovingDirection::Backward => 1u8,
        }
    }
}

impl Serialize for MovingDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Priority (MotorCommandId::Acceleration)

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Priority {
    TranslationalSpeed,
    RotationVelocity,
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> u8 {
        match priority {
            Priority::TranslationalSpeed => 0u8,
            Priority::RotationVelocity => 1u8,
        }
    }
}

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
    }
}
