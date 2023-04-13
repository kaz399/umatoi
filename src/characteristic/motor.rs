//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::device_interface::CubeInterface;
use anyhow::Result;
pub mod acceleration;
pub mod control;
pub mod def;
pub mod speed;
pub mod target;

pub async fn write(interface: &dyn CubeInterface, bytes: &[u8]) -> Result<bool> {
    interface.write(CoreCubeUuid::MotorCtrl.into(), bytes).await
}


use def::CommandId;
use speed::ResponseMotorSpeed;
use target::ResponseMotorControlMultipleTargets;
use target::ResponseMotorControlTarget;
use crate::payload::ToPayload;

/// Motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MotorResponse {
    MotorControlTarget(ResponseMotorControlTarget),
    MotorControlMultipleTargets(ResponseMotorControlMultipleTargets),
    MotorSpeed(ResponseMotorSpeed),
}

impl MotorResponse {
    pub fn new(byte_data: &[u8]) -> Option<MotorResponse> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = ResponseMotorControlTarget::new(byte_data) {
            return Some(MotorResponse::MotorControlTarget(response_data));
        }
        if let Some(response_data) = ResponseMotorControlMultipleTargets::new(byte_data) {
            return Some(MotorResponse::MotorControlMultipleTargets(response_data));
        }
        if let Some(response_data) = ResponseMotorSpeed::new(byte_data) {
            return Some(MotorResponse::MotorSpeed(response_data));
        }
        None
    }
}

impl From<MotorResponse> for u8 {
    fn from(response_type: MotorResponse) -> u8 {
        match response_type {
            MotorResponse::MotorControlTarget(_) => CommandId::TargetPosition.response(),
            MotorResponse::MotorControlMultipleTargets(_) => CommandId::MultiTargetPositions.response(),
            MotorResponse::MotorSpeed(_) => 0xe0u8,
        }
    }
}

impl ToPayload<Vec<u8>> for MotorResponse {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            MotorResponse::MotorControlTarget(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            MotorResponse::MotorControlMultipleTargets(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            MotorResponse::MotorSpeed(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
        }
        payload
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::RequestId;
    use crate::characteristic::motor::def::MotorResponseCode;
    use crate::characteristic::motor::speed::ResponseMotorSpeed;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
    }

    #[test]
    fn motor_response1() {
        _setup();

        let res = MotorResponse::MotorSpeed(ResponseMotorSpeed {
            left: 10,
            right: 11,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }

    #[test]
    fn motor_response2() {
        _setup();

        let res = MotorResponse::MotorControlTarget(ResponseMotorControlTarget {
            request_id: RequestId::new(),
            response_code: MotorResponseCode::ErrorTimeout,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }

    #[test]
    fn motor_response3() {
        _setup();

        let res = MotorResponse::MotorControlMultipleTargets(ResponseMotorControlMultipleTargets {
            request_id: RequestId::new(),
            response_code: MotorResponseCode::ErrorInvalidParameter,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }
}
