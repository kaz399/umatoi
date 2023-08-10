use super::super::def::common_def::ConfigurationType;
use crate::payload::FromPayload;


/// Obtaining the BLE protocol version
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#obtaining-the-ble-protocol-version>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseBleProtocolVersionData {
    pub version: [u8; 5],
}

impl FromPayload<&[u8]> for ResponseBleProtocolVersionData {
    fn from_payload(payload: &[u8]) -> Option<Self> where Self: Sized {
        if payload.len() != 7 {
            return None;
        }
        if payload[0] == ConfigurationType::BleProtocolVersion.response() {
            let version: [u8; 5] = [
                payload[2],
                payload[3],
                payload[4],
                payload[5],
                payload[6],
            ];
            Some(Self { version })
        } else {
            None
        }
    }
}
