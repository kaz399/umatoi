use crate::payload::ToPayload;
use crate::position::CubeLocation;
use serde::ser::Serializer;
use serde::Serialize;

use super::def::CommandId;

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-target-specified>

#[derive(Serialize, Debug, Copy, Clone)]
struct MotorControlTargetPosition {
    command: CommandId,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    cube_location: CubeLocation,
}

impl ToPayload<u8> for MotorControlTargetPosition {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified>

#[derive(Debug, Clone)]
pub struct MotorControlMultiTargetPositions {
    command: CommandId,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    write_mode: WriteMode,
    target_list: Vec<Target>,
}

#[derive(Serialize, Debug)]
pub struct MotorControlMultiTargetPositionsHeader {
    command: CommandId,
    id: u8,
    timeout: u8,
    movement_type: MovementType,
    max_speed: u8,
    speed_change_type: SpeedChangeType,
    _reserved_1: u8,
    write_mode: WriteMode,
}

impl ToPayload<u8> for MotorControlMultiTargetPositionsHeader {
    fn to_payload(self) -> Vec<u8> {
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

impl ToPayload<u8> for MotorControlMultiTargetPositions {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.header().to_payload();
        for target in &self.target_list {
            payload.extend(&target.to_payload());
        }
        payload
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

/// Target to go

#[derive(Debug, Copy, Clone)]
pub struct Target {
    cube_location: CubeLocation,
    rotation_option: RotationOption,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            cube_location: CubeLocation::default(),
            rotation_option: RotationOption::AbsoluteOptimal,
        }
    }
}

impl ToPayload<u8> for Target {
    fn to_payload(self) -> Vec<u8> {
        let rotation_option: u16 = (self.rotation_option as u16) << 13;
        let combined_data: [u16; 3] = [
            self.cube_location.point.x,
            self.cube_location.point.y,
            (self.cube_location.angle & 0b0001_1111_1111_1111) | rotation_option,
        ];
        bincode::serialize(&combined_data).unwrap()
    }
}

/// Write mode (MotorCommandId::MultlTargetPositions)

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

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
    }
}
