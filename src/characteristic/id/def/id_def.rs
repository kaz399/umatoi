use crate::position::CubeLocation;
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

