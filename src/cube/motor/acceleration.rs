use super::def::{CommandId, Period};
use crate::payload::ToPayload;
use serde::Serialize;
use serde::Serializer;

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-acceleration-specified>

#[derive(Serialize, Debug, Copy, Clone)]
struct MotorControlAccleration {
    command: CommandId,
    acceleration: Acceleration,
    angle_velocity: AngleVelocity,
    moving_direction: MovingDirection,
    priority: Priority,
    period: Period,
}

impl ToPayload<Vec<u8>> for MotorControlAccleration {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl Default for MotorControlAccleration {
    fn default() -> Self {
        Self {
            command: CommandId::Acceleration,
            acceleration: Acceleration::default(),
            angle_velocity: AngleVelocity::default(),
            moving_direction: MovingDirection::Forward,
            priority: Priority::TranslationalSpeed,
            period: Period::default(),
        }
    }
}

/// Acceleration

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Acceleration {
    translational_speed: u8,
    acceleration: u8,
}

/// Angle velocity

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    fn motor_acceleration1() {
        _setup();

        let st = MotorControlAccleration::default();
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 9);

        let st = MotorControlAccleration {
            acceleration: Acceleration {
                translational_speed: 10,
                acceleration: 20,
            },
            angle_velocity: AngleVelocity {
                rotation_velocity: 30,
                rotation_direction: RotationDirection::Negative,
            },
            moving_direction: MovingDirection::Backward,
            priority: Priority::RotationVelocity,
            period: Period::from_millis(40),
            ..MotorControlAccleration::default()
        };
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 9);
    }
}
