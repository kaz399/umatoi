pub mod ble;

use async_trait::async_trait;
use btleplug::api::BDAddr;
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

pub enum CoreCubeNotifyControl {
    Run,
    Pause,
    Quit,
}


#[async_trait]
pub trait DeviceInterface {
    type DevicePeripheral;

    fn new() -> Self;

    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // read data from specified characteristic
    async fn read(&self, uuid: Uuid) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>;

    // write data to specified characteristic
    async fn write(
        &self,
        uuid: Uuid,
        bytes: Vec<u8>,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>;

    // scan
    async fn scan(
        &mut self,
        address: Option<BDAddr>,
        device_name: Option<String>,
        wait: Duration,
    ) -> Result<&mut Self, Box<dyn Error + Send + Sync + 'static>>;

    fn get_peripheral(&self) -> Option<Self::DevicePeripheral>;
}
