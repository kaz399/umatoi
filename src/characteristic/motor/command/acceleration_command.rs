use crate::characteristic::motor::def::acceleration_def::{
    Acceleration, AngleVelocity, MovingDirection, Priority,
};
use crate::characteristic::motor::def::{CommandId, Period};
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::acceleration_def::{
        Acceleration, AngleVelocity, MovingDirection, Priority, RotationDirection,
    };

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
