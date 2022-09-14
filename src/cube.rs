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

use crate::device_interface::DeviceInterface;
use async_trait::async_trait;
use btleplug::api::{BDAddr, ValueNotification};
use futures::future::Future;
use log::error;
use std::error::Error;
use std::marker::PhantomData;
use std::pin::Pin;
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

#[async_trait]
pub trait CoreCubeBasicFunction<'device_life, T>
where
    T: DeviceInterface<'device_life> + Default + Sync + Send + 'static,
{
    fn new() -> Self;
    fn get_id(&self) -> Uuid;
    fn set_nickname(&mut self, nickname: String);
    fn get_nickname(self) -> Option<String>;
    async fn connect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>>;
    async fn disconnect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>>;
    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<(dyn Error + Sync + Send + 'static)>>;
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>>;
    async fn register_notify_handler(
        &mut self,
        func: T::NotificationHandler,
    ) -> Result<uuid::Uuid, Box<(dyn Error + Sync + Send + 'static)>>;
    async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>>;
    async fn receive_notify(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>>;
    fn create_notification_receiver(
        &'device_life self,
    ) -> Result<
        Pin<Box<dyn Future<Output = ()> + Send + 'device_life>>,
        Box<(dyn Error + Sync + Send + 'device_life)>,
    >;
    async fn scan(
        &mut self,
        address: Option<BDAddr>,
        device_name: Option<String>,
        timeout: Duration,
    ) -> Result<&mut Self, Box<(dyn Error + Sync + Send + 'static)>>;
}

pub struct CoreCube<'device_life, T>
where
    T: DeviceInterface<'device_life> + Default + Sync + Send + 'static,
{
    id: Uuid,
    nickname: Option<String>,
    pub(crate) device: T,
    _phantom: PhantomData<&'device_life T>,
}

impl<'device_life, T> Default for CoreCube<'device_life, T>
where
    T: DeviceInterface<'device_life> + Default + Sync + Send + 'static,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nickname: None,
            device: T::default(),
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<'device_life, T> CoreCubeBasicFunction<'device_life, T> for CoreCube<'device_life, T>
where
    T: DeviceInterface<'device_life> + Default + Sync + Send + 'static,
{
    fn new() -> Self {
        Self::default()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);
    }

    fn get_nickname(self) -> Option<String> {
        self.nickname
    }

    async fn connect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.connect().await?;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.disconnect().await?;
        Ok(())
    }

    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<(dyn Error + Sync + Send + 'static)>> {
        let data = self.device.read(uuid).await?;
        Ok(data)
    }

    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>> {
        let result = self.device.write(uuid, bytes).await?;
        Ok(result)
    }

    async fn register_notify_handler(
        &mut self,
        func: T::NotificationHandler,
    ) -> Result<uuid::Uuid, Box<(dyn Error + Sync + Send + 'static)>> {
        let uuid = self.device.register_notify_handler(func).await?;
        Ok(uuid)
    }

    async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<(dyn Error + Sync + Send + 'static)>> {
        let result = self.device.unregister_notify_handler(id_handler).await?;
        Ok(result)
    }

    async fn receive_notify(&mut self) -> Result<(), Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.receive_notify().await?;
        Ok(())
    }

    fn create_notification_receiver(
        &'device_life self,
    ) -> Result<
        Pin<Box<dyn Future<Output = ()> + Send + 'device_life>>,
        Box<(dyn Error + Sync + Send + 'device_life)>,
    > {
        Ok(self.device.create_notification_receiver()?)
    }

    async fn scan(
        &mut self,
        address: Option<BDAddr>,
        device_name: Option<String>,
        timeout: Duration,
    ) -> Result<&mut Self, Box<(dyn Error + Sync + Send + 'static)>> {
        self.device.scan(address, device_name, timeout).await?;
        Ok(self)
    }
}
