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

pub type NotificationData = ValueNotification;
pub type NotificationHandler = HandlerFunction<NotificationData>;
