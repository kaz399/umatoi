//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_battery>

use crate::payload::FromPayload;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BatteryInformation {
    pub level: usize,
}

impl FromPayload<&[u8]> for BatteryInformation {
    fn from_payload(payload: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if payload.is_empty() {
            return None;
        }
        Some(BatteryInformation {
            level: payload[1].into(),
        })
    }
}
