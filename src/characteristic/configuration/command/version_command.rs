use super::super::def::common_def::ConfigurationType;

/// Requesting the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#requesting-the-ble-protocol-version>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestBleProtocolVersion {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
}

impl Default for RequestBleProtocolVersion {
    fn default() -> Self {
        Self {
            configuration_type: ConfigurationType::BleProtocolVersion,
            _reserved: 0,
        }
    }
}

impl RequestBleProtocolVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
