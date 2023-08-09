use crate::payload::ToPayload;
use crate::position::CubeLocation;

/// Position ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#position-id>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PositionIdData {
    pub center: CubeLocation,
    pub sensor: CubeLocation,
}

impl ToPayload<Vec<u8>> for PositionIdData {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.center.to_payload());
        payload.extend(self.sensor.to_payload());
        payload
    }
}

/// Standard ID
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_id#standard-id>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StandardIdData {
    pub value: u32,
    pub angle: u16,
}

impl ToPayload<Vec<u8>> for StandardIdData {
    fn to_payload(self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.value.to_le_bytes().to_vec());
        payload.extend(self.angle.to_le_bytes().to_vec());
        payload
    }
}
