use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::handler::NotifyFunction;

#[async_trait]
pub trait BleInterface {
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // read data from specified characteristic
    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>;

    // write data to specified characteristic
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &Vec<u8>,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // register handler function to specified notify
    async fn register_notify_handler(
        &self,
        uuid: Uuid,
        func: NotifyFunction,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;
}
