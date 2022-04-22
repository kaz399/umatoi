use super::def::RequestId;
use crate::payload::ToPayload;
use serde::ser::Serializer;
use serde::Serialize;

/// Response to motor control with target specifiled
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#responses-to-motor-control-with-target-specified>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMotorControlWithAngleSpecifiled {
    pub request_id: RequestId,
    pub response_code: ResponseCode,
}

/// Responses to motor control with multiple targets specifiled
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#responses-to-motor-control-with-multiple-targets-specified>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMotorControlWithMultipleTargetsSpecified {
    pub request_id: RequestId,
    pub response_code: ResponseCode,
}

/// Motor Speed information
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_motor/#obtaining-motor-speed-information>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MotorSpeeedInformation {
    pub left: u8,
    pub right: u8,
}

/// Motor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Response {
    MotorControlWithAngleSpecifiled(ResponseMotorControlWithAngleSpecifiled),
    MotorControlWithMultipleTargetsSpecified(ResponseMotorControlWithMultipleTargetsSpecified),
    MotorSpeeedInformation(MotorSpeeedInformation),
    UnknownResponse(u8),
}

impl From<Response> for u8 {
    fn from(response_type: Response) -> u8 {
        match response_type {
            Response::MotorControlWithAngleSpecifiled(_) => 0x83u8,
            Response::MotorControlWithMultipleTargetsSpecified(_) => 0x84u8,
            Response::MotorSpeeedInformation(_) => 0xe0u8,
            Response::UnknownResponse(x) => x,
        }
    }
}

impl Response {
    pub fn new(byte_code: Vec<u8>) -> Option<Response> {
        if byte_code.len() < 3 {
            return None;
        }
        match byte_code[0] {
            0x83u8 => Some(Response::MotorControlWithAngleSpecifiled(
                ResponseMotorControlWithAngleSpecifiled {
                    request_id: RequestId::received(byte_code[1]),
                    response_code: ResponseCode::from(byte_code[2]),
                },
            )),
            0x84u8 => Some(Response::MotorControlWithMultipleTargetsSpecified(
                ResponseMotorControlWithMultipleTargetsSpecified {
                    request_id: RequestId::received(byte_code[1]),
                    response_code: ResponseCode::from(byte_code[2]),
                },
            )),
            0xe0u8 => Some(Response::MotorSpeeedInformation(MotorSpeeedInformation {
                left: byte_code[1],
                right: byte_code[2],
            })),
            _ => None,
        }
    }
}

impl ToPayload<Vec<u8>> for Response {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            Response::MotorControlWithAngleSpecifiled(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MotorControlWithMultipleTargetsSpecified(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MotorSpeeedInformation(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::UnknownResponse(_) => (),
        }
        payload
    }
}

/// Response code from cube

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use crate::payload;

    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_response1() {
        _setup();

        let res = Response::MotorSpeeedInformation(MotorSpeeedInformation {
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
            Response::MotorControlWithAngleSpecifiled(ResponseMotorControlWithAngleSpecifiled {
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
