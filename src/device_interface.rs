pub mod ble;

use async_trait::async_trait;
use std::vec::Vec;
use uuid::Uuid;

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
}
