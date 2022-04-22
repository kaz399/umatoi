use super::def::{CommandId, MotorDriveParameter, MotorId, Period, Velocity};
use crate::payload::ToPayload;
use serde::Serialize;
use std::error::Error;

/// Motor control
/// <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control>

#[derive(Serialize, Debug, Copy, Clone)]
pub struct MotorControl {
    command: CommandId,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
}

impl Default for MotorControl {
    fn default() -> Self {
        Self {
            command: CommandId::Run,
            left: MotorDriveParameter::default(),
            right: MotorDriveParameter::default(),
        }
    }
}

impl ToPayload<Vec<u8>> for MotorControl {
    /// convert to BLE payload
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
impl MotorControl {
    /// create new struct from primitive type parameters
    pub fn from_primitive(
        left: i16,
        right: i16,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let left = MotorDriveParameter::new(MotorId::Left, Velocity::new(left)?)?;
        let right = MotorDriveParameter::new(MotorId::Right, Velocity::new(right)?)?;
        Ok(Self {
            left,
            right,
            ..MotorControl::default()
        })
    }
}

/// Motor control with specified duration
/// <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration>

#[derive(Serialize, Debug, Copy, Clone)]
pub struct MotorControlWithSpecifiledDuration {
    command: CommandId,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
    period: Period,
}

impl Default for MotorControlWithSpecifiledDuration {
    fn default() -> Self {
        Self {
            command: CommandId::Period,
            left: MotorDriveParameter::default(),
            right: MotorDriveParameter::default(),
            period: Period::default(),
        }
    }
}

impl ToPayload<Vec<u8>> for MotorControlWithSpecifiledDuration {
    /// convert to BLE payload
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl MotorControlWithSpecifiledDuration {
    /// create new struct from primitive type parameters
    pub fn from_primitive(
        left: i16,
        right: i16,
        period_ms: u64,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let left = MotorDriveParameter::new(MotorId::Left, Velocity::new(left)?)?;
        let right = MotorDriveParameter::new(MotorId::Right, Velocity::new(right)?)?;
        let period = Period::from_millis(period_ms);
        Ok(Self {
            left,
            right,
            period,
            ..MotorControlWithSpecifiledDuration::default()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cube::motor::def::{MotorId, Velocity};

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_control_test1() {
        _setup();

        let run_default = MotorControl::default();
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 7);
    }

    #[test]
    fn motor_control_test2() {
        _setup();

        let run_default = MotorControlWithSpecifiledDuration::default();
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 8);
    }

    #[test]
    fn motor_control_test3() {
        _setup();

        let run_default = MotorControl::from_primitive(10, -11).unwrap();
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 7);
    }

    #[test]
    fn motor_control_test4() {
        _setup();

        let run_default = MotorControl {
            left: MotorDriveParameter::new(MotorId::Left, Velocity::new(-10).unwrap()).unwrap(),
            right: MotorDriveParameter::new(MotorId::Right, Velocity::new(-10).unwrap()).unwrap(),
            ..MotorControl::default()
        };
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 7);
    }
}
