use crate::characteristic::battery::BatteryInformation;
use crate::characteristic::button::ButtonInformation;
use crate::characteristic::id::IdInformation;
use crate::characteristic::motor::events::MotorInformation;
use crate::characteristic::sensor::events::SensorInformation;
use crate::characteristic::CoreCubeUuid;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CubeEvent {
    Id(IdInformation),
    Button(ButtonInformation),
    Battery(BatteryInformation),
    Sensor(SensorInformation),
    Motor(MotorInformation),
    // Configuration responses can be added here as needed
    Unknown { uuid: Uuid, data: Vec<u8> },
}

impl CubeEvent {
    pub fn from_notification(uuid: Uuid, data: &[u8]) -> Self {
        if uuid == CoreCubeUuid::IdInfo.uuid() {
            if let Some(info) = crate::payload::FromPayload::from_payload(data) {
                return CubeEvent::Id(info);
            }
        } else if uuid == CoreCubeUuid::ButtonInfo.uuid() {
            if let Some(info) = ButtonInformation::new(data) {
                return CubeEvent::Button(info);
            }
        } else if uuid == CoreCubeUuid::BatteryInfo.uuid() {
            if let Some(info) = crate::payload::FromPayload::from_payload(data) {
                return CubeEvent::Battery(info);
            }
        } else if uuid == CoreCubeUuid::SensorInfo.uuid() {
            if let Some(info) = crate::payload::FromPayload::from_payload(data) {
                return CubeEvent::Sensor(info);
            }
        } else if uuid == CoreCubeUuid::MotorCtrl.uuid() {
            if let Some(info) = crate::payload::FromPayload::from_payload(data) {
                return CubeEvent::Motor(info);
            }
        }

        CubeEvent::Unknown {
            uuid,
            data: data.to_vec(),
        }
    }
}
