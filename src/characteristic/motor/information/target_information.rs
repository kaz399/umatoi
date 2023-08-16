use crate::characteristic::motor::def::{CommandId, RequestId, ResponseCode};
use crate::payload::FromPayload;

/// Response to motor control with target specified
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#responses-to-motor-control-with-target-specified>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMotorControlTarget {
    pub request_id: RequestId,
    pub response_code: ResponseCode,
}

impl FromPayload<&[u8]> for ResponseMotorControlTarget {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 3 {
            return None;
        }
        if payload[0] == CommandId::TargetPosition.response() {
            Some(Self {
                request_id: RequestId::received(payload[1]),
                response_code: ResponseCode::from(payload[2]),
            })
        } else {
            None
        }
    }
}

/// Responses to motor control with multiple targets specified
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#responses-to-motor-control-with-multiple-targets-specified>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMotorControlMultipleTargets {
    pub request_id: RequestId,
    pub response_code: ResponseCode,
}

impl FromPayload<&[u8]> for ResponseMotorControlMultipleTargets {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.len() < 3 {
            return None;
        }
        if payload[0] == CommandId::MultiTargetPositions.response() {
            Some(Self {
                request_id: RequestId::received(payload[1]),
                response_code: ResponseCode::from(payload[2]),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::characteristic::motor::command::{MotorControlMultipleTargets, MotorControlTarget};
    use crate::characteristic::motor::def::*;
    use crate::payload::ToPayload;

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
