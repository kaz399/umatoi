use crate::handler::NotifyManager;
use btleplug::platform::Peripheral;
use log::{debug, error};
use futures;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc;
use thiserror::Error;
use btleplug::api::{
    BDAddr, Characteristic, Peripheral as _, ValueNotification,
};
use uuid::Uuid;

pub type NotificationData = ValueNotification;

pub enum CoreCubeNotifyControl {
    Run,
    Pause,
    Quit,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreCubeError {
    #[error("toio core cube is not found")]
    CubeNotFound,
    #[error("no bluetooth peripherals")]
    NoBlePeripherals,
    #[error("internal error of cube.rs")]
    FoundBug,
}

pub struct CoreCube {
    pub id: Uuid,
    pub local_name: Option<String>,
    pub address: Option<BDAddr>,
    pub ble_peripheral: Option<Peripheral>,
    pub ble_characteristics: HashMap<Uuid, Characteristic>,
    pub notify_enabled: Vec<Uuid>,
    pub root_notify_manager: NotifyManager<NotificationData>,
}

impl Default for CoreCube {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            local_name: None,
            address: None,
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_enabled: Vec::new(),
            root_notify_manager: NotifyManager::new(),
        }
    }
}

impl<'a> CoreCube {
    pub fn new() -> Self {
        CoreCube::default()
    }

    pub fn new_with_name(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            local_name: Some(name),
            address: None,
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_enabled: Vec::new(),
            root_notify_manager: NotifyManager::new(),
        }
    }

    pub fn new_with_address(addr: BDAddr) -> Self {
        Self {
            id: Uuid::new_v4(),
            local_name: None,
            address: Some(addr),
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_enabled: Vec::new(),
            root_notify_manager: NotifyManager::new(),
        }
    }

    pub fn new_with_name_address(name: String, addr: BDAddr) -> Self {
        Self {
            id: Uuid::new_v4(),
            local_name: Some(name),
            address: Some(addr),
            ble_peripheral: None,
            ble_characteristics: HashMap::new(),
            notify_enabled: Vec::new(),
            root_notify_manager: NotifyManager::new(),
        }
    }

    pub async fn receive_notify(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            let mut notification_stream = ble.notifications().await?.take(1);
            if let Some(data) = notification_stream.next().await {
                self.root_notify_manager.invoke_all_handlers(data)?;
            }
            Ok(())
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }

    pub async fn run_notify_receiver(
        &self,
        rx: mpsc::Receiver<CoreCubeNotifyControl>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ble) = &self.ble_peripheral {
            let mut notification_stream = ble.notifications().await?;
            while let Some(data) = notification_stream.next().await {
                if let Ok(ctrl) = rx.try_recv() {
                    match ctrl {
                        CoreCubeNotifyControl::Quit => break,
                        CoreCubeNotifyControl::Pause => continue,
                        _ => (),
                    }
                }
                self.root_notify_manager.invoke_all_handlers(data)?;
            }
            debug!("stop notify receiver");
            Ok(())
        } else {
            Err(Box::new(CoreCubeError::NoBlePeripherals))
        }
    }
}

