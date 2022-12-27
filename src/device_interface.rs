pub mod ble;

use crate::cube::NotificationHandler;

use async_trait::async_trait;
use std::vec::Vec;
use uuid::Uuid;
use anyhow::Result;


pub enum CoreCubeNotificationControl {
    Run,
    Pause,
    Quit,
}

#[async_trait]
pub trait CubeInterface
{
    async fn connect(&mut self) -> Result<()>;

    async fn disconnect(&mut self) -> Result<()>;

    // read data from specified characteristic
    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>>;

    // write data to specified characteristic (without response)
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool>;

    // write data to specified characteristic (with response)
    async fn write_with_response(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool>;

    // register handler function to specified notify
    async fn register_notification_handler(
        &mut self,
        func: NotificationHandler,
    ) -> Result<uuid::Uuid>;

    // register handler function to specified notify
    async fn unregister_notification_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool>;

    async fn notification_receiver(
        &mut self
    ) -> ();
}
