use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::characteristic::NotificationData;
use crate::cube::CoreCubeError;
use crate::device_interface::CubeInterface;
use crate::notification_manager::NotificationManager;
use anyhow::Result;
use async_trait::async_trait;
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType, BDAddr,
};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use log::{debug, error, info};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
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

    pub fn create_notification_receiver(&self, notification_handler: Box<dyn Fn(btleplug::api::ValueNotification) + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let ble_peripheral = self.ble_peripheral.clone();
        Box::pin(async {
            let nf_manager = NotificationManager::<NotificationData>::new();
            let handler_uuid = nf_manager.register(notification_handler).unwrap();
            let _ = ble_notification_receiver(ble_peripheral, &nf_manager).await;
            nf_manager.unregister(handler_uuid).unwrap();
        })
    }

}

pub async fn ble_notification_receiver (
    ble_peripheral: Peripheral,
    notification_manager: &NotificationManager<NotificationData>,
) -> Result<()> {
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
            self.ble_peripheral
                .unsubscribe(&self.ble_characteristics[notified])
                .await?;
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

    async fn write(&self, uuid: Uuid, bytes: &[u8]) -> Result<bool> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral
            .write(characteristic, bytes, WriteType::WithoutResponse)
            .await?;
        Ok(true)
    }

    async fn write_with_response(&self, uuid: Uuid, bytes: &[u8]) -> Result<bool> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral
            .write(characteristic, bytes, WriteType::WithResponse)
            .await?;
        Ok(true)
    }
}

pub struct BleScanner;

impl BleScanner {
    async fn scan_ble(&self, filter: ScanFilter, wait: Duration) -> Result<Vec<BleInterface>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        let mut peripheral_list: Vec<BleInterface> = Vec::new();

        #[derive(Clone)]
        struct RssiBle {
            rssi: i16,
            ble: BleInterface
        }
        let mut rssi_peripheral_hash: HashMap<BDAddr, RssiBle> = HashMap::new();

        if adapter_list.is_empty() {
            error!("No Bluetooth adapters found");
            return Ok(peripheral_list);
        }

        for adapter in adapter_list.iter() {
            println!("Starting scan on {}...", adapter.adapter_info().await?);
            adapter.start_scan(filter.clone()).await?;
            time::sleep(wait).await;
            adapter.stop_scan().await?;
            for (_index, peripheral) in adapter.peripherals().await?.iter().enumerate() {
                // debug!("{} {:?}", index, peripheral);
                if peripheral.is_connected().await? {
                    debug!("skip connected device");
                    continue;
                }
                let properties = peripheral.properties().await?.unwrap();
                for service_uuid in properties.services.iter() {
                    info!("service uuid: {}", service_uuid);
                    if *service_uuid == CoreCubeUuid::Service.uuid() {
                        debug!("found toio core cube: service uuid: {}", service_uuid);
                        let rssi = peripheral.properties().await.unwrap().unwrap().rssi.unwrap();
                        let rssi_ble = RssiBle {
                            rssi,
                            ble: peripheral.clone()

                        };
                        let ble_address = peripheral.properties().await?.unwrap().address;
                        rssi_peripheral_hash.insert(ble_address, rssi_ble);
                    }
                }
            }
            let mut rssi_peripheral_list = Vec::from_iter(rssi_peripheral_hash.values().cloned());
            rssi_peripheral_list.sort_by(|a, b| a.rssi.cmp(&b.rssi));
            for inerface in rssi_peripheral_list.iter() {
                peripheral_list.push(inerface.ble.clone());
            }
        }
        debug!("scan_ble: total {} peripherals found", peripheral_list.len());
        Ok(peripheral_list)
    }

    pub async fn scan(&self, num: usize, wait: Duration) -> Result<Vec<BleInterface>> {
        let mut peripheral_list = self.scan_ble(ScanFilter::default(), wait).await.unwrap();
        peripheral_list.truncate(num);
        if peripheral_list.is_empty() {
            error!("toio core cube is not found");
            return Err(CoreCubeError::CubeNotFound.into());
        }
        debug!("scan: total {} peripherals found", peripheral_list.len());
        Ok(peripheral_list)
    }

    pub async fn scan_with_address(&self, address_list: &[BDAddr], wait: Duration) -> Result<Vec<BleInterface>> {
        let mut matched_peripheral_list: Vec<BleInterface> = Vec::new();
        let peripheral_list = self.scan_ble(ScanFilter::default(), wait).await.unwrap();
        for peripheral in peripheral_list  {
            let properties = peripheral.properties().await?.unwrap();
            if address_list.iter().any(|e: &BDAddr| e == &properties.address) {
                info!("found cube: '{}'", &properties.address);
                matched_peripheral_list.push(peripheral);
            }
        }

        if matched_peripheral_list.is_empty() {
            error!("toio core cube is not found");
            return Err(CoreCubeError::CubeNotFound.into());
        }
        debug!("scan_with_address: total {} peripherals found", matched_peripheral_list.len());
        Ok(matched_peripheral_list)
    }

    pub async fn scan_with_name(&self, name_list: &[&str], wait: Duration) -> Result<Vec<BleInterface>> {
        let mut matched_peripheral_list: Vec<BleInterface> = Vec::new();
        let peripheral_list = self.scan_ble(ScanFilter::default(), wait).await.unwrap();
        for peripheral in peripheral_list  {
            let properties = peripheral.properties().await?.unwrap();
            if let Some(local_name) = properties.local_name {
                if name_list.iter().any(|e| e == &local_name) {
                    info!("found cube: '{}'", &local_name);
                    matched_peripheral_list.push(peripheral);
                }
            }
        }

        if matched_peripheral_list.is_empty() {
            error!("toio core cube is not found");
            return Err(CoreCubeError::CubeNotFound.into());
        }
        debug!("scan_with_name: total {} peripherals found", matched_peripheral_list.len());
        Ok(matched_peripheral_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::characteristic::id_information::{self, IdInformation};
    use std::time::Duration;

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

    #[tokio::test]
    async fn cube_scan1() {
        _setup();
        let scanner = BleScanner;
        let interfaces = scanner.scan(1, Duration::from_secs(5)).await.unwrap();
        assert!(! interfaces.is_empty());
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan2() {
        _setup();
        let scanner = BleScanner;
        let interfaces = scanner.scan_with_address(
            &[BDAddr::from(TEST_CUBE_BDADDR)],
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        assert!(! interfaces.is_empty());
        _teardown();
    }

    #[tokio::test]
    async fn cube_scan3() {
        _setup();
        let scanner = BleScanner;
        let interfaces = scanner.scan_with_name(
            &[TEST_CUBE_NAME],
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        assert!(! interfaces.is_empty());
        _teardown();
    }
}
