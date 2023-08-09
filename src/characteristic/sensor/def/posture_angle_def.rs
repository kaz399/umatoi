use crate::payload::ToPayload;

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

impl ToPayload<Vec<u8>> for PostureDataType {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
