use super::def::{CommandId, RequestId, Timeout};
use crate::payload::ToPayload;
use crate::position::CubeLocation;
use serde::ser::Serializer;
use serde::Serialize;

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-target-specified>

#[derive(Debug, Copy, Clone)]
struct MotorControlWithTargetSpecified {
    command: CommandId,
    id: RequestId,
    timeout: Timeout,
    movement_type: MovementType,
    speed: Speed,
    _reserved_1: u8,
    target: Target,
}

impl ToPayload<u8> for MotorControlWithTargetSpecified {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.header().to_payload();
        payload.extend(&self.target.to_payload());
        payload
    }
}

impl Default for MotorControlWithTargetSpecified {
    fn default() -> Self {
        Self {
            command: CommandId::TargetPosition,
            id: RequestId::get(),
            timeout: Timeout::default(),
            movement_type: MovementType::default(),
            speed: Speed::default(),
            _reserved_1: 0,
            target: Target::default(),
        }
    }
}

impl MotorControlWithTargetSpecified {
    fn header(&self) -> MotorControlWithTargetSpecifiedHeader {
        MotorControlWithTargetSpecifiedHeader {
            command: self.command,
            id: self.id,
            timeout: self.timeout,
            movement_type: self.movement_type,
            speed: self.speed,
            _reserved_1:  self._reserved_1,
        }
    }
}

/// Bite-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified>

#[derive(Debug, Clone)]
pub struct MotorControlWithMultipleTargetsSpecified {
    command: CommandId,
    id: RequestId,
    timeout: Timeout,
    movement_type: MovementType,
    speed: Speed,
    _reserved_1: u8,
    write_mode: WriteMode,
    target_list: Vec<Target>,
}

impl ToPayload<u8> for MotorControlWithMultipleTargetsSpecified {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.header().to_payload();
        for target in &self.target_list {
            payload.extend(&target.to_payload());
        }
        payload
    }
}

impl Default for MotorControlWithMultipleTargetsSpecified {
    fn default() -> Self {
        Self {
            command: CommandId::MultlTargetPositions,
            id: RequestId::get(),
            timeout: Timeout::default(),
            movement_type: MovementType::default(),
            speed: Speed::default(),
            _reserved_1: 0,
            write_mode: WriteMode::default(),
            target_list: vec!(Target::default()),
        }
    }
}

impl MotorControlWithMultipleTargetsSpecified {
    fn header(&self) -> MotorControlWithMultipleTargetsSpecifiedHeader {
        MotorControlWithMultipleTargetsSpecifiedHeader {
            command: self.command,
            id: self.id,
            timeout: self.timeout,
            movement_type: self.movement_type,
            speed: self.speed,
            _reserved_1: self._reserved_1,
            write_mode: self.write_mode,
        }
    }
}

/// Header part of `MotorControlWithTargetSpecified`
///
/// This struct is NOT public
#[derive(Serialize, Debug)]
struct MotorControlWithTargetSpecifiedHeader {
    command: CommandId,
    id: RequestId,
    timeout: Timeout,
    movement_type: MovementType,
    speed: Speed,
    _reserved_1: u8,
}

impl ToPayload<u8> for MotorControlWithTargetSpecifiedHeader {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}


/// Header part of `MotorControlWithMultipleTargetsSpecified`
///
/// This struct is NOT public.
#[derive(Serialize, Debug)]
struct MotorControlWithMultipleTargetsSpecifiedHeader {
    command: CommandId,
    id: RequestId,
    timeout: Timeout,
    movement_type: MovementType,
    speed: Speed,
    _reserved_1: u8,
    write_mode: WriteMode,
}

impl ToPayload<u8> for MotorControlWithMultipleTargetsSpecifiedHeader {
    fn to_payload(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
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

impl Default for MovementType {
    fn default() -> Self {
        MovementType::Curve
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

/// Speed parameter

#[derive(Serialize, Default, Debug, Copy, Clone, PartialEq)]
pub struct Speed {
    max_speed: u8,
    speed_change_type: SpeedChangeType,
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

impl Default for SpeedChangeType {
    fn default() -> Self {
        SpeedChangeType::Constant
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

impl Default for RotationOption {
    fn default() -> Self {
        RotationOption::AbsoluteOptimal
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

impl Default for WriteMode {
    fn default() -> Self {
        WriteMode::Overwrite
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
    fn motor_target1() {
        _setup();

        let st = MotorControlWithTargetSpecified::default();
        let payload = st.to_payload();
        println!("{:?}", payload);
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len() , 13);

        let st = MotorControlWithTargetSpecified {
            timeout: Timeout::Second(10),
            movement_type: MovementType::Liner,
            speed: Speed {
                max_speed: 20,
                speed_change_type: SpeedChangeType::Acceleration,
            },
            _reserved_1: 0xff,
            ..MotorControlWithTargetSpecified::default()
        };
        let payload = st.to_payload();
        println!("{:?}", payload);
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len() , 13);
    }

    #[test]
    fn motor_target2() {
        _setup();

        let st = MotorControlWithMultipleTargetsSpecified::default();
        let payload = st.to_payload();
        println!("{:?}", payload);
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len() , 14);

        let st = MotorControlWithMultipleTargetsSpecified {
            timeout: Timeout::default(),
            movement_type: MovementType::CuverWithoutReverse,
            speed: Speed { max_speed: 100, speed_change_type: SpeedChangeType::AccelerationAndDeceleration },
            write_mode: WriteMode::Append,
            target_list: vec![Target::default(), Target::default(), Target::default()],
            ..MotorControlWithMultipleTargetsSpecified::default()
        };
        let payload = st.to_payload();
        println!("{:?}", payload);
        println!("len: {:2} payload:{:?}", payload.len(), payload);
    }
}
