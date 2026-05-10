pub mod ble;

use crate::api::event::CubeEvent;
use crate::characteristic::battery::BatteryInformation;
use crate::characteristic::button::ButtonInformation;
use crate::characteristic::id::IdInformation;
use crate::characteristic::motor::events::MotorInformation;
use crate::characteristic::sensor::events::SensorInformation;
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use std::time::Duration;
use std::vec::Vec;
use uuid::Uuid;

use btleplug::api::BDAddr;

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

    // get event stream
    async fn event_stream(
        &self,
    ) -> Result<BoxStream<'static, CubeEvent>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    // get ID stream
    async fn id_stream(
        &self,
    ) -> Result<BoxStream<'static, IdInformation>, Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let stream = self.event_stream().await?;
        Ok(stream
            .filter_map(|e| async move {
                if let CubeEvent::Id(id) = e {
                    Some(id)
                } else {
                    None
                }
            })
            .boxed())
    }

    // get button stream
    async fn button_stream(
        &self,
    ) -> Result<
        BoxStream<'static, ButtonInformation>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let stream = self.event_stream().await?;
        Ok(stream
            .filter_map(|e| async move {
                if let CubeEvent::Button(info) = e {
                    Some(info)
                } else {
                    None
                }
            })
            .boxed())
    }

    // get battery stream
    async fn battery_stream(
        &self,
    ) -> Result<
        BoxStream<'static, BatteryInformation>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let stream = self.event_stream().await?;
        Ok(stream
            .filter_map(|e| async move {
                if let CubeEvent::Battery(info) = e {
                    Some(info)
                } else {
                    None
                }
            })
            .boxed())
    }

    // get sensor stream
    async fn sensor_stream(
        &self,
    ) -> Result<
        BoxStream<'static, SensorInformation>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let stream = self.event_stream().await?;
        Ok(stream
            .filter_map(|e| async move {
                if let CubeEvent::Sensor(info) = e {
                    Some(info)
                } else {
                    None
                }
            })
            .boxed())
    }

    // get motor stream
    async fn motor_stream(
        &self,
    ) -> Result<
        BoxStream<'static, MotorInformation>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let stream = self.event_stream().await?;
        Ok(stream
            .filter_map(|e| async move {
                if let CubeEvent::Motor(info) = e {
                    Some(info)
                } else {
                    None
                }
            })
            .boxed())
    }
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
