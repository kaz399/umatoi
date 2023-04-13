//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_battery>

use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::device_interface::CubeInterface;
use anyhow::Result;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BatteryInformationResponse {
    pub level: usize,
}

impl BatteryInformationResponse {
    pub fn new(byte_data: &[u8]) -> Option<BatteryInformationResponse> {
        if byte_data.is_empty() {
            return None;
        }
        Some(BatteryInformationResponse {
            level: byte_data[1].into(),
        })
    }
}

pub async fn read(interface: &dyn CubeInterface) -> Result<Vec<u8>> {
    interface.read(CoreCubeUuid::BatteryInfo.into()).await
}
