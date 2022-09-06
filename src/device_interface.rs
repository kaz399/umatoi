pub mod ble;

use async_trait::async_trait;
use std::error::Error;
use std::sync::mpsc;
use uuid::Uuid;

pub enum CoreCubeNotifyControl {
    Run,
    Pause,
    Quit,
}

#[async_trait]
pub trait DeviceInterface {
    type NotifyHandler;

    fn new() -> Self;

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

    async fn receive_notify(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // run notify receiver
    async fn run_notify_receiver(
        &self,
        rx: mpsc::Receiver<CoreCubeNotifyControl>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}
