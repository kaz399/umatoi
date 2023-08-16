use super::super::def::id_type::{PositionIdData, StandardIdData};
use crate::{
    payload::FromPayload,
    position::{CubeLocation, Point},
};

/// Id information
/// Position ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#position-id>
/// Standard ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#standard-id>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IdInformation {
    PositionId(PositionIdData),
    StandardId(StandardIdData),
    StandardIdMissed,
    PositionIdMissed,
    Unknown(u8),
}

impl From<IdInformation> for u8 {
    fn from(id_information: IdInformation) -> u8 {
        match id_information {
            IdInformation::PositionId(_) => 0x01u8,
            IdInformation::StandardId(_) => 0x02u8,
            IdInformation::StandardIdMissed => 0x03u8,
            IdInformation::PositionIdMissed => 0x04u8,
            IdInformation::Unknown(x) => x,
        }
    }
}

impl FromPayload<&[u8]> for IdInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.is_empty() {
            return None;
        }
        match payload[0] {
            0x01u8 => {
                if payload.len() >= 13 {
                    Some(IdInformation::PositionId(PositionIdData {
                        center: CubeLocation {
                            point: Point {
                                x: isize::from_le_bytes([payload[1], payload[2], 0, 0, 0, 0, 0, 0]),
                                y: isize::from_le_bytes([payload[3], payload[4], 0, 0, 0, 0, 0, 0]),
                            },
                            angle: u16::from_le_bytes([payload[5], payload[6]]),
                        },
                        sensor: CubeLocation {
                            point: Point {
                                x: isize::from_le_bytes([payload[7], payload[8], 0, 0, 0, 0, 0, 0]),
                                y: isize::from_le_bytes([
                                    payload[9],
                                    payload[10],
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ]),
                            },
                            angle: u16::from_le_bytes([payload[11], payload[12]]),
                        },
                    }))
                } else {
                    None
                }
            }
            0x02u8 => {
                if payload.len() >= 4 {
                    Some(IdInformation::StandardId(StandardIdData {
                        value: u32::from_le_bytes([payload[1], payload[2], payload[3], payload[4]]),
                        angle: u16::from_le_bytes([payload[5], payload[6]]),
                    }))
                } else {
                    None
                }
            }
            0x03u8 => Some(IdInformation::PositionIdMissed),
            0x04u8 => Some(IdInformation::StandardIdMissed),
            _ => None,
        }
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

        let res = IdInformation::PositionId(PositionIdData {
            center: CubeLocation {
                point: Point { x: 1, y: 2 },
                angle: 3,
            },
            sensor: CubeLocation {
                point: Point { x: 4, y: 5 },
                angle: 6,
            },
        });
    }
}
