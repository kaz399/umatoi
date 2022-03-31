use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

#[async_trait]
pub trait BleInterface {
    type NotifyHandler;

    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // read data from specified characteristic
    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>;

    // write data to specified characteristic
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // register handler function to specified notify
    async fn register_notify_handler(
        &mut self,
        func: Self::NotifyHandler,
    ) -> Result<uuid::Uuid, Box<dyn Error + Send + Sync + 'static>>;

    // register handler function to specified notify
    async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;
}
