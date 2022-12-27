pub mod battery;
pub mod button;
pub mod characteristic_uuid;
pub mod configuration;
pub mod id_information;
pub mod indicator;
pub mod motor;
pub mod sensor;
pub mod sound;
pub mod tilt;

use crate::notification_manager::HandlerFunction;
use btleplug::api::ValueNotification;
use log::error;
use thiserror::Error;

pub type NotificationData = ValueNotification;
pub type NotificationHandler = HandlerFunction<NotificationData>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreCubeError {
    #[error("toio core cube is not found")]
    CubeNotFound,
    #[error("no bluetooth peripherals")]
    WrongParameter,
    #[error("wrong parameter")]
    NoBlePeripherals,
    #[error("inteface is not defined")]
    NoInterface,
    #[error("internal error of cube.rs")]
    FoundBug,
}
