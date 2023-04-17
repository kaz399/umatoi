use super::def::CommandId;
use super::speed::ResponseMotorSpeed;
use super::target::ResponseMotorControlMultipleTargets;
use super::target::ResponseMotorControlTarget;
use crate::payload::ToPayload;

/// Motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MotorResponse {
    MotorControlTarget(ResponseMotorControlTarget),
    MotorControlMultipleTargets(ResponseMotorControlMultipleTargets),
    MotorSpeed(ResponseMotorSpeed),
}

impl MotorResponse {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
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
            MotorResponse::MotorControlMultipleTargets(_) => {
                CommandId::MultiTargetPositions.response()
            }
            MotorResponse::MotorSpeed(_) => 0xe0u8,
        }
    }
}

impl ToPayload<Vec<u8>> for MotorResponse {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            MotorResponse::MotorControlTarget(st) => {
                payload.extend(st.to_payload());
            }
            MotorResponse::MotorControlMultipleTargets(st) => {
                payload.extend(st.to_payload());
            }
            MotorResponse::MotorSpeed(st) => {
                payload.extend(st.to_payload());
            }
        }
        payload
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::characteristic::motor::def::RequestId;
    use crate::characteristic::motor::def::ResponseCode;
    use crate::characteristic::motor::speed::ResponseMotorSpeed;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
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
            response_code: ResponseCode::ErrorTimeout,
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
            response_code: ResponseCode::ErrorInvalidParameter,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }
}
