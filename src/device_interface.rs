pub mod ble;

use async_trait::async_trait;
use btleplug::api::BDAddr;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

pub enum CoreCubeNotificationControl {
    Run,
    Pause,
    Quit,
}

#[async_trait]
pub trait DeviceInterface<'device_life> {
    type NotificationHandler: Send + Sync + 'static;

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
        func: Self::NotificationHandler,
    ) -> Result<uuid::Uuid, Box<dyn Error + Send + Sync + 'static>>;

    // register handler function to specified notify
    async fn unregister_notify_handler(
        &mut self,
        id_handler: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    async fn receive_notify(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // run notify receiver
    fn create_notification_receiver(
        &'device_life self,
    ) -> Result<
        Pin<Box<dyn Future<Output = ()> + Send + 'device_life>>,
        Box<dyn Error + Send + Sync + 'device_life>,
    >;

    // scan
    async fn scan(
        &mut self,
        address: Option<BDAddr>,
        device_name: Option<String>,
        wait: Duration,
    ) -> Result<&mut Self, Box<dyn Error + Send + Sync + 'static>>;
}
