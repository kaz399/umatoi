//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

use crate::position::{CubeLocation, Point};
use serde::Serialize;
use std::error::Error;
use std::time;
use thiserror::Error;

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

/// Errors

#[derive(Error, Debug, PartialEq)]
pub enum MotorError {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("internal error of motor.rs")]
    FoundBug,
}

/// Response type

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseType {
    SingleMotorControl,
    MultipleMotorControl,
    CubeSpeed,
    Undefined,
}

impl ResponseType {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            ResponseType::SingleMotorControl => Some(0x83u8),
            ResponseType::MultipleMotorControl => Some(0x84u8),
            ResponseType::CubeSpeed => Some(0xe0u8),
            ResponseType::Undefined => None,
        }
    }

    pub fn from(binary_code: u8) -> Self {
        match binary_code {
            0x83 => ResponseType::SingleMotorControl,
            0x84 => ResponseType::MultipleMotorControl,
            0xe0 => ResponseType::CubeSpeed,
            _ => ResponseType::Undefined,
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

/// Response/Notify data from cube

#[derive(Debug, Copy, Clone)]
pub struct MotorInfo {
    pub time: time::Instant,
    pub response_type: ResponseType,
    pub id: usize,
    pub response_code: ResponseCode,
}

/// Motor Id

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorId {
    Left,
    Right,
}

impl MotorId {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MotorId::Left => Some(1u8),
            MotorId::Right => Some(2u8),
        }
    }
}

/// Motor direction

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotorDirection {
    Forward,
    Backward,
}

impl MotorDirection {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MotorDirection::Forward => Some(1u8),
            MotorDirection::Backward => Some(2u8),
        }
    }
}

/// Period for running

#[derive(Serialize, Debug)]
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

/// Movement type

#[derive(Serialize, Debug)]
pub enum MovementType {
    Curve,
    CuverWithoutReverse,
    Liner,
}

impl MovementType {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MovementType::Curve => Some(0u8),
            MovementType::CuverWithoutReverse => Some(1u8),
            MovementType::Liner => Some(2u8),
        }
    }
}

/// Speed change type

#[derive(Serialize, Debug)]
pub enum SpeedChangeType {
    Constant,
    Acceleration,
    Deceleration,
    AccelerationAndDeceleration,
}

impl SpeedChangeType {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            SpeedChangeType::Constant => Some(0u8),
            SpeedChangeType::Acceleration => Some(1u8),
            SpeedChangeType::Deceleration => Some(2u8),
            SpeedChangeType::AccelerationAndDeceleration => Some(3u8),
        }
    }
}

/// Rotation options on the move

#[derive(Serialize, Debug)]
pub enum RotationOption {
    AbsoluteOptimal,
    AbsolutePositive,
    AbsoluteNegative,
    RelativePositive,
    RelativeNegative,
    WithoutRotation,
    SameAsAtWriting,
}

impl RotationOption {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            RotationOption::AbsoluteOptimal => Some(0u8),
            RotationOption::AbsolutePositive => Some(1u8),
            RotationOption::AbsoluteNegative => Some(2u8),
            RotationOption::RelativePositive => Some(3u8),
            RotationOption::RelativeNegative => Some(4u8),
            RotationOption::WithoutRotation => Some(5u8),
            RotationOption::SameAsAtWriting => Some(6u8),
        }
    }
}

/// Timeout

#[derive(Serialize, Debug)]
pub enum Timeout {
    Second(u8),
}

impl Timeout {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            Timeout::Second(t) => Some(t),
        }
    }
}

/// Write mode (MotorCommand::MultlTargetPositions)

#[derive(Serialize, Debug)]
pub enum WriteMode {
    Overwrite,
    Append,
}

impl WriteMode {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            WriteMode::Overwrite => Some(0u8),
            WriteMode::Append => Some(1u8),
        }
    }
}

/// Rotation direction (MotorCommand::Acceleration)

#[derive(Serialize, Debug)]
pub enum RotationDirection {
    Positive,
    Negative,
}

impl RotationDirection {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            RotationDirection::Positive => Some(0u8),
            RotationDirection::Negative => Some(1u8),
        }
    }
}

/// Moving direction (MotorCommand::Acceleration)

#[derive(Serialize, Debug)]
pub enum MovingDirection {
    Forward,
    Backward,
}

impl MovingDirection {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            MovingDirection::Forward => Some(0u8),
            MovingDirection::Backward => Some(1u8),
        }
    }
}

/// Priority (MotorCommand::Acceleration)

#[derive(Serialize, Debug)]
pub enum Priority {
    TranslationalSpeed,
    RotationVelocity,
}

impl Priority {
    pub fn to_binary(self) -> Option<u8> {
        match self {
            Priority::TranslationalSpeed => Some(0u8),
            Priority::RotationVelocity => Some(1u8),
        }
    }
}

/// Motor drive parameter

#[derive(Serialize, Debug)]
struct MotorDriveParameter {
    id: u8,
    direction: u8,
    speed: u8,
}

impl MotorDriveParameter {
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

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control>

#[derive(Serialize, Debug)]
struct MotorControlRun {
    command: u8,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
}

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration>

#[derive(Serialize, Debug)]
struct MotorControlPeriod {
    command: u8,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
    period: u8,
}

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-target-specified>

#[derive(Serialize, Debug)]
struct MotorControlTargetPosition {
    command: u8,
    id: u8,
    timeout: u8,
    movement_type: u8,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    cube_location: CubeLocation,
}

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified>

#[derive(Serialize, Debug)]
struct MotorControlMultiTargetPositionsHeader {
    command: u8,
    id: u8,
    timeout: u8,
    movement_type: u8,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    write_mode: WriteMode,
}

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-acceleration-specified>

#[derive(Serialize, Debug)]
struct MotorControlAccleration {
    command: u8,
    id: u8,
    translational_speed: u8,
    acceleration: u8,
    rotation_velocity: u16,
    rotation_direction: u8,
    moving_direction: u8,
    priority: u8,
    period: RunningPeriod,
}

pub trait MotorBleData {
    fn encode_run(
        &self,
        left: i16,
        right: i16,
        duration: RunningPeriod,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let left_param = MotorDriveParameter::new(MotorId::Left, left)?;
        let right_param = MotorDriveParameter::new(MotorId::Right, right)?;
        let control_data = match duration {
            RunningPeriod::Forever => bincode::serialize(&MotorControlRun {
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
        Ok(control_data)
    }

    fn encode_run_to_target_position(
        &self,
        timeout: Timeout,
        movement_type: MovementType,
        max_speed: u8,
        speed_change_type: SpeedChangeType,
        cube_location: CubeLocation,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let control_data = bincode::serialize(&MotorControlTargetPosition {
            command: MotorCommand::TargetPosition.to_binary().unwrap(),
            id: 0xbb,
            timeout: timeout.to_binary().unwrap(),
            movement_type: movement_type.to_binary().unwrap(),
            max_speed,
            speed_change_type,
            _reserved_1: 0xee,
            cube_location,
        })
        .unwrap();
        println!("byte code: {:?}", control_data);
        Ok(control_data)
    }

    fn encode_run_to_multi_target_positions(
        &self,
        timeout: Timeout,
        movement_type: MovementType,
        max_speed: u8,
        speed_change_type: SpeedChangeType,
        write_mode: WriteMode,
        cube_location: Vec<CubeLocation>,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        // create header part
        let mut control_data = bincode::serialize(&MotorControlMultiTargetPositionsHeader {
            command: MotorCommand::MultlTargetPositions.to_binary().unwrap(),
            id: 0xcc,
            timeout: timeout.to_binary().unwrap(),
            movement_type: movement_type.to_binary().unwrap(),
            max_speed,
            speed_change_type,
            write_mode,
            _reserved_1: 0xff,
        })
        .unwrap();
        // add body
        for location in cube_location.iter() {
            control_data.extend(bincode::serialize(location).unwrap());
        }
        println!("byte code: {:?}", control_data);
        Ok(control_data)
    }
}

pub trait Motor: MotorBleData {
    fn run(
        &self,
        left: i16,
        right: i16,
        duration: RunningPeriod,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    fn encode_run_to_target_position(
        &self,
        timeout: Timeout,
        movement_type: MovementType,
        max_speed: u8,
        speed_change_type: SpeedChangeType,
        cube_location: CubeLocation,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct MotorTest {}
    impl MotorBleData for MotorTest {}

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
        let test1 = MotorTest {};

        assert!(test1.encode_run(10, 20, RunningPeriod::Forever).is_ok());
        assert!(test1.encode_run(-10, 20, RunningPeriod::Forever).is_ok());
        assert!(test1.encode_run(10, -20, RunningPeriod::Forever).is_ok());
        assert!(test1.encode_run(1000, -20, RunningPeriod::Forever).is_err());
        assert!(test1
            .encode_run(
                10,
                -20,
                RunningPeriod::Period(time::Duration::from_millis(200))
            )
            .is_ok());
        assert!(test1
            .encode_run_to_target_position(
                Timeout::Second(10),
                MovementType::Curve,
                32,
                SpeedChangeType::Acceleration,
                CubeLocation::default(),
            )
            .is_ok());
        assert!(test1
            .encode_run_to_multi_target_positions(
                Timeout::Second(10),
                MovementType::Curve,
                32,
                SpeedChangeType::Acceleration,
                WriteMode::Append,
                vec![
                    CubeLocation::default(),
                    CubeLocation::default(),
                    CubeLocation::default()
                ],
            )
            .is_ok());
    }
}
