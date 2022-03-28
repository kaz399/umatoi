use log::debug;
use uuid::Uuid;
use btleplug::api::BDAddr;
use btleplug::platform::Peripheral;

use crate::ble;

// UUIDs
pub const TOIO_UUID_SERVICE: Uuid      = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_ID_INFO: Uuid      = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_SENSOR_INFO: Uuid  = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_BUTTON_INFO: Uuid  = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_BATTERY_INFO: Uuid = Uuid::from_u128(0x10B20108_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_MOTOR_CTRL: Uuid   = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_LIGHT_CTRL: Uuid   = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_SOUND_CTRL: Uuid   = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_CONFIG: Uuid       = Uuid::from_u128(0x10B201FF_5B3B_4571_9508_CF3EFCD7BBAE);

pub struct CoreCube {
    pub nickname: String,
    pub local_name: Option<String>,
    pub address: Option<BDAddr>,
    pub peripheral: Option<Peripheral>,

}

impl Default for CoreCube {
    fn default() -> Self {
        Self {
            nickname: "cube".to_string(),
            local_name: None,
            address: None,
            peripheral: None,
        }
    }
}

impl Drop for CoreCube {
    fn drop(&mut self) {
        debug!("Drop: CoreCube:{}", self.nickname);
    }
}


