use crate::payload::ToPayload;
use serde::Serialize;

/// Position ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#position-id>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PositionIdData {
    pub x: u16,
    pub y: u16,
    pub angle: u16,
    pub sensor_x: u16,
    pub sensor_y: u16,
    pub sensor_angle: u16,
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

impl IdInformation {
    pub fn new(byte_code: &[u8]) -> Option<IdInformation> {
        if byte_code.is_empty() {
            return None;
        }
        match byte_code[0] {
            0x01u8 => {
                if byte_code.len() >= 13 {
                    Some(IdInformation::PositionId(PositionIdData {
                        x: u16::from_le_bytes([byte_code[1], byte_code[2]]),
                        y: u16::from_le_bytes([byte_code[3], byte_code[4]]),
                        angle: u16::from_le_bytes([byte_code[5], byte_code[6]]),
                        sensor_x: u16::from_le_bytes([byte_code[7], byte_code[8]]),
                        sensor_y: u16::from_le_bytes([byte_code[9], byte_code[10]]),
                        sensor_angle: u16::from_le_bytes([byte_code[11], byte_code[12]]),
                    }))
                } else {
                    None
                }
            }
            0x02u8 => {
                if byte_code.len() >= 4 {
                    Some(IdInformation::StandardId(StandardIdData {
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
            0x03u8 => Some(IdInformation::PositionIdMissed),
            0x04u8 => Some(IdInformation::StandardIdMissed),
            _ => None,
        }
    }
}

impl ToPayload<Vec<u8>> for IdInformation {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![u8::from(self)];
        match self {
            IdInformation::PositionId(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            IdInformation::StandardId(st) => {
                payload.extend(bincode::serialize(&st).unwrap());
            }
            IdInformation::PositionIdMissed => (),
            IdInformation::StandardIdMissed => (),
            IdInformation::Unknown(_) => (),
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

        let res = IdInformation::PositionId(PositionIdData {
            x: 1,
            y: 2,
            angle: 3,
            sensor_x: 4,
            sensor_y: 5,
            sensor_angle: 6,
        });
        let payload = res.to_payload();
        println!("len:{:2} data:{:?}", payload.len(), payload);
        assert_eq!(payload.len(), 13);
    }
}
