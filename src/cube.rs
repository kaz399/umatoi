use async_trait::async_trait;
use btleplug::api::{BDAddr, Characteristic, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::Peripheral;
use log::{debug, error, info};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

use crate::ble::BleInterface;
use crate::handler::{NotifyFunction, NotifyManager};
use crate::scanner;

// UUIDs
pub const TOIO_UUID_SERVICE: Uuid = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_ID_INFO: Uuid = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_SENSOR_INFO: Uuid = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_BUTTON_INFO: Uuid = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_BATTERY_INFO: Uuid = Uuid::from_u128(0x10B20108_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_MOTOR_CTRL: Uuid = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_LIGHT_CTRL: Uuid = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_SOUND_CTRL: Uuid = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);
pub const TOIO_UUID_CONFIG: Uuid = Uuid::from_u128(0x10B201FF_5B3B_4571_9508_CF3EFCD7BBAE);

#[derive(Error, Debug, PartialEq)]
pub enum CoreCubeError {
    #[error("toio core cube is not found")]
    CobeNotFound,
    #[error("no bluetooth peripherals")]
    NoBlePeripherals,
    #[error("internal error of cube.rs")]
    FoundBug,
}

pub struct CoreCube {
    pub local_name: Option<String>,
    pub address: Option<BDAddr>,
    ble_peripheral: Option<Peripheral>,
    ble_characteristics: HashMap<Uuid, Characteristic>,
    notify_handlers: HashMap<Uuid, NotifyManager<Uuid, NotifyFunction>>,
}

impl CoreCube {
    pub fn new() -> Self {
        Self {
            local_name: None,
            address: None,
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_handlers: HashMap::new(),
        }
    }

    pub fn new_with_name(name: String) -> Self {
        Self {
            local_name: Some(name),
            address: None,
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_handlers: HashMap::new(),
        }
    }

    pub fn new_with_address(addr: BDAddr) -> Self {
        Self {
            local_name: None,
            address: Some(addr),
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_handlers: HashMap::new(),
        }
    }

    pub fn new_with_name_address(name: String, addr: BDAddr) -> Self {
        Self {
            local_name: Some(name),
            address: Some(addr),
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_handlers: HashMap::new(),
        }
    }
}

#[async_trait]
impl BleInterface for CoreCube {
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            ble.connect().await?;
            let is_connected = ble.is_connected().await?;
            assert_eq!(is_connected, true);
            ble.discover_services().await?;
            for service in ble.services() {
                for characteristic in service.characteristics {
                    println!("characteristic uuid: {:?}", characteristic.uuid);
                    self.ble_characteristics
                        .insert(characteristic.uuid, characteristic);
                }
            }
            Ok(())
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            ble.disconnect().await?;
            let is_connected = ble.is_connected().await?;
            assert_eq!(is_connected, false);
            self.ble_characteristics.clear();
            Ok(())
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }

    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            let characteristic = self.ble_characteristics.get(&uuid).unwrap();
            let data = ble.read(characteristic).await?;
            Ok(data)
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }

    // write data to specified characteristic
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &Vec<u8>,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            let characteristic = self.ble_characteristics.get(&uuid).unwrap();
            ble.write(characteristic, bytes, WriteType::WithoutResponse)
                .await?;
            Ok(true)
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }

    async fn register_notify_handler(
        &self,
        uuid: Uuid,
        func: NotifyFunction,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            let characteristic = self.ble_characteristics.get(&uuid).unwrap();
            Ok(true)
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }
}

impl Drop for CoreCube {
    fn drop(&mut self) {
        debug!("Drop: CoreCube");
    }
}

pub async fn search_cube(
    cube: &mut CoreCube,
    wait: Duration,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    let peripheral_list = scanner::scan(ScanFilter::default(), wait).await?;
    for peripheral in peripheral_list.into_iter() {
        if peripheral.is_connected().await? {
            debug!("skip connected device");
            continue;
        }
        let mut found = false;
        let properties = peripheral.properties().await?.unwrap();
        for service_uuid in properties.services.iter() {
            info!("service uuid: {}", service_uuid);
            if *service_uuid == TOIO_UUID_SERVICE {
                debug!("found toio core cube: service uuid: {}", service_uuid);
                found = true;
                break;
            }
        }
        if found {
            let device_local_name = properties.local_name.unwrap();
            let device_address = peripheral.address();

            if let Some(cube_local_name) = &cube.local_name {
                if device_local_name != *cube_local_name {
                    debug!(
                        "local name does not match (device:{}, specifiled:{})",
                        device_local_name, cube_local_name
                    );
                    continue;
                }
            }
            if let Some(cube_address) = &cube.address {
                if device_address != *cube_address {
                    debug!(
                        "address does not match (device:{}, specifiled:{})",
                        device_address, cube_address
                    );
                    continue;
                }
            }
            cube.local_name = Some(device_local_name.clone());
            cube.address = Some(device_address.clone());
            cube.ble_peripheral = Some(peripheral);
            info!(
                "found toio core cube: local_name: {}, addr: {}",
                device_local_name, device_address
            );
            return Ok(true);
        }
    }
    error!("toio core cube is not found");
    Err(Box::new(CoreCubeError::CobeNotFound))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn cube_scan1() {
        _setup();
        let mut cube = CoreCube::new();
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan2() {
        _setup();
        let mut cube = CoreCube::new_with_name("toio Core Cube-G9F".to_string());
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan3() {
        _setup();
        let mut cube =
            CoreCube::new_with_address(BDAddr::from([0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce]));
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan4() {
        _setup();
        let mut cube = CoreCube::new_with_name_address(
            "toio Core Cube-G9F".to_string(),
            BDAddr::from([0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce]),
        );
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan5() {
        _setup();
        let mut cube = CoreCube::new_with_name_address(
            "toio Core Cube-G9F".to_string(),
            BDAddr::from([0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce]),
        );
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();

        cube.connect().await.unwrap();

        let mut cube2 = CoreCube::new_with_name_address(
            "toio Core Cube-G9F".to_string(),
            BDAddr::from([0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce]),
        );
        let result = search_cube(&mut cube2, Duration::from_secs(3)).await;
        match result {
            Ok(_) => panic!(),
            Err(_) => (),
        }
        let result = cube2.connect().await;
        match result {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }
}
