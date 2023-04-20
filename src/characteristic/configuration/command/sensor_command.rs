use super::super::def::common_def::ConfigurationType;
use super::super::def::sensor_def::{MagnetFunction, MagnetNotificationCondition, PostureAngleNotificationCondition};
use crate::characteristic::sensor::def::posture_angle_def::PostureDataType;
use serde::Serialize;

/// Horizontal detection threshold setting
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#horizontal-detection-threshold-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetHorizontalDetectionThreshold {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub threshold: u8,
}

impl SetHorizontalDetectionThreshold {
    pub fn new(threshold: u8) -> Self {
        Self {
            configuration_type: ConfigurationType::HorizontalDetectionThreshold,
            _reserved: 0,
            threshold,
        }
    }
}

/// Collision detection threshold settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#collision-detection-threshold-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetCollisionDetectionThreshold {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub threshold: u8,
}

impl SetCollisionDetectionThreshold {
    pub fn new(threshold: u8) -> Self {
        Self {
            configuration_type: ConfigurationType::CollisionDetectionTheshold,
            _reserved: 0,
            threshold,
        }
    }
}

/// Double tap detection time interval settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#double-tap-detection-time-interval-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetDoubleTapDetectionTimeInterval {
    pub configuration_type: ConfigurationType,
    pub _reserved: u8,
    pub interval: u8,
}

impl SetDoubleTapDetectionTimeInterval {
    pub fn new(interval: u8) -> Self {
        Self {
            configuration_type: ConfigurationType::CollisionDetectionTheshold,
            _reserved: 0,
            interval,
        }
    }
}

/// Set magnetic sensor
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#magnetic-sensor-settings->

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetMagneticSensor {
    pub configuration_type: ConfigurationType,
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
            configuration_type: ConfigurationType::MagneticSensor,
            _reserved: 0,
            function_type,
            interval,
            condition,
        }
    }
}

/// Posture angle detection settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#posture-angle-detection-settings->

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetPostureAngleDetection {
    pub configuration_type: ConfigurationType,
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
            configuration_type: ConfigurationType::PostureAngleDetection,
            _reserved: 0,
            data_type,
            interval,
            condition,
        }
    }
}

