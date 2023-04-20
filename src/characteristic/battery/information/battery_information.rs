//! Official Specification:  <https://toio.github.io/toio-spec/en/docs/ble_battery>

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BatteryInformation {
    pub level: usize,
}

impl BatteryInformation {
    pub fn new(byte_data: &[u8]) -> Option<BatteryInformation> {
        if byte_data.is_empty() {
            return None;
        }
        Some(BatteryInformation {
            level: byte_data[1].into(),
        })
    }
}

