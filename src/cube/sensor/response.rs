use super::magnetic::MagneticSensorData;
use super::motion::{MotionDetectionData, Posture};
use super::posture_angle::{PostureAngleEulerData, PostureAngleQuaternionsData, PostureDataType};
use crate::payload::ToPayload;

/// Sensor response

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Response {
    MotionDetection(MotionDetectionData),
    PostureAngleEuler(PostureAngleEulerData),
    PostureAngleQuaternion(PostureAngleQuaternionsData),
    MagneticSensor(MagneticSensorData),
}

impl From<Response> for u8 {
    fn from(response_type: Response) -> u8 {
        match response_type {
            Response::MotionDetection(_) => 0x01u8,
            Response::PostureAngleEuler(_) => 0x03u8,
            Response::PostureAngleQuaternion(_) => 0x03u8,
            Response::MagneticSensor(_) => 0x02u8,
        }
    }
}

impl Response {
    pub fn new(byte_code: &Vec<u8>) -> Option<Response> {
        if byte_code.is_empty() {
            return None;
        }
        match byte_code[0] {
            0x01u8 => {
                if byte_code.len() < 6 {
                    return None;
                }
                Some(Response::MotionDetection(MotionDetectionData {
                    horizontal: byte_code[1] != 0,
                    collision: byte_code[2] != 0,
                    double_tap: byte_code[3] != 0,
                    posture: Posture::from(byte_code[4]),
                    shake: byte_code[5],
                }))
            }
            0x02u8 => {
                if byte_code.len() < 6 {
                    return None;
                }
                Some(Response::MagneticSensor(MagneticSensorData {
                    state: byte_code[1],
                    strength: byte_code[2],
                    x: i8::from_le_bytes([byte_code[3]]),
                    y: i8::from_le_bytes([byte_code[4]]),
                    z: i8::from_le_bytes([byte_code[5]]),
                }))
            }
            0x03u8 => {
                if byte_code.len() < 2 {
                    return None;
                }
                match byte_code[1] {
                    0x01u8 => {
                        if byte_code.len() < 8 {
                            return None;
                        }
                        Some(Response::PostureAngleEuler(PostureAngleEulerData {
                            roll: i16::from_le_bytes([byte_code[2], byte_code[3]]),
                            pitch: i16::from_le_bytes([byte_code[4], byte_code[5]]),
                            yaw: i16::from_le_bytes([byte_code[6], byte_code[7]]),
                        }))
                    }
                    0x02u8 => {
                        if byte_code.len() < 10 {
                            return None;
                        }
                        Some(Response::PostureAngleQuaternion(
                            PostureAngleQuaternionsData {
                                w: i16::from_le_bytes([byte_code[2], byte_code[3]]),
                                x: i16::from_le_bytes([byte_code[4], byte_code[5]]),
                                y: i16::from_le_bytes([byte_code[6], byte_code[7]]),
                                z: i16::from_le_bytes([byte_code[8], byte_code[9]]),
                            },
                        ))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl ToPayload<Vec<u8>> for Response {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            Response::MotionDetection(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::PostureAngleEuler(st) => {
                payload.push(PostureDataType::Euler.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::PostureAngleQuaternion(st) => {
                payload.push(PostureDataType::Quaternions.into());
                payload.extend(bincode::serialize(&st).unwrap());
            }
            Response::MagneticSensor(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
        }

        payload
    }
}
