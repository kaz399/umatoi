pub mod battery;
pub mod button;
pub mod characteristic_uuid;
pub mod configuration;
pub mod id_information;
pub mod indicator;
pub mod motor;
pub mod sensor;
pub mod sound;
pub mod tilt;

use crate::device_interface::{CoreCubeNotifyControl, DeviceInterface};
use btleplug::api::{BDAddr, ValueNotification};
use log::error;
use std::error::Error;
use std::sync::mpsc;
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

pub type NotificationData = ValueNotification;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreCubeError {
    #[error("toio core cube is not found")]
    CubeNotFound,
    #[error("no bluetooth peripherals")]
    WrongParameter,
    #[error("wrong parameter")]
    NoBlePeripherals,
    #[error("internal error of cube.rs")]
    FoundBug,
}

pub struct CoreCube<T>
where
    T: DeviceInterface,
{
    id: Uuid,
    nickname: Option<String>,
    pub(crate) device: T,
}

impl<T> Default for CoreCube<T>
where
    T: DeviceInterface,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nickname: None,
            device: T::new(),
        }
    }
}

impl<T> CoreCube<T>
where
    T: DeviceInterface,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);
    }

    pub fn get_nickname(self) -> Option<String> {
        self.nickname
    }

    pub async fn connect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.connect().await?;
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.disconnect().await?;
        Ok(())
    }

    pub async fn read(
        &self,
        uuid: Uuid,
    ) -> Result<Vec<u8>, Box<(dyn Error + Sync + Send + 'static)>> {
        let data = self.device.read(uuid).await?;
        Ok(data)
    }

    pub async fn write(
        &self,
        uuid: Uuid,
        bytes: &'static [u8],
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>> {
        let result = self.device.write(uuid, bytes).await?;
        Ok(result)
    }

    pub async fn register_notify_handler(
        &mut self,
        func: T::NotifyHandler,
    ) -> Result<uuid::Uuid, Box<(dyn Error + Sync + Send + 'static)>> {
        let uuid = self.device.register_notify_handler(func).await?;
        Ok(uuid)
    }

    pub async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>> {
        let result = self.device.unregister_notify_handler(id_handler).await?;
        Ok(result)
    }

    pub async fn receive_notify(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.receive_notify().await?;
        Ok(())
    }

    pub async fn run_notify_receiver(
        &self,
        rx: mpsc::Receiver<CoreCubeNotifyControl>,
    ) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.run_notify_receiver(rx).await?;
        Ok(())
    }

    pub async fn scan(
        &mut self,
        address: Option<BDAddr>,
        device_name: Option<String>,
        timeout: Duration,
    ) -> Result<&mut Self, Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.scan(address, device_name, timeout).await?;
        Ok(self)
    }
}
