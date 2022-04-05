//! https://toio.github.io/toio-spec/en/docs/ble_motor

use crate::cube::connection::CoreCube;
use crate::position::MatPosition;
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MotorError {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("internal error of motor.rs")]
    FoundBug,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseType {
    SingleMotorControl,
    MultipleMotorControl,
    CubeSpeed,
}

/// Control command

#[derive(Debug, Copy, Clone, PartialEq)]
enum MotorCommand {
    Run,
    Period,
    TargetPosition,
    MultlTargetPositions,
    Acceleration,
}

impl MotorCommand {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MotorCommand::Run => Some(1u8),
            MotorCommand::Period => Some(2u8),
            MotorCommand::TargetPosition => Some(3u8),
            MotorCommand::MultlTargetPositions => Some(4u8),
            MotorCommand::Acceleration => Some(5u8),
        }
    }
}

/// Response code from cube

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseCode {
    Success,
    ErrorTimeout,
    ErrorIdMissed,
    ErrorInvalidParameter,
    ErrorInvalidCubeState,
    SuccessWithOverwrite,
    ErrorNotSupported,
    ErrorFailToAppend,
    Undefined,
}

impl ResponseCode {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            ResponseCode::Success => Some(0u8),
            ResponseCode::ErrorTimeout => Some(1u8),
            ResponseCode::ErrorIdMissed => Some(2u8),
            ResponseCode::ErrorInvalidParameter => Some(3u8),
            ResponseCode::ErrorInvalidCubeState => Some(4u8),
            ResponseCode::SuccessWithOverwrite => Some(5u8),
            ResponseCode::ErrorNotSupported => Some(6u8),
            ResponseCode::ErrorFailToAppend => Some(7u8),
            ResponseCode::Undefined => None,
        }
    }

    pub fn from(binary_code: u8) -> Self {
        match binary_code {
            0 => ResponseCode::Success,
            1 => ResponseCode::ErrorTimeout,
            2 => ResponseCode::ErrorIdMissed,
            4 => ResponseCode::ErrorInvalidParameter,
            5 => ResponseCode::ErrorInvalidCubeState,
            6 => ResponseCode::SuccessWithOverwrite,
            7 => ResponseCode::ErrorNotSupported,
            8 => ResponseCode::ErrorFailToAppend,
            _ => ResponseCode::Undefined,
        }
    }
}

/// Motor Id

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorId {
    Left,
    Right,
    Undefined,
}

impl MotorId {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MotorId::Left => Some(1u8),
            MotorId::Right => Some(2u8),
            MotorId::Undefined => None,
        }
    }

    pub fn from(binary_code: u8) -> Self {
        match binary_code {
            1 => MotorId::Left,
            2 => MotorId::Right,
            _ => MotorId::Undefined,
        }
    }
}

/// Motor direction

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Undefined,
}

impl Direction {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            Direction::Forward => Some(1u8),
            Direction::Backward => Some(2u8),
            Direction::Undefined => None,
        }
    }

    pub fn form(binary_code: u8) -> Self {
        match binary_code {
            1 => Direction::Forward,
            2 => Direction::Backward,
            _ => Direction::Undefined,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunningPeriod {
    Forever,
    Period(time::Duration),
    Undefined,
}

impl RunningPeriod {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            RunningPeriod::Forever => Some(0u8),
            RunningPeriod::Period(p) => {
                let period = p.as_millis() / 10;
                if period > u8::MAX.into() {
                    Some(u8::MAX)
                } else {
                    Some(period as u8)
                }
            }
            RunningPeriod::Undefined => None,
        }
    }

    pub fn from_millis(period: u64) -> Self {
        match period {
            0 => RunningPeriod::Forever,
            1..=255 => RunningPeriod::Period(time::Duration::from_millis(period)),
            _ => RunningPeriod::Undefined,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MotorInfo {
    pub time: time::Instant,
    pub response_type: ResponseType,
    pub id: usize,
    pub response_code: ResponseCode,
}

/// Moving parameter of a motor

#[derive(Serialize, Deserialize, Debug)]
struct MovingParameter {
    id: u8,
    direction: u8,
    speed: u8,
}

impl MovingParameter {
    pub fn new(
        motor_id: MotorId,
        value: i16,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if value > 0 && value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        if 0 > value && -value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        let id = motor_id.to_binary().unwrap();
        let direction = if value >= 0 { 0x01u8 } else { 0x02u8 };
        let speed = if value >= 0 {
            (value & 0xff) as u8
        } else {
            (-value & 0xff) as u8
        };
        Ok(Self {
            id,
            direction,
            speed,
        })
    }
}

/// Binary parameter representation of https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control

#[derive(Serialize, Deserialize, Debug)]
struct MotorControl {
    command: u8,
    left: MovingParameter,
    right: MovingParameter,
}

/// Binary parameter representation of https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration

#[derive(Serialize, Deserialize, Debug)]
struct MotorControlPeriod {
    command: u8,
    left: MovingParameter,
    right: MovingParameter,
    period: u8,
}

pub trait MotorControlData {
    fn bincode_run(
        &self,
        left: i16,
        right: i16,
        duration: RunningPeriod,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let left_param = MovingParameter::new(MotorId::Left, left)?;
        let right_param = MovingParameter::new(MotorId::Right, right)?;
        let control_data = match duration {
            RunningPeriod::Forever => bincode::serialize(&MotorControl {
                command: MotorCommand::Run.to_binary().unwrap(),
                left: left_param,
                right: right_param,
            })
            .unwrap(),
            RunningPeriod::Period(_) => bincode::serialize(&MotorControlPeriod {
                command: MotorCommand::Period.to_binary().unwrap(),
                left: left_param,
                right: right_param,
                period: duration.to_binary().unwrap(),
            })
            .unwrap(),
            RunningPeriod::Undefined => {
                return Err(Box::new(MotorError::InvalidParameter));
            }
        };
        println!("byte code: {:?}", control_data);
        Ok(())
    }

    fn bincode_run_to_target_position(
        &self,
        left: i16,
        right: i16,
        target: MatPosition,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
}

pub trait Motor: MotorControlData {
    fn run(
        &self,
        left: i16,
        right: i16,
        duration: RunningPeriod,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn run_to_target_position(
        &self,
        left: i16,
        right: i16,
        target: MatPosition,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct MotorTest {}
    impl MotorControlData for MotorTest {}

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
        let test1 = MotorTest {};

        assert!(test1.bincode_run(10, 20, RunningPeriod::Forever).is_ok());
        assert!(test1.bincode_run(-10, 20, RunningPeriod::Forever).is_ok());
        assert!(test1.bincode_run(10, -20, RunningPeriod::Forever).is_ok());
        assert!(test1
            .bincode_run(1000, -20, RunningPeriod::Forever)
            .is_err());
        assert!(test1
            .bincode_run(
                10,
                -20,
                RunningPeriod::Period(time::Duration::from_millis(200))
            )
            .is_ok());
    }
}
