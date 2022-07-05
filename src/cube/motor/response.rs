use super::def::CommandId;
use crate::payload::ToPayload;
use super::target::ResponseMotorControlWithTargetSpecified;
use super::target::ResponseMotorControlWithMultipleTargetsSpecified;
use super::speed::ResponseMotorSpeedInformation;

/// Motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Response {
    MotorControlWithTargetSpecified(ResponseMotorControlWithTargetSpecified),
    MotorControlWithMultipleTargetsSpecified(ResponseMotorControlWithMultipleTargetsSpecified),
    MotorSpeedInformation(ResponseMotorSpeedInformation),
}

impl Response {
    pub fn new(byte_data: &[u8]) -> Option<Response> {
        if byte_data.is_empty() {
            return None;
        }
        if let Some(response_data) = ResponseMotorControlWithTargetSpecified::new(byte_data) {
            return Some(Response::MotorControlWithTargetSpecified(response_data));
        }
        if let Some(response_data) = ResponseMotorControlWithMultipleTargetsSpecified::new(byte_data) {
            return Some(Response::MotorControlWithMultipleTargetsSpecified(response_data));
        }
        if let Some(response_data) = ResponseMotorSpeedInformation::new(byte_data) {
            return Some(Response::MotorSpeedInformation(response_data));
        }
        None
    }
}

impl From<Response> for u8 {
    fn from(response_type: Response) -> u8 {
        match response_type {
            Response::MotorControlWithTargetSpecified(_) => CommandId::TargetPosition.response(),
            Response::MotorControlWithMultipleTargetsSpecified(_) => CommandId::MultiTargetPositions.response(),
            Response::MotorSpeedInformation(_) => 0xe0u8,
        }
    }
}

impl ToPayload<Vec<u8>> for Response {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            Response::MotorControlWithTargetSpecified(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MotorControlWithMultipleTargetsSpecified(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MotorSpeedInformation(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
        }
        payload
    }
}

#[cfg(test)]
mod test {
    use crate::cube::motor::def::RequestId;
    use crate::cube::motor::def::ResponseCode;
    use crate::cube::motor::speed::ResponseMotorSpeedInformation;
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_response1() {
        _setup();

        let res = Response::MotorSpeedInformation(ResponseMotorSpeedInformation {
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

        let res =
            Response::MotorControlWithTargetSpecified(ResponseMotorControlWithTargetSpecified {
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

        let res = Response::MotorControlWithMultipleTargetsSpecified(
            ResponseMotorControlWithMultipleTargetsSpecified {
                request_id: RequestId::new(),
                response_code: ResponseCode::ErrorInvalidParameter,
            },
        );
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 3);
    }
}
