use super::super::def::command_id_def::CommandId;
use super::super::def::sensor_def::{
    MagnetFunction, MagnetNotificationCondition, PostureAngleNotificationCondition,
};
use crate::characteristic::sensor::def::posture_angle_def::PostureDataType;
use crate::payload::ToPayload;

/// Horizontal detection threshold setting
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#horizontal-detection-threshold-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetHorizontalDetectionThreshold {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub threshold: u8,
}

impl SetHorizontalDetectionThreshold {
    pub fn new(threshold: u8) -> Self {
        Self {
            configuration_type: CommandId::HorizontalDetectionThreshold,
            _reserved: 0,
            threshold,
        }
    }
}

impl ToPayload<Vec<u8>> for SetHorizontalDetectionThreshold {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![
            self.configuration_type.into(),
            self._reserved,
            self.threshold,
        ];
        payload
    }
}

/// Collision detection threshold settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#collision-detection-threshold-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetCollisionDetectionThreshold {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub threshold: u8,
}

impl SetCollisionDetectionThreshold {
    pub fn new(threshold: u8) -> Self {
        Self {
            configuration_type: CommandId::CollisionDetectionThreshold,
            _reserved: 0,
            threshold,
        }
    }
}

impl ToPayload<Vec<u8>> for SetCollisionDetectionThreshold {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![
            self.configuration_type.into(),
            self._reserved,
            self.threshold,
        ];
        payload
    }
}

/// Double tap detection time interval settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#double-tap-detection-time-interval-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetDoubleTapDetectionTimeInterval {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub interval: u8,
}

impl SetDoubleTapDetectionTimeInterval {
    pub fn new(interval: u8) -> Self {
        Self {
            configuration_type: CommandId::CollisionDetectionThreshold,
            _reserved: 0,
            interval,
        }
    }
}

impl ToPayload<Vec<u8>> for SetDoubleTapDetectionTimeInterval {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![
            self.configuration_type.into(),
            self._reserved,
            self.interval,
        ];
        payload
    }
}

/// Set magnetic sensor
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#magnetic-sensor-settings->

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetMagneticSensor {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub function_type: MagnetFunction,
    pub interval: u8,
    pub condition: MagnetNotificationCondition,
}

impl SetMagneticSensor {
    pub fn new(
        function_type: MagnetFunction,
        interval: u8,
        condition: MagnetNotificationCondition,
    ) -> Self {
        Self {
            configuration_type: CommandId::MagneticSensor,
            _reserved: 0,
            function_type,
            interval,
            condition,
        }
    }
}

impl ToPayload<Vec<u8>> for SetMagneticSensor {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![
            self.configuration_type.into(),
            self._reserved,
            self.function_type.into(),
            self.interval,
            self.condition.into(),
        ];
        payload
    }
}

/// Posture angle detection settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#posture-angle-detection-settings->

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetPostureAngleDetection {
    pub configuration_type: CommandId,
    pub _reserved: u8,
    pub data_type: PostureDataType,
    pub interval: u8,
    pub condition: PostureAngleNotificationCondition,
}

impl SetPostureAngleDetection {
    pub fn new(
        data_type: PostureDataType,
        interval: u8,
        condition: PostureAngleNotificationCondition,
    ) -> Self {
        Self {
            configuration_type: CommandId::PostureAngleDetection,
            _reserved: 0,
            data_type,
            interval,
            condition,
        }
    }
}

impl ToPayload<Vec<u8>> for SetPostureAngleDetection {
    fn to_payload(self) -> Vec<u8> {
        let payload: Vec<u8> = vec![
            self.configuration_type.into(),
            self._reserved,
            self.data_type.into(),
            self.interval,
            self.condition.into(),
        ];
        payload
    }
}
