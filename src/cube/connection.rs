use crate::cube::core_cube::{CoreCube, CoreCubeError, NotificationData};
use crate::ble::BleInterface;
use crate::cube::uuid::CoreCubeUuid;
use crate::handler::HandlerFunction;
use crate::scanner;
use async_trait::async_trait;
use btleplug::api::{
    CharPropFlags, Peripheral as _, ScanFilter,
    WriteType,
};
use log::{debug, error, info, warn};
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

#[async_trait]
impl BleInterface for CoreCube {
    type NotifyHandler = HandlerFunction<NotificationData>;

    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            ble.connect().await?;
            let is_connected = ble.is_connected().await?;
            assert!(is_connected);
            ble.discover_services().await?;
            for service in ble.services() {
                for characteristic in service.characteristics {
                    //println!("characteristic uuid: {:?}", characteristic.uuid);
                    if characteristic.properties.contains(CharPropFlags::NOTIFY) {
                        self.notify_enabled.push(characteristic.uuid);
                        debug!("enable notify uuid: {:?}", characteristic.uuid);
                        ble.subscribe(&characteristic).await?;
                    }
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
            for notified in &self.notify_enabled {
                debug!("disable notify uuid: {:?}", notified);
                ble.unsubscribe(&self.ble_characteristics[notified]).await?;
            }
            ble.disconnect().await?;
            // windows: is_connected is not turned off when device disconnect.
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                let is_connected = ble.is_connected().await?;
                assert!(!is_connected);
            }
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

    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
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
        &mut self,
        func: Self::NotifyHandler,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync + 'static>> {
        let id_handler = self.root_notify_manager.register(func)?;
        Ok(id_handler)
    }

    async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        self.root_notify_manager.unregister(id_handler)?;
        Ok(true)
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
            if *service_uuid == CoreCubeUuid::Service.uuid() {
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
                        "local name does not match (device:{}, specified:{})",
                        device_local_name, cube_local_name
                    );
                    continue;
                }
            }
            if cfg!(target_os = "macos") {
                warn!("scanning cube with BDAddress is not supported on MacOS");
            } else if let Some(cube_address) = &cube.address {
                if device_address != *cube_address {
                    debug!(
                        "address does not match (device:{}, specified:{})",
                        device_address, cube_address
                    );
                    continue;
                }
            }
            cube.local_name = Some(device_local_name.clone());
            cube.address = Some(device_address);
            cube.ble_peripheral = Some(peripheral);
            info!(
                "found toio core cube: local_name: {}, addr: {}",
                device_local_name, device_address
            );
            return Ok(true);
        }
    }
    error!("toio core cube is not found");
    Err(Box::new(CoreCubeError::CubeNotFound))
}

#[cfg(test)]
mod tests {
    use crate::cube::id_information::{self, IdInformation};

    use super::*;
    use std::time::Duration;
    use tokio::time;

    static TEST_CUBE_NAME: &str = "toio Core Cube-h7p";
    static TEST_CUBE_BDADDR: [u8; 6] = [0xd8, 0xe3, 0x49, 0xa0, 0xef, 0x19];

    // static TEST_CUBE_NAME = "toio Core Cube-G9F";
    // static TEST_CUBE_BDADDR = [0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce];

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn notify_handler(data: NotificationData) {
        if let Some(id_data) = id_information::IdInformation::new(&data.value) {
            match id_data {
                IdInformation::PositionId(pos_id) => {
                    println!("position id: {:?}", pos_id);
                }
                IdInformation::StandardId(std_id) => {
                    println!("standard id: {:?}", std_id);
                }
                _ => (),
            }
        } else {
            println!(
                "notify handler1: uuid: {:?} value: {:?}",
                data.uuid, data.value
            );
        }
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
        let mut cube = CoreCube::new_with_name(TEST_CUBE_NAME.to_string());
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan3() {
        _setup();
        let mut cube = CoreCube::new_with_address(BDAddr::from(TEST_CUBE_BDADDR));
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan4() {
        _setup();
        let mut cube = CoreCube::new_with_name_address(
            TEST_CUBE_NAME.to_string(),
            BDAddr::from(TEST_CUBE_BDADDR),
        );
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cube_scan5() {
        _setup();
        let mut cube = CoreCube::new_with_name_address(
            TEST_CUBE_NAME.to_string(),
            BDAddr::from(TEST_CUBE_BDADDR),
        );
        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();

        cube.connect().await.unwrap();

        let mut cube2 = CoreCube::new_with_name_address(
            TEST_CUBE_NAME.to_string(),
            BDAddr::from(TEST_CUBE_BDADDR),
        );
        let result = search_cube(&mut cube2, Duration::from_secs(3)).await;
        if result.is_ok() {
            panic!();
        }
        let result = cube2.connect().await;
        if result.is_ok() {
            panic!();
        }
        if cube.disconnect().await.is_err() {
            panic!()
        }
        drop(cube);
    }

    #[tokio::test]
    async fn cube_notify1() {
        _setup();
        let (tx, rx) = mpsc::channel::<CoreCubeNotifyControl>();
        let mut cube = CoreCube::new();

        search_cube(&mut cube, Duration::from_secs(3))
            .await
            .unwrap();

        cube.connect().await.unwrap();
        let handler_uuid = cube
            .register_notify_handler(Box::new(&notify_handler))
            .await
            .unwrap();
        info!("handler uuid {:?}", handler_uuid);

        let data: NotificationData = NotificationData {
            uuid: Uuid::new_v4(),
            value: [1, 2, 3].to_vec(),
        };
        cube.root_notify_manager.invoke_all_handlers(data).unwrap();

        //cube.receive_notify().await.unwrap();

        let notify_receiver = cube.run_notify_receiver(rx);
        let timer = async {
            tx.send(CoreCubeNotifyControl::Run).unwrap();
            time::sleep(Duration::from_secs(8)).await;
            tx.send(CoreCubeNotifyControl::Quit).unwrap();
        };

        let _ = tokio::join!(notify_receiver, timer);

        if cube.unregister_notify_handler(handler_uuid).await.is_err() {
            panic!();
        }
        if cube.disconnect().await.is_err() {
            panic!()
        }
        drop(cube);

        // wait to complete the disconnection process of the cube
        time::sleep(Duration::from_secs(5)).await;
    }
}
