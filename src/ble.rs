use async_trait::async_trait;
use btleplug::api::Peripheral as _;
use btleplug::platform::Peripheral;
use std::error::Error;
use std::pin::Pin;
use thiserror::Error;
use uuid::Uuid;

use crate::handler::NotifyFunction;

pub struct BleAccessor {
    pub peripheral: Peripheral,
}

#[derive(Error, Debug, PartialEq)]
pub enum BleAccessError {
    #[error("fail to connect: '{0}'")]
    FailToConnect(String),
}

#[async_trait]
pub trait BleAccessorMethod {
    // create
    fn new(peripheral: Peripheral) -> Self;

    // connect to cube
    async fn connect(
        &mut self,
        address: u64,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // disconnect with cube
    async fn disconnect(&mut self) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // read data from specified characteristic
    async fn read(
        &self,
        characteristic: Uuid,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>;

    // write data to specified characteristic
    async fn write(
        &self,
        characteristic: Uuid,
        bytes: &Vec<u8>,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // register handler function to specified notify
    fn register_notify(
        &self,
        characteristic: Uuid,
        handler_func: NotifyFunction,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;
}

#[async_trait]
impl BleAccessorMethod for BleAccessor {
    fn new(peripheral: Peripheral) -> Self {
        Self { peripheral }
    }

    async fn connect(
        &mut self,
        address: u64,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        if self.peripheral.is_connected().await? {
            return Ok(true);
        }
        self.peripheral.connect().await?;
        Ok(true)
    }

    async fn disconnect(&mut self) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        if !self.peripheral.is_connected().await? {
            return Ok(true);
        }
        self.peripheral.disconnect().await?;
        Ok(true)
    }

    async fn read(
        &self,
        characteristic: Uuid,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let data: Vec<u8> = Vec::new();
        Ok(data)
    }

    async fn write(
        &self,
        characteristic: Uuid,
        bytes: &Vec<u8>,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        Ok(true)
    }

    // register handler function to specified notify
    fn register_notify(
        &self,
        characteristic: Uuid,
        handler_func: NotifyFunction,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        Ok(true)
    }
}
