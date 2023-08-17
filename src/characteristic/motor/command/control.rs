use crate::characteristic::motor::def::{
    CommandId, MotorDriveParameter, MotorId, Period, Velocity,
};
use crate::payload::ToPayload;
use std::error::Error;

/// Motor control
/// <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control>

#[derive(Debug, Copy, Clone)]
pub struct MotorControl {
    pub command: CommandId,
    pub left: MotorDriveParameter,
    pub right: MotorDriveParameter,
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

impl MotorControl {
    /// create new struct from primitive type parameters
    pub fn set_value(
        left: i16,
        right: i16,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let left = MotorDriveParameter::new(MotorId::Left, Velocity::set_value(left)?)?;
        let right = MotorDriveParameter::new(MotorId::Right, Velocity::set_value(right)?)?;
        Ok(Self {
            left,
            right,
            ..MotorControl::default()
        })
    }
}

impl ToPayload<Vec<u8>> for MotorControl {
    /// convert to BLE payload
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.command.to_payload());
        payload.extend(self.left.to_payload());
        payload.extend(self.right.to_payload());
        payload
    }
}

/// Motor control with specified duration
/// <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration>

#[derive(Debug, Copy, Clone)]
pub struct MotorControlWithSpecifiedDuration {
    pub command: CommandId,
    pub left: MotorDriveParameter,
    pub right: MotorDriveParameter,
    pub period: Period,
}

impl Default for MotorControlWithSpecifiedDuration {
    fn default() -> Self {
        Self {
            command: CommandId::Period,
            left: MotorDriveParameter::default(),
            right: MotorDriveParameter::default(),
            period: Period::default(),
        }
    }
}

impl MotorControlWithSpecifiedDuration {
    /// create new struct from primitive type parameters
    pub fn set_value(
        left: i16,
        right: i16,
        period_ms: usize,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let left = MotorDriveParameter::new(MotorId::Left, Velocity::set_value(left)?)?;
        let right = MotorDriveParameter::new(MotorId::Right, Velocity::set_value(right)?)?;
        let period = Period::from_millis(period_ms);
        Ok(Self {
            left,
            right,
            period,
            ..MotorControlWithSpecifiedDuration::default()
        })
    }
}

impl ToPayload<Vec<u8>> for MotorControlWithSpecifiedDuration {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.command.to_payload());
        payload.extend(self.left.to_payload());
        payload.extend(self.right.to_payload());
        payload.extend(self.period.to_payload());
        payload
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::{MotorId, Velocity};

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

        let run_default = MotorControlWithSpecifiedDuration::default();
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 8);
    }

    #[test]
    fn motor_control_test3() {
        _setup();

        let run_default = MotorControl::set_value(10, -11).unwrap();
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 7);
    }

    #[test]
    fn motor_control_test4() {
        _setup();

        let run_default = MotorControl {
            left: MotorDriveParameter::new(MotorId::Left, Velocity::set_value(-10).unwrap())
                .unwrap(),
            right: MotorDriveParameter::new(MotorId::Right, Velocity::set_value(-10).unwrap())
                .unwrap(),
            ..MotorControl::default()
        };
        let payload = run_default.to_payload();
        println!("length: {:2} payload: {:?}", payload.len(), payload);
        assert_eq!(payload.len(), 7);
    }
}
