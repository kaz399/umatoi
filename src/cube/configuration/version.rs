
use super::def::ConfigurationType;
use serde::Serialize;

/// Requesting the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#requesting-the-ble-protocol-version>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestBleProtocolVersion {
    configuration_type: ConfigurationType,
    _reserved: u8,
}

impl RequestBleProtocolVersion {
    pub fn new() -> Self {
        Self {
            configuration_type: ConfigurationType::BleProtocolVersion,
            _reserved: 0,
        }
    }
}


/// Obtaining the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#obtaining-the-ble-protocol-version>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseBleProtocolVersionData {
    version: [u8; 5],
}

