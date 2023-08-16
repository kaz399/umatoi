use crate::payload::ToPayload;

/// Magnet function

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MagnetFunction {
    Disable,
    MagnetState,
    MagnetForce,
}

impl From<MagnetFunction> for u8 {
    fn from(function_type: MagnetFunction) -> u8 {
        match function_type {
            MagnetFunction::Disable => 0x00u8,
            MagnetFunction::MagnetState => 0x01u8,
            MagnetFunction::MagnetForce => 0x02u8,
        }
    }
}

impl ToPayload<Vec<u8>> for MagnetFunction {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.into()];
        payload
    }
}
