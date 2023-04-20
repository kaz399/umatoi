use serde::Serialize;
use serde::Serializer;


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Posture {
    #[default]
    Unknown,
    Top,
    Bottom,
    Rear,
    Front,
    Right,
    Left,
}

impl From<Posture> for u8 {
    fn from(posture: Posture) -> u8 {
        match posture {
            Posture::Unknown => 0u8,
            Posture::Top => 1u8,
            Posture::Bottom => 2u8,
            Posture::Rear => 3u8,
            Posture::Front => 4u8,
            Posture::Right => 5u8,
            Posture::Left => 6u8,
        }
    }
}

impl From<u8> for Posture {
    fn from(code: u8) -> Posture {
        match code {
            1u8 => Posture::Top,
            2u8 => Posture::Bottom,
            3u8 => Posture::Rear,
            4u8 => Posture::Front,
            5u8 => Posture::Right,
            6u8 => Posture::Left,
            _ => Posture::Unknown,
        }
    }
}

impl Serialize for Posture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}
