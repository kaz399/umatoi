use crate::payload::ToPayload;
use crate::position::CubeLocation;

/// Movement type

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MovementType {
    #[default]
    Curve,
    CurveWithoutReverse,
    Linear,
}

impl From<MovementType> for u8 {
    fn from(movement_type: MovementType) -> u8 {
        match movement_type {
            MovementType::Curve => 0u8,
            MovementType::CurveWithoutReverse => 1u8,
            MovementType::Linear => 2u8,
        }
    }
}

impl ToPayload<Vec<u8>> for MovementType {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Speed parameter

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Speed {
    pub max: u8,
    pub speed_change_type: SpeedChangeType,
}

impl ToPayload<Vec<u8>> for Speed {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![self.max];
        payload.extend(self.speed_change_type.to_payload());

        payload
    }
}

/// Speed change type

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpeedChangeType {
    #[default]
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

impl ToPayload<Vec<u8>> for SpeedChangeType {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Rotation options on the move

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum RotationOption {
    #[default]
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

impl ToPayload<Vec<u8>> for RotationOption {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

/// Target to go

#[derive(Debug, Copy, Clone)]
pub struct TargetPosition {
    pub cube_location: CubeLocation,
    pub rotation_option: RotationOption,
}

impl Default for TargetPosition {
    fn default() -> Self {
        Self {
            cube_location: CubeLocation::default(),
            rotation_option: RotationOption::AbsoluteOptimal,
        }
    }
}

impl ToPayload<Vec<u8>> for TargetPosition {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        let rotation_option: u16 = (self.cube_location.angle & 0b0001_1111_1111_1111)
            | ((self.rotation_option as u16) << 13);
        let target_x: u16 = self.cube_location.point.x.try_into().unwrap();
        let target_y: u16 = self.cube_location.point.y.try_into().unwrap();
        payload.extend(target_x.to_le_bytes().to_vec());
        payload.extend(target_y.to_le_bytes().to_vec());
        payload.extend(rotation_option.to_le_bytes().to_vec());
        payload
    }
}

/// Write mode (MotorCommandId::MultiTargetPositions)

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriteMode {
    #[default]
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

impl ToPayload<Vec<u8>> for WriteMode {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::command::{MotorControlMultipleTargets, MotorControlTarget};
    use crate::characteristic::motor::def::Timeout;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_target1() {
        _setup();

        let st = MotorControlTarget::default();
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 13);

        let st = MotorControlTarget {
            timeout: Timeout::Second(10),
            movement_type: MovementType::Linear,
            speed: Speed {
                max: 20,
                speed_change_type: SpeedChangeType::Acceleration,
            },
            _reserved_1: 0xff,
            ..MotorControlTarget::default()
        };
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 13);
    }

    #[test]
    fn motor_target2() {
        _setup();

        let st = MotorControlMultipleTargets::default();
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 14);

        let st = MotorControlMultipleTargets {
            timeout: Timeout::default(),
            movement_type: MovementType::CurveWithoutReverse,
            speed: Speed {
                max: 100,
                speed_change_type: SpeedChangeType::AccelerationAndDeceleration,
            },
            write_mode: WriteMode::Append,
            target_list: vec![
                TargetPosition::default(),
                TargetPosition::default(),
                TargetPosition::default(),
            ],
            ..MotorControlMultipleTargets::default()
        };
        let payload = st.to_payload();
        println!("len: {:2} payload:{:?}", payload.len(), payload);
    }
}
