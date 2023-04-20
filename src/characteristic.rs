mod characteristic_uuid;

pub mod battery;
pub mod button;
pub mod configuration;
pub mod id;
pub mod indicator;
pub mod motor;
pub mod sensor;
pub mod sound;

use crate::notification_manager::HandlerFunction;
use btleplug::api::ValueNotification;

pub use crate::characteristic::characteristic_uuid::CoreCubeUuid;

pub type NotificationData = ValueNotification;
pub type NotificationHandler = HandlerFunction<NotificationData>;
