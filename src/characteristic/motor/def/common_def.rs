use crate::payload::ToPayload;
use once_cell::sync::OnceCell;
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
    MultiTargetPositions,
    Acceleration,
    ObtainSpeed,
}

impl From<CommandId> for u8 {
    fn from(cmd: CommandId) -> u8 {
        match cmd {
            CommandId::Run => 1u8,
            CommandId::Period => 2u8,
            CommandId::TargetPosition => 3u8,
            CommandId::MultiTargetPositions => 4u8,
            CommandId::Acceleration => 5u8,
            CommandId::ObtainSpeed => 0x60u8,
        }
    }
}

impl CommandId {
    pub fn response(self) -> u8 {
        u8::from(self) | 0x80u8
    }
}

impl ToPayload<Vec<u8>> for CommandId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Response code from cube
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResponseCode {
    Success,
    ErrorTimeout,
    ErrorIdMissed,
    ErrorInvalidParameter,
    ErrorInvalidCubeState,
    SuccessWithOverwrite,
    ErrorNotSupported,
    ErrorFailToAppend,
    UnknownError(u8),
}

impl From<ResponseCode> for u8 {
    fn from(response_code: ResponseCode) -> u8 {
        match response_code {
            ResponseCode::Success => 0u8,
            ResponseCode::ErrorTimeout => 1u8,
            ResponseCode::ErrorIdMissed => 2u8,
            ResponseCode::ErrorInvalidParameter => 3u8,
            ResponseCode::ErrorInvalidCubeState => 4u8,
            ResponseCode::SuccessWithOverwrite => 5u8,
            ResponseCode::ErrorNotSupported => 6u8,
            ResponseCode::ErrorFailToAppend => 7u8,
            ResponseCode::UnknownError(x) => x,
        }
    }
}

impl From<u8> for ResponseCode {
    fn from(binary_code: u8) -> ResponseCode {
        match binary_code {
            0 => ResponseCode::Success,
            1 => ResponseCode::ErrorTimeout,
            2 => ResponseCode::ErrorIdMissed,
            4 => ResponseCode::ErrorInvalidParameter,
            5 => ResponseCode::ErrorInvalidCubeState,
            6 => ResponseCode::SuccessWithOverwrite,
            7 => ResponseCode::ErrorNotSupported,
            8 => ResponseCode::ErrorFailToAppend,
            x => ResponseCode::UnknownError(x),
        }
    }
}

impl ToPayload<Vec<u8>> for ResponseCode {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Errors
///
/// No default.

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MotorError {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("internal error of motor.rs")]
    FoundBug,
}

/// Request ID
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestId {
    pub id: u8,
}

impl From<RequestId> for u8 {
    fn from(value: RequestId) -> Self {
        value.id
    }
}

impl ToPayload<Vec<u8>> for RequestId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Request ID counter (global scope)
static REQUEST_ID: OnceCell<Mutex<u8>> = OnceCell::new();

impl Default for RequestId {
    fn default() -> Self {
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

impl RequestId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn received(id: u8) -> Self {
        Self { id }
    }
}

/// Timeout

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl ToPayload<Vec<u8>> for Timeout {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Period for running

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

    pub fn from_millis(period: usize) -> Self {
        let period = period / 10;
        match period {
            1..=255 => Period {
                period: period as u8,
            },
            _ => Period { period: 0 },
        }
    }
}

impl ToPayload<Vec<u8>> for Period {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.period];
        payload
    }
}

/// Motor Id
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl ToPayload<Vec<u8>> for MotorId {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Motor direction
///
/// No default.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl ToPayload<Vec<u8>> for MotorDirection {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Cube speed

#[derive(Debug, Copy, Clone)]
pub struct Velocity {
    pub direction: MotorDirection,
    pub speed: u8,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            direction: MotorDirection::Forward,
            speed: 0,
        }
    }
}

impl Velocity {
    pub fn set_value(value: i16) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if value > 0 && value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        if 0 > value && -value > u8::MAX.into() {
            return Err(Box::new(MotorError::InvalidParameter));
        }
        let direction = if value >= 0 {
            MotorDirection::Forward
        } else {
            MotorDirection::Backward
        };
        let speed = if value >= 0 {
            (value & 0xff) as u8
        } else {
            (-value & 0xff) as u8
        };
        Ok(Self { direction, speed })
    }
}

impl ToPayload<Vec<u8>> for Velocity {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.direction.to_payload());
        payload.push(self.speed);
        payload
    }
}

/// Motor drive parameter

#[derive(Debug, Copy, Clone)]
pub struct MotorDriveParameter {
    pub id: MotorId,
    pub velocity: Velocity,
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

impl ToPayload<Vec<u8>> for MotorDriveParameter {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.id.to_payload());
        payload.extend(self.velocity.to_payload());
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
    fn motor_def_request_id() {
        _setup();

        for ct in 0usize..=300usize {
            let req = RequestId::new();
            assert_eq!(req.id as usize, ct % (1 + u8::MAX as usize));
        }
    }
}
