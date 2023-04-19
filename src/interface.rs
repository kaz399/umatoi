pub mod ble;

use crate::characteristic::NotificationData;
use crate::notification_manager::HandlerFunction;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::vec::Vec;
use uuid::Uuid;

use btleplug::api::BDAddr;

pub enum CoreCubeNotificationControl {
    Run,
    Pause,
    Quit,
}

#[async_trait]
pub trait CubeInterface {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn disconnect(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    // read data from specified characteristic
    async fn read(
        &self,
        uuid: Uuid,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    // write data to specified characteristic (without response)
    async fn write(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>;

    // write data to specified characteristic (with response)
    async fn write_with_response(
        &self,
        uuid: Uuid,
        bytes: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn create_notification_receiver(
        &self,
        handlers: Box<Vec<HandlerFunction<NotificationData>>>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

#[async_trait]
pub trait CubeScanner {
    async fn scan(
        &self,
        num: usize,
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn scan_with_address(
        &self,
        address_list: &[BDAddr],
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    async fn scan_with_name(
        &self,
        name_list: &[&str],
        wait: Duration,
    ) -> Result<
        Vec<Box<dyn CubeInterface + Send + Sync + 'static>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;
}
