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
use btleplug::api::ValueNotification;
use futures;
use futures::Future;
use log::error;
use std::error::Error;
use std::pin::Pin;
use std::sync::mpsc;
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

    pub async fn connect(
        &'static mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.connect()
    }

    pub async fn disconnect(
        &'static mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.disconnect()
    }

    pub async fn read(
        &'static self,
        uuid: Uuid,
    ) -> Pin<
        Box<dyn Future<Output = Result<Vec<u8>, Box<(dyn Error + Sync + Send + 'static)>>> + Send>,
    > {
        self.device.read(uuid)
    }

    pub async fn write(
        &'static self,
        uuid: Uuid,
        bytes: &'static [u8],
    ) -> Pin<Box<dyn Future<Output = Result<bool, Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.write(uuid, bytes)
    }

    pub async fn register_notify_handler(
        &'static mut self,
        func: T::NotifyHandler,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<uuid::Uuid, Box<(dyn Error + Sync + Send + 'static)>>>
                + Send,
        >,
    > {
        self.device.register_notify_handler(func)
    }

    pub async fn unregister_notify_handler(
        &'static mut self,
        id_handler: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<bool, Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.unregister_notify_handler(id_handler)
    }

    pub async fn receive_notify(
        &'static mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.receive_notify()
    }

    pub async fn run_notify_receiver(
        &'static self,
        rx: mpsc::Receiver<CoreCubeNotifyControl>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<(dyn Error + Sync + Send + 'static)>>> + Send>>
    {
        self.device.run_notify_receiver(rx)
    }
}
