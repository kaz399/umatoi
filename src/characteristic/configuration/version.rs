use super::def::ConfigurationType;
use serde::Serialize;

/// Requesting the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#requesting-the-ble-protocol-version>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestBleProtocolVersion {
    configuration_type: ConfigurationType,
    _reserved: u8,
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

/// Obtaining the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#obtaining-the-ble-protocol-version>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseBleProtocolVersionData {
    version: [u8; 5],
}

impl ResponseBleProtocolVersionData {
    pub fn new(byte_data: &[u8]) -> Option<Self> {
        if byte_data.len() != 7 {
            return None;
        }
        if byte_data[0] == ConfigurationType::BleProtocolVersion.response() {
            let version: [u8; 5] = [
                byte_data[2],
                byte_data[3],
                byte_data[4],
                byte_data[5],
                byte_data[6],
            ];
            Some(Self { version })
        } else {
            None
        }
    }
}
