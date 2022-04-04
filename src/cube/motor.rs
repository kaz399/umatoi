//! https://toio.github.io/toio-spec/en/docs/ble_motor

use crate::position::MatPosition;
use std::time;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseType {
    SingleMotorControl,
    MultipleMotorControl,
    CubeSpeed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseCode {
    Success = 0,
    ErrorTimeout = 1,
    ErrorIdMissed = 2,
    ErrorInvalidParameter = 3,
    ErrorInvalidCubeState = 4,
    SuccessWithOverwrite = 5,
    ErrorNotSupported = 6,
    ErrorFailToAppend = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorDuration {
    Forever,
    Period(time::Duration),
}

#[derive(Debug, Copy, Clone)]
pub struct MotorInfo {
    pub time: time::Instant,
    pub response_type: ResponseType,
    pub id: usize,
    pub response_code: ResponseCode,
}

pub trait Motor {
    fn run(&self, left: i16, right: i16, duration: MotorDuration);
    fn run_to_target_position(&self, left: i16, right: i16, target: MatPosition);
}
