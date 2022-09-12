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

use async_trait::async_trait;
use btleplug::api::ValueNotification;
use log::error;
use thiserror::Error;
use uuid::Uuid;
// use std::sync::Arc;
// use std::sync::Mutex;

use crate::device_interface::DeviceInterface;

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
pub trait CoreCubeBasicFunction<T>
{
    fn new() -> Self;
    fn get_id(&self) -> Uuid;
    // fn get_device_clone(&self) -> Arc<Mutex<T>>;
    fn get_device_clone(&self) -> &T;
}

pub struct CoreCube<T>
where
    T: DeviceInterface + Default + Sync + Send,
{
    id: Uuid,
    // pub device: Arc<Mutex<T>>,
     pub device: T,
}

impl<T> Default for CoreCube<T>
where
    T: DeviceInterface + Default + Sync + Send,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            // device: Arc::new(Mutex::new(T::default())),
            device: T::default(),
        }
    }
}

#[async_trait]
impl<T> CoreCubeBasicFunction<T> for CoreCube<T>
where
    T: DeviceInterface + Default + Sync + Send,
{
    fn new() -> Self {
        Self::default()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    // fn get_device_clone(&self) -> Arc<Mutex<T>> {
    fn get_device_clone(&self) -> &T {
        &self.device
    }
}
