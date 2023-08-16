use super::super::def::command_id_def::CommandId;
use crate::payload::ToPayload;

/// Requesting the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#requesting-the-ble-protocol-version>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RequestBleProtocolVersion {
    pub configuration_type: CommandId,
    pub _reserved: u8,
}

impl Default for RequestBleProtocolVersion {
    fn default() -> Self {
        Self {
            configuration_type: CommandId::BleProtocolVersion,
            _reserved: 0,
        }
    }
}

impl RequestBleProtocolVersion {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToPayload<Vec<u8>> for RequestBleProtocolVersion {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![self.configuration_type.into(), self._reserved];
        payload
    }
}
