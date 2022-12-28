use anyhow::Result;
use crate::cube::characteristic_uuid::CoreCubeUuid;
use crate::cube::{CoreCubeError, NotificationData};
use crate::device_interface::CubeInterface;
use crate::notification_manager::NotificationManager;
use async_trait::async_trait;
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use log::{debug, error, info};
use std::collections::HashMap;
use std::time::Duration;
use std::vec::Vec;
use tokio::time;
use uuid::Uuid;

type BleInterface = Peripheral;

pub struct BleCube {
    pub ble_peripheral: BleInterface,
    pub ble_characteristics: HashMap<Uuid, Characteristic>,
    pub notification_enabled: Vec<Uuid>,
}


impl BleCube {

    pub fn new(peripheral: Peripheral) -> Self {
        Self {
            ble_peripheral: peripheral,
            ble_characteristics: HashMap::new(),
            notification_enabled: Vec::new(),
        }
    }

}

pub async fn ble_notification_receiver(ble_peripheral: Peripheral, notification_manager: &NotificationManager<NotificationData>) -> Result<()> {
    let mut notification_stream = ble_peripheral.notifications().await.unwrap();
    while let Some(data) = notification_stream.next().await {
        let _ = notification_manager.invoke_all_handlers(data);
    }
    Ok(())
}

#[async_trait]
impl CubeInterface for BleCube {

    async fn connect(&mut self) -> Result<()> {
        self.ble_peripheral.connect().await?;
        let is_connected = self.ble_peripheral.is_connected().await?;
        assert!(is_connected);
        self.ble_peripheral.discover_services().await?;
        for service in self.ble_peripheral.services() {
            for characteristic in service.characteristics {
                //println!("characteristic uuid: {:?}", characteristic.uuid);
                if characteristic.properties.contains(CharPropFlags::NOTIFY) {
                    self.notification_enabled.push(characteristic.uuid);
                    debug!("enable notification uuid: {:?}", characteristic.uuid);
                    self.ble_peripheral.subscribe(&characteristic).await?;
                }
                self.ble_characteristics
                    .insert(characteristic.uuid, characteristic);
            }
        }
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        for notified in &self.notification_enabled {
            debug!("disable notification uuid: {:?}", notified);
            self.ble_peripheral.unsubscribe(&self.ble_characteristics[notified]).await?;
        }
        self.ble_peripheral.disconnect().await?;
        // windows: is_connected is not turned off when device disconnect.
        // macos: is_connected is not turned off when device disconnect.
        if cfg!(target_os = "linux") {
            let is_connected = self.ble_peripheral.is_connected().await?;
            assert!(!is_connected);
        }
        self.ble_characteristics.clear();
        Ok(())
    }

    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        let data = self.ble_peripheral.read(characteristic).await?;
        Ok(data)
    }

    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral.write(characteristic, bytes, WriteType::WithoutResponse)
            .await?;
        Ok(true)
    }

    async fn write_with_response(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral.write(characteristic, bytes, WriteType::WithResponse)
            .await?;
        Ok(true)
    }
}


pub struct BleScanner;

impl BleScanner
{
    async fn scan_ble(
        &self,
        filter: ScanFilter,
        wait: Duration,
    ) -> Result<Vec<BleInterface>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        let mut peripheral_list: Vec<BleInterface> = Vec::new();

        if adapter_list.is_empty() {
            error!("No Bluetooth adapters found");
            return Ok(peripheral_list)
        }

        for adapter in adapter_list.iter() {
            println!("Starting scan on {}...", adapter.adapter_info().await?);
            adapter.start_scan(filter.clone()).await?;
            time::sleep(wait).await;
            adapter.stop_scan().await?;
            for (index, peripheral) in adapter.peripherals().await?.iter().enumerate() {
                debug!("{} {:?}", index, peripheral);
                if peripheral.is_connected().await? {
                    debug!("skip connected device");
                    continue;
                }
                let properties = peripheral.properties().await?.unwrap();
                for service_uuid in properties.services.iter() {
                    info!("service uuid: {}", service_uuid);
                    if *service_uuid == CoreCubeUuid::Service.uuid() {
                        debug!("found toio core cube: service uuid: {}", service_uuid);
                        peripheral_list.push(peripheral.clone());
                    }
                }
            }
        }
        debug!("total {} peripherals found", peripheral_list.len());
        Ok(peripheral_list)
    }

    pub async fn scan(
        &self,
        num: usize,
        wait: Duration,
    ) -> Result<Vec<BleInterface>> {
        let mut peripheral_list = self.scan_ble(ScanFilter::default(), wait).await.unwrap();
        peripheral_list.truncate(num);
        if peripheral_list.is_empty() {
            error!("toio core cube is not found");
            return Err(CoreCubeError::CubeNotFound.into());
        }
        Ok(peripheral_list)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::id_information::{self, IdInformation};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time;

    static TEST_CUBE_NAME: &str = "toio Core Cube-h7p";
    static TEST_CUBE_BDADDR: [u8; 6] = [0xd8, 0xe3, 0x49, 0xa0, 0xef, 0x19];

    // static TEST_CUBE_NAME = "toio Core Cube-G9F";
    // static TEST_CUBE_BDADDR = [0xc1, 0xd5, 0x19, 0x31, 0x5f, 0xce];

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn _teardown() {
        std::thread::sleep(Duration::from_millis(500));
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
        let mut scanner = BleScanner;
        let mut cube = CoreCube::<BleCubeInterface>::new();
        cube.scan(None, None, Duration::from_secs(5)).await.unwrap();
        drop(cube);
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan2() {
        _setup();
        let mut cube = CoreCube::<BleCubeInterface>::new();
        cube.scan(
            Some(BDAddr::from(TEST_CUBE_BDADDR)),
            None,
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        drop(cube);
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan3() {
        _setup();
        let mut cube = CoreCube::<BleCubeInterface>::new();
        cube.scan(
            None,
            Some(TEST_CUBE_NAME.to_string()),
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        drop(cube);
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan4() {
        _setup();
        let mut cube = CoreCube::<BleCubeInterface>::new();
        cube.scan(
            Some(BDAddr::from(TEST_CUBE_BDADDR)),
            Some(TEST_CUBE_NAME.to_string()),
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        drop(cube);
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan5() {
        _setup();
        let mut cube = CoreCube::<BleCubeInterface>::new();
        cube.scan(
            Some(BDAddr::from(TEST_CUBE_BDADDR)),
            Some(TEST_CUBE_NAME.to_string()),
            Duration::from_secs(3),
        )
        .await
        .unwrap()
        .connect()
        .await
        .unwrap();

        let mut cube2 = CoreCube::<BleCubeInterface>::new();
        let result = cube2
            .scan(
                Some(BDAddr::from(TEST_CUBE_BDADDR)),
                Some(TEST_CUBE_NAME.to_string()),
                Duration::from_secs(3),
            )
            .await;
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
        _teardown();
    }

    #[tokio::test]
    async fn cube_notify1() {
        _setup();
        let cube_arc = Arc::new(tokio::sync::RwLock::new(CoreCube::<BleCubeInterface>::new()));
        let notification_cube = cube_arc.clone();
        let cube = cube_arc.clone();

        // search and connect

        let handler_uuid: Uuid;
        cube.write()
            .await
            .scan(None, None, Duration::from_secs(3))
            .await
            .unwrap()
            .connect()
            .await
            .unwrap();
        println!("** connection established");

        // register notify handler

        handler_uuid = cube
            .write()
            .await
            .register_notify_handler(Box::new(&notify_handler))
            .await
            .unwrap();
        info!("notify handler uuid {:?}", handler_uuid);

        // start to receive notifications from cube

        let notification_receiver = async move {
            notification_cube
                .read()
                .await
                .create_notification_receiver()
                .unwrap()
                .await;
        };
        let notification_task = tokio::spawn(notification_receiver);

        // wait until Ctrl-C is pressed
        let timer = async {
            time::sleep(Duration::from_secs(8)).await;
        };
        timer.await;
        notification_task.abort();

        println!("** disconnecting now");

        if cube
            .write()
            .await
            .unregister_notify_handler(handler_uuid)
            .await
            .is_err()
        {
            panic!();
        }
        if cube.write().await.disconnect().await.is_err() {
            panic!()
        }
        drop(cube);

        // wait to complete the disconnection process of the cube

        time::sleep(Duration::from_secs(8)).await;
        _teardown();
    }
}

