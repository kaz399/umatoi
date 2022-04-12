use serde::ser::Serializer;
use serde::Serialize;
use std::time;

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
