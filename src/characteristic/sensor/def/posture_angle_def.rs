use serde::Serialize;
use serde::Serializer;


/// Posture data type

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PostureDataType {
    #[default]
    Euler,
    Quaternions,
}

impl From<PostureDataType> for u8 {
    fn from(posture_data_type: PostureDataType) -> u8 {
        match posture_data_type {
            PostureDataType::Euler => 1u8,
            PostureDataType::Quaternions => 2u8,
        }
    }
}

impl Serialize for PostureDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

