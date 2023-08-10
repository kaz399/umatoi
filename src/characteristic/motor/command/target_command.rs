use crate::characteristic::motor::def::target_def::{
    MovementType, Speed, TargetPosition, WriteMode, _MotorControlMultipleTargetsHeader,
    _MotorControlTargetHeader,
};
use crate::characteristic::motor::def::{CommandId, RequestId, Timeout};
use crate::payload::ToPayload;

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-target-specified>

#[derive(Debug, Copy, Clone)]
pub struct MotorControlTarget {
    pub command: CommandId,
    pub id: RequestId,
    pub timeout: Timeout,
    pub movement_type: MovementType,
    pub speed: Speed,
    pub _reserved_1: u8,
    pub target: TargetPosition,
}

impl Default for MotorControlTarget {
    fn default() -> Self {
        Self {
            command: CommandId::TargetPosition,
            id: RequestId::new(),
            timeout: Timeout::default(),
            movement_type: MovementType::default(),
            speed: Speed::default(),
            _reserved_1: 0,
            target: TargetPosition::default(),
        }
    }
}

impl MotorControlTarget {
    fn header(&self) -> _MotorControlTargetHeader {
        _MotorControlTargetHeader {
            command: self.command,
            id: self.id,
            timeout: self.timeout,
            movement_type: self.movement_type,
            speed: self.speed,
            _reserved_1: self._reserved_1,
        }
    }
}

impl ToPayload<Vec<u8>> for MotorControlTarget {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.header().to_payload();
        payload.extend(&self.target.to_payload());
        payload
    }
}

/// Byte-string representation of <https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified>

#[derive(Debug, Clone)]
pub struct MotorControlMultipleTargets {
    pub command: CommandId,
    pub id: RequestId,
    pub timeout: Timeout,
    pub movement_type: MovementType,
    pub speed: Speed,
    pub _reserved_1: u8,
    pub write_mode: WriteMode,
    pub target_list: Vec<TargetPosition>,
}

impl Default for MotorControlMultipleTargets {
    fn default() -> Self {
        Self {
            command: CommandId::MultiTargetPositions,
            id: RequestId::new(),
            timeout: Timeout::default(),
            movement_type: MovementType::default(),
            speed: Speed::default(),
            _reserved_1: 0,
            write_mode: WriteMode::default(),
            target_list: vec![TargetPosition::default()],
        }
    }
}

impl MotorControlMultipleTargets {
    fn header(&self) -> _MotorControlMultipleTargetsHeader {
        _MotorControlMultipleTargetsHeader {
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

impl ToPayload<Vec<u8>> for MotorControlMultipleTargets {
    fn to_payload(self) -> Vec<u8> {
        let mut payload = self.header().to_payload();
        for target in &self.target_list {
            payload.extend(&target.to_payload());
        }
        payload
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::target_def::{
        MovementType, Speed, SpeedChangeType, TargetPosition, WriteMode,
    };

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
