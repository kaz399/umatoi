//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

use crate::position::CubeLocation;
use log::debug;
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;
use std::error::Error;
use std::time;
use thiserror::Error;
use to_vec::ToVec;

/// Control command

#[derive(Debug, Copy, Clone, PartialEq)]
enum MotorCommand {
    Run,
    Period,
    TargetPosition,
    MultlTargetPositions,
    Acceleration,
}

impl From<MotorCommand> for u8 {
    fn from(cmd: MotorCommand) -> u8 {
        match cmd {
            MotorCommand::Run => 1u8,
            MotorCommand::Period => 2u8,
            MotorCommand::TargetPosition => 3u8,
            MotorCommand::MultlTargetPositions => 4u8,
            MotorCommand::Acceleration => 5u8,
        }
    }
}

impl Serialize for MotorCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
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
    UnknownResponse(u8),
}

impl From<ResponseType> for u8 {
    fn from(response_type: ResponseType) -> u8 {
        match response_type {
            ResponseType::SingleMotorControl => 0x83u8,
            ResponseType::MultipleMotorControl => 0x84u8,
            ResponseType::CubeSpeed => 0xe0u8,
            ResponseType::UnknownResponse(x) => x,
        }
    }
}

impl From<u8> for ResponseType {
    fn from(num: u8) -> ResponseType {
        match num {
            0x83 => ResponseType::SingleMotorControl,
            0x84 => ResponseType::MultipleMotorControl,
            0xe0 => ResponseType::CubeSpeed,
            x => ResponseType::UnknownResponse(x),
        }
    }
}

impl Serialize for ResponseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
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

impl Serialize for ResponseCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Response/Notify data from cube

#[derive(Debug)]
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

/// Period for running

#[derive(Serialize, Debug)]
pub struct RunningPeriod {
    period: u8,
}

impl From<RunningPeriod> for u8 {
    fn from(running_period: RunningPeriod) -> u8 {
        running_period.period
    }
}

impl RunningPeriod {
    pub fn forever() -> Self {
        Self { period: 0 }
    }

    pub fn from_millis(perio: u64) -> Self {
        let period = perio / 10;
        match period {
            1..=255 => RunningPeriod {
                period: period as u8,
            },
            _ => RunningPeriod { period: 0 },
        }
    }
}

/// Movement type

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovementType {
    Curve,
    CuverWithoutReverse,
    Liner,
}

impl From<MovementType> for u8 {
    fn from(movement_type: MovementType) -> u8 {
        match movement_type {
            MovementType::Curve => 0u8,
            MovementType::CuverWithoutReverse => 1u8,
            MovementType::Liner => 2u8,
        }
    }
}

impl Serialize for MovementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Speed change type

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpeedChangeType {
    Constant,
    Acceleration,
    Deceleration,
    AccelerationAndDeceleration,
}

impl From<SpeedChangeType> for u8 {
    fn from(speed_change_type: SpeedChangeType) -> u8 {
        match speed_change_type {
            SpeedChangeType::Constant => 0u8,
            SpeedChangeType::Acceleration => 1u8,
            SpeedChangeType::Deceleration => 2u8,
            SpeedChangeType::AccelerationAndDeceleration => 3u8,
        }
    }
}

impl Serialize for SpeedChangeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Rotation options on the move

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationOption {
    AbsoluteOptimal,
    AbsolutePositive,
    AbsoluteNegative,
    RelativePositive,
    RelativeNegative,
    WithoutRotation,
    SameAsAtWriting,
}

impl From<RotationOption> for u8 {
    fn from(rotation_option: RotationOption) -> u8 {
        match rotation_option {
            RotationOption::AbsoluteOptimal => 0u8,
            RotationOption::AbsolutePositive => 1u8,
            RotationOption::AbsoluteNegative => 2u8,
            RotationOption::RelativePositive => 3u8,
            RotationOption::RelativeNegative => 4u8,
            RotationOption::WithoutRotation => 5u8,
            RotationOption::SameAsAtWriting => 6u8,
        }
    }
}

impl Serialize for RotationOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Move target

#[derive(Debug, Copy, Clone)]
pub struct MoveTarget {
    cube_location: CubeLocation,
    rotation_option: RotationOption,
}

impl Default for MoveTarget {
    fn default() -> Self {
        Self {
            cube_location: CubeLocation::default(),
            rotation_option: RotationOption::AbsoluteOptimal,
        }
    }
}
impl ToVec<u8> for MoveTarget {
    fn to_vec(self) -> Vec<u8> {
        let rotation_option: u16 = (self.rotation_option as u16) << 13;
        let combined_data: [u16; 3] = [
            self.cube_location.point.x,
            self.cube_location.point.y,
            (self.cube_location.angle & 0b0001_1111_1111_1111) | rotation_option,
        ];
        bincode::serialize(&combined_data).unwrap()
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

impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Write mode (MotorCommand::MultlTargetPositions)

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WriteMode {
    Overwrite,
    Append,
}

impl From<WriteMode> for u8 {
    fn from(write_mode: WriteMode) -> u8 {
        match write_mode {
            WriteMode::Overwrite => 0u8,
            WriteMode::Append => 1u8,
        }
    }
}

impl Serialize for WriteMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Rotation direction (MotorCommand::Acceleration)

#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Moving direction (MotorCommand::Acceleration)

#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Priority (MotorCommand::Acceleration)

#[derive(Debug, Copy, Clone, PartialEq)]
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
        let id = motor_id.into();
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

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control>

#[derive(Serialize, Debug)]
struct MotorControlRun {
    command: MotorCommand,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
}

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration>

#[derive(Serialize, Debug)]
struct MotorControlPeriod {
    command: MotorCommand,
    left: MotorDriveParameter,
    right: MotorDriveParameter,
    period: RunningPeriod,
}

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-target-specified>

#[derive(Serialize, Debug)]
struct MotorControlTargetPosition {
    command: MotorCommand,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    cube_location: CubeLocation,
}

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified>

#[derive(Debug)]
pub struct MotorControlMultiTargetPositions {
    command: MotorCommand,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    write_mode: WriteMode,
    target_list: Vec<MoveTarget>,
}

#[derive(Serialize, Debug)]
pub struct MotorControlMultiTargetPositionsHeader {
    command: MotorCommand,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    write_mode: WriteMode,
}

impl ToVec<u8> for MotorControlMultiTargetPositionsHeader {
    fn to_vec(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl MotorControlMultiTargetPositions {
    fn header(&self) -> MotorControlMultiTargetPositionsHeader {
        MotorControlMultiTargetPositionsHeader {
            command: self.command,
            id: self.id,
            timeout: self.timeout,
            movement_type: self.movement_type,
            max_speed: self.max_speed,
            speed_change_type: self.speed_change_type,
            _reserved_1: self._reserved_1,
            write_mode: self.write_mode,
        }
    }
}

impl Serialize for MotorControlMultiTargetPositions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vertorized_self = self.header().to_vec();
        for target in &self.target_list {
            vertorized_self.extend(&target.to_vec());
        }
        let mut seq = serializer.serialize_seq(Some(vertorized_self.len()))?;
        for byte_data in vertorized_self {
            seq.serialize_element(&byte_data)?;
        }
        seq.end()
    }
}

/// Binary parameter representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-acceleration-specified>

#[derive(Serialize, Debug)]
struct MotorControlAccleration {
    command: MotorCommand,
    id: u8,
    translational_speed: u8,
    acceleration: u8,
    rotation_velocity: u16,
    rotation_direction: RotationDirection,
    moving_direction: MovingDirection,
    priority: Priority,
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
        let control_data = match duration.period {
            0 => bincode::serialize(&MotorControlRun {
                command: MotorCommand::Run,
                left: left_param,
                right: right_param,
            })
            .unwrap(),
            _ => bincode::serialize(&MotorControlPeriod {
                command: MotorCommand::Period,
                left: left_param,
                right: right_param,
                period: duration,
            })
            .unwrap(),
        };
        debug!("len: {:2}, data: {:?}", control_data.len(), control_data);
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
            command: MotorCommand::TargetPosition,
            id: 0xbb,
            timeout: timeout.into(),
            movement_type,
            max_speed,
            speed_change_type,
            _reserved_1: 0xee,
            cube_location,
        })
        .unwrap();
        debug!("len: {:2}, data: {:?}", control_data.len(), control_data);
        Ok(control_data)
    }

    fn encode_run_to_multi_target_positions(
        &self,
        timeout: Timeout,
        movement_type: MovementType,
        max_speed: u8,
        speed_change_type: SpeedChangeType,
        write_mode: WriteMode,
        target_list: Vec<MoveTarget>,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        // create header part
        let control_data = bincode::serialize(&MotorControlMultiTargetPositions {
            command: MotorCommand::MultlTargetPositions,
            id: 0xcc,
            timeout: timeout.into(),
            movement_type,
            max_speed,
            speed_change_type,
            write_mode,
            _reserved_1: 0xff,
            target_list,
        })
        .unwrap();
        debug!("len: {:2}, data: {:?}", control_data.len(), control_data);
        Ok(control_data)
    }

    fn encode_run_with_acceleration(
        &self,
        translational_speed: u8,
        acceleration: u8,
        rotation_velocity: u16,
        rotation_direction: RotationDirection,
        moving_direction: MovingDirection,
        priority: Priority,
        period: RunningPeriod,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let control_data = bincode::serialize(&MotorControlAccleration {
            command: MotorCommand::Acceleration,
            id: 0xaa,
            translational_speed,
            acceleration,
            rotation_velocity,
            rotation_direction,
            moving_direction,
            priority,
            period,
        })
        .unwrap();
        debug!("len: {:2}, data: {:?}", control_data.len(), control_data);
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

        assert!(test1.encode_run(10, 20, RunningPeriod::forever()).is_ok());
        assert!(test1.encode_run(-10, 20, RunningPeriod::forever()).is_ok());
        assert!(test1.encode_run(10, -20, RunningPeriod::forever()).is_ok());
        assert!(test1
            .encode_run(1000, -20, RunningPeriod::forever())
            .is_err());
        assert!(test1
            .encode_run(10, -20, RunningPeriod::from_millis(200))
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
                vec![MoveTarget {
                    cube_location: CubeLocation::default(),
                    rotation_option: RotationOption::SameAsAtWriting,
                },]
            )
            .is_ok());
    }
}
