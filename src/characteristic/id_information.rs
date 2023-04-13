use crate::{payload::ToPayload, position::CubeLocation, position::Point};
use serde::Serialize;

/// Position ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#position-id>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PositionIdData {
    pub center: CubeLocation,
    pub sensor: CubeLocation,
}

/// Standard ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#standard-id>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct StandardIdData {
    pub value: u32,
    pub angle: u16,
}

/// Id information

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IdInformationResponse {
    PositionId(PositionIdData),
    StandardId(StandardIdData),
    StandardIdMissed,
    PositionIdMissed,
    Unknown(u8),
}

impl From<IdInformationResponse> for u8 {
    fn from(id_information: IdInformationResponse) -> u8 {
        match id_information {
            IdInformationResponse::PositionId(_) => 0x01u8,
            IdInformationResponse::StandardId(_) => 0x02u8,
            IdInformationResponse::StandardIdMissed => 0x03u8,
            IdInformationResponse::PositionIdMissed => 0x04u8,
            IdInformationResponse::Unknown(x) => x,
        }
    }
}

impl IdInformationResponse {
    pub fn new(byte_code: &[u8]) -> Option<IdInformationResponse> {
        if byte_code.is_empty() {
            return None;
        }
        match byte_code[0] {
            0x01u8 => {
                if byte_code.len() >= 13 {
                    Some(IdInformationResponse::PositionId(PositionIdData {
                        center: CubeLocation {
                            point: Point {
                                x: isize::from_le_bytes([
                                    byte_code[1],
                                    byte_code[2],
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ]),
                                y: isize::from_le_bytes([
                                    byte_code[3],
                                    byte_code[4],
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ]),
                            },
                            angle: u16::from_le_bytes([byte_code[5], byte_code[6]]),
                        },
                        sensor: CubeLocation {
                            point: Point {
                                x: isize::from_le_bytes([
                                    byte_code[7],
                                    byte_code[8],
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ]),
                                y: isize::from_le_bytes([
                                    byte_code[9],
                                    byte_code[10],
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ]),
                            },
                            angle: u16::from_le_bytes([byte_code[11], byte_code[12]]),
                        },
                    }))
                } else {
                    None
                }
            }
            0x02u8 => {
                if byte_code.len() >= 4 {
                    Some(IdInformationResponse::StandardId(StandardIdData {
                        value: u32::from_le_bytes([
                            byte_code[1],
                            byte_code[2],
                            byte_code[3],
                            byte_code[4],
                        ]),
                        angle: u16::from_le_bytes([byte_code[5], byte_code[6]]),
                    }))
                } else {
                    None
                }
            }
            0x03u8 => Some(IdInformationResponse::PositionIdMissed),
            0x04u8 => Some(IdInformationResponse::StandardIdMissed),
            _ => None,
        }
    }
}

impl ToPayload<Vec<u8>> for IdInformationResponse {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            IdInformationResponse::PositionId(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            IdInformationResponse::StandardId(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            IdInformationResponse::PositionIdMissed => (),
            IdInformationResponse::StandardIdMissed => (),
            IdInformationResponse::Unknown(_) => (),
        }
        payload
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn id_information1() {
        _setup();

        let res = IdInformationResponse::PositionId(PositionIdData {
            center: CubeLocation {
                point: Point { x: 1, y: 2 },
                angle: 3,
            },
            sensor: CubeLocation {
                point: Point { x: 4, y: 5 },
                angle: 6,
            },
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 13);
    }
}
