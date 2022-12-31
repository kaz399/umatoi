//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_battery>

use anyhow::Result;
use crate::device_interface::CubeInterface;
use crate::characteristic::characteristic_uuid::CoreCubeUuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BatteryInformation {
    pub level: usize,
}

impl Default for BatteryInformation {
    fn default() -> Self {
        Self {
            level: 0,
        }
    }
}

impl BatteryInformation {
    pub fn new(byte_data: &[u8]) -> Option<BatteryInformation> {
        if byte_data.is_empty() {
            return None;
        }
        Some(BatteryInformation{
            level: byte_data[1].into(),
        }
        )
    }
}


pub async fn read(interface: &dyn CubeInterface) -> Result<Vec<u8>> {
    interface.read(CoreCubeUuid::BatteryInfo.into()).await
}
