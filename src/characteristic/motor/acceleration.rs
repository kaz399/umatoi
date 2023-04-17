use byteorder::{LittleEndian, WriteBytesExt};
use super::def::{CommandId, Period};
use crate::payload::ToPayload;

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-acceleration-specified>

#[derive(Debug, Copy, Clone)]
pub struct MotorControlAcceleration {
    pub command: CommandId,
    pub acceleration: Acceleration,
    pub angle_velocity: AngleVelocity,
    pub moving_direction: MovingDirection,
    pub priority: Priority,
    pub period: Period,
}

impl ToPayload<Vec<u8>> for MotorControlAcceleration {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.command.to_payload());
        payload.extend(self.acceleration.to_payload());
        payload.extend(self.angle_velocity.to_payload());
        payload.extend(self.moving_direction.to_payload());
        payload.extend(self.priority.to_payload());
        payload.extend(self.period.to_payload());
        payload
    }
}

impl Default for MotorControlAcceleration {
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Acceleration {
    pub translational_speed: u8,
    pub acceleration: u8,
}

impl ToPayload<Vec<u8>> for Acceleration {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.translational_speed, self.acceleration];
        payload
    }
}


/// Angle velocity

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AngleVelocity {
    pub rotation_velocity: u16,
    pub rotation_direction: RotationDirection,
}

impl Default for AngleVelocity {
    fn default() -> Self {
        Self {
            rotation_velocity: 0,
            rotation_direction: RotationDirection::Positive,
        }
    }
}

impl ToPayload<Vec<u8>> for AngleVelocity {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.write_u16::<LittleEndian>(self.rotation_velocity).unwrap();
        payload.extend(self.rotation_direction.to_payload());
        payload
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

impl ToPayload<Vec<u8>> for RotationDirection {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
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

impl ToPayload<Vec<u8>> for MovingDirection {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
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

impl ToPayload<Vec<u8>> for Priority {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
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

        let st = MotorControlAcceleration::default();
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 9);

        let st = MotorControlAcceleration {
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
            ..MotorControlAcceleration::default()
        };
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 9);
    }
}
