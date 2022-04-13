use once_cell::sync::OnceCell;
use serde::ser::Serializer;
use serde::Serialize;
use std::error::Error;
use std::sync::Mutex;
use thiserror::Error;

/// Command
///
/// No default.

#[derive(Debug, Copy, Clone)]
pub enum CommandId {
    Run,
    Period,
    TargetPosition,
    MultlTargetPositions,
    Acceleration,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::Run => 1u8,
            CommandId::Period => 2u8,
            CommandId::TargetPosition => 3u8,
            CommandId::MultlTargetPositions => 4u8,
            CommandId::Acceleration => 5u8,
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

/// Errors
///
/// No default.

#[derive(Error, Debug, PartialEq)]
pub enum MotorError {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("internal error of motor.rs")]
    FoundBug,
}

/// Request ID
///
/// No default.

#[derive(Debug, Serialize, Copy, Clone, PartialEq)]
pub struct RequestId {
    id: u8,
}

/// Request ID counter (global scope)
static REQUEST_ID: OnceCell<Mutex<u8>> = OnceCell::new();

impl RequestId {
    pub fn get() -> Self {
        let mut request_id = REQUEST_ID.get_or_init(|| Mutex::new(0u8)).lock().unwrap();
        let id = *request_id;
        if *request_id == u8::MAX {
            *request_id = 0;
        } else {
            *request_id += 1;
        }
        Self { id }
    }
}

/// Timeout

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Timeout {
    Second(u8),
}

impl From<Timeout> for u8 {
    fn from(timeout: Timeout) -> u8 {
        match timeout {
            Timeout::Second(t) => t,
        }
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout::Second(0)
    }
}

impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Period for running

#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct Period {
    period: u8,
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

/// Motor Id
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorId {
    Left,
    Right,
}

impl From<MotorId> for u8 {
    fn from(motor_id: MotorId) -> u8 {
        match motor_id {
            MotorId::Left => 1u8,
            MotorId::Right => 2u8,
        }
    }
}

impl Serialize for MotorId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Motor direction
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorDirection {
    Forward,
    Backward,
}

impl From<MotorDirection> for u8 {
    fn from(moving_direction: MotorDirection) -> u8 {
        match moving_direction {
            MotorDirection::Forward => 1u8,
            MotorDirection::Backward => 2u8,
        }
    }
}

impl Serialize for MotorDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Cube speed

#[derive(Serialize, Debug, Copy, Clone)]
pub struct Velocity {
    direction: u8,
    speed: u8,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            direction: 1,
            speed: 0,
        }
    }
}

impl Velocity {
    pub fn new(value: i16) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if value > 0 && value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        if 0 > value && -value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        let direction = if value >= 0 { 0x01u8 } else { 0x02u8 };
        let speed = if value >= 0 {
            (value & 0xff) as u8
        } else {
            (-value & 0xff) as u8
        };
        Ok(Self { direction, speed })
    }
}

/// Motor drive parameter

#[derive(Serialize, Debug, Copy, Clone)]
pub struct MotorDriveParameter {
    id: MotorId,
    velocity: Velocity,
}

impl Default for MotorDriveParameter {
    fn default() -> Self {
        Self {
            id: MotorId::Left,
            velocity: Velocity::default(),
        }
    }
}

impl MotorDriveParameter {
    pub fn new(
        id: MotorId,
        velocity: Velocity,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(Self { id, velocity })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_def_request_id() {
        _setup();

        for ct in 0usize..=300usize {
            let req = RequestId::get();
            println!("request id: {}", req.id);
            assert_eq!(req.id as usize, ct % (1 + u8::MAX as usize));
        }
    }
}
