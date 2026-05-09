use crate::api::event::CubeEvent;
use crate::characteristic::CoreCubeUuid;
use crate::notification_manager::NotificationManager;
use async_trait::async_trait;
use btleplug::api::{
    BDAddr, Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    WriteType,
};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::{BoxStream, StreamExt};
use log::{debug, error};
use std::collections::HashMap;
use std::time::Duration;
use std::vec::Vec;
use tokio::time;
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use super::{CubeInterface, CubeScanner};

type BleInterface = Peripheral;

#[derive(Clone)]
pub struct BleCube {
    pub ble_peripheral: BleInterface,
    pub ble_characteristics: HashMap<Uuid, Characteristic>,
    pub notification_enabled: Vec<Uuid>,
    pub event_manager: NotificationManager<CubeEvent>,
    pub cancel_token: CancellationToken,
}

impl BleCube {
    pub fn new(peripheral: Peripheral) -> Self {
        Self {
            ble_peripheral: peripheral,
            ble_characteristics: HashMap::new(),
            notification_enabled: Vec::new(),
            event_manager: NotificationManager::new(100),
            cancel_token: CancellationToken::new(),
        }
    }
}

pub async fn ble_notification_loop(
    ble_peripheral: Peripheral,
    event_manager: NotificationManager<CubeEvent>,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut notification_stream = ble_peripheral.notifications().await?;
    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                debug!("Notification loop cancelled");
                break;
            }
            Some(data) = notification_stream.next() => {
                let event = CubeEvent::from_notification(data.uuid, &data.value);
                let _ = event_manager.invoke_all_handlers(event);
            }
            else => break,
        }
    }
    Ok(())
}

#[async_trait]
impl CubeInterface for BleCube {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.ble_peripheral.connect().await?;
        let is_connected = self.ble_peripheral.is_connected().await?;
        assert!(is_connected);
        self.ble_peripheral.discover_services().await?;
        for service in self.ble_peripheral.services() {
            for characteristic in service.characteristics {
                if characteristic.properties.contains(CharPropFlags::NOTIFY) {
                    self.notification_enabled.push(characteristic.uuid);
                    debug!("enable notification uuid: {:?}", characteristic.uuid);
                    self.ble_peripheral.subscribe(&characteristic).await?;
                }
                self.ble_characteristics
                    .insert(characteristic.uuid, characteristic);
            }
        }

        // Reset cancel token for a new connection
        self.cancel_token = CancellationToken::new();

        // Start notification loop
        let ble_peripheral = self.ble_peripheral.clone();
        let event_manager = self.event_manager.clone();
        let cancel_token = self.cancel_token.clone();
        tokio::spawn(async move {
            if let Err(e) = ble_notification_loop(ble_peripheral, event_manager, cancel_token).await
            {
                error!("Notification loop error: {}", e);
            }
        });

        Ok(())
    }

    async fn disconnect(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.cancel_token.cancel();
        for notified in &self.notification_enabled {
            debug!("disable notification uuid: {:?}", notified);
            if let Some(characteristic) = self.ble_characteristics.get(notified) {
                let _ = self.ble_peripheral.unsubscribe(characteristic).await;
            }
        }
        self.ble_peripheral.disconnect().await?;
        self.ble_characteristics.clear();
        self.notification_enabled.clear();
        Ok(())
    }

    async fn read(
        &self,
        uuid: Uuid,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        let data = self.ble_peripheral.read(characteristic).await?;
        Ok(data)
    }

    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral
            .write(characteristic, bytes, WriteType::WithoutResponse)
            .await?;
        Ok(true)
    }

    async fn write_with_response(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let characteristic = self.ble_characteristics.get(&uuid).unwrap();
        self.ble_peripheral
            .write(characteristic, bytes, WriteType::WithResponse)
            .await?;
        Ok(true)
    }

    async fn event_stream(
        &self,
    ) -> Result<BoxStream<'static, CubeEvent>, Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let receiver = self.event_manager.subscribe();
        let stream = BroadcastStream::new(receiver).filter_map(|res| async move { res.ok() });
        Ok(stream.boxed())
    }
}

pub struct BleScanner;

impl BleScanner {
    async fn scan_ble(
        &self,
        filter: ScanFilter,
        wait: Duration,
    ) -> Result<Vec<Box<BleCube>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        let mut peripheral_list: Vec<Box<BleCube>> = Vec::new();

        #[derive(Clone)]
        struct RssiBle {
            rssi: i16,
            ble: BleInterface,
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
                if peripheral.is_connected().await? {
                    debug!("skip connected device");
                    continue;
                }
                let properties = peripheral.properties().await?.unwrap();
                for service_uuid in properties.services.iter() {
                    if *service_uuid == CoreCubeUuid::Service.uuid() {
                        let rssi = properties.rssi.unwrap_or(0);
                        let rssi_ble = RssiBle {
                            rssi,
                            ble: peripheral.clone(),
                        };
                        let ble_address = properties.address;
                        rssi_peripheral_hash.insert(ble_address, rssi_ble);
                    }
                }
            }
            let mut rssi_peripheral_list = Vec::from_iter(rssi_peripheral_hash.values().cloned());
            rssi_peripheral_list.sort_by(|a, b| b.rssi.cmp(&a.rssi)); // Sort by RSSI descending
            for interface in rssi_peripheral_list.iter() {
                peripheral_list.push(Box::new(BleCube::new(interface.ble.clone())));
            }
        }
        Ok(peripheral_list)
    }
}

#[async_trait]
impl CubeScanner for BleScanner {
    async fn scan(
        &self,
        num: usize,
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let peripheral_list = self.scan_ble(ScanFilter::default(), wait).await?;
        let mut matched_peripheral_list: Vec<Box<dyn CubeInterface + Send + Sync + 'static>> =
            Vec::new();
        for (n, cube) in peripheral_list.into_iter().enumerate() {
            if n < num {
                matched_peripheral_list.push(cube)
            } else {
                break;
            }
        }
        Ok(matched_peripheral_list)
    }

    async fn scan_with_address(
        &self,
        address_list: &[BDAddr],
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let mut matched_peripheral_list: Vec<Box<dyn CubeInterface + Send + Sync + 'static>> =
            Vec::new();
        let peripheral_list = self.scan_ble(ScanFilter::default(), wait).await?;
        for cube in peripheral_list {
            let properties = cube.ble_peripheral.properties().await?.unwrap();
            if address_list
                .iter()
                .any(|e: &BDAddr| e == &properties.address)
            {
                matched_peripheral_list.push(cube);
            }
        }
        Ok(matched_peripheral_list)
    }

    async fn scan_with_name(
        &self,
        name_list: &[&str],
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let mut matched_peripheral_list: Vec<Box<dyn CubeInterface + Send + Sync + 'static>> =
            Vec::new();
        let peripheral_list = self.scan_ble(ScanFilter::default(), wait).await?;
        for cube in peripheral_list {
            let properties = cube.ble_peripheral.properties().await?.unwrap();
            if let Some(local_name) = properties.local_name {
                if name_list.iter().any(|e| e == &local_name) {
                    matched_peripheral_list.push(cube);
                }
            }
        }
        Ok(matched_peripheral_list)
    }
}
