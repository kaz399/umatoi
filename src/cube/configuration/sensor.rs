
use super::def::ConfigurationType;
use crate::cube::sensor::posture_angle::PostureDataType;
use serde::Serializer;
use serde::Serialize;

/// Horizontal detection threshold setting
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#horizontal-detection-threshold-settings>

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetHorizontalDetectionThreshold {
    configuration_type: ConfigurationType,
    _reserved: u8,
    threshold: u8,
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
pub struct SetCollisionDetectionTreshold {
    configuration_type: ConfigurationType,
    _reserved: u8,
    threshold: u8,
}

impl SetCollisionDetectionTreshold {
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
    configuration_type: ConfigurationType,
    _reserved: u8,
    interval: u8,
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
    configuration_type: ConfigurationType,
    _reserved: u8,
    function_type: MagnetFunction,
    interval: u8,
    condition: MagnetNotificationCondition,
}

impl SetMagneticSensor {
    pub fn new(function_type: MagnetFunction, interval: u8, condition: MagnetNotificationCondition) -> Self {
        Self {
            configuration_type: ConfigurationType::MagneticSensor,
            _reserved: 0,
            function_type,
            interval,
            condition,
        }
    }
}

/// Magnet function

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MagnetFunction {
    Disable,
    MagnetState,
    MagnetForce,
}

impl From<MagnetFunction> for u8 {
    fn from(function_type: MagnetFunction) -> u8 {
        match function_type {
            MagnetFunction::Disable => 0x00u8,
            MagnetFunction::MagnetState => 0x01u8,
            MagnetFunction::MagnetForce => 0x02u8,
        }
    }
}

impl Serialize for MagnetFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Magnet condition

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MagnetNotificationCondition {
    Always,
    AtChange,
}

impl From<MagnetNotificationCondition> for u8 {
    fn from(function_type: MagnetNotificationCondition) -> u8 {
        match function_type {
            MagnetNotificationCondition::Always => 0x00u8,
            MagnetNotificationCondition::AtChange => 0x01u8,
        }
    }
}

impl Serialize for MagnetNotificationCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let byte_string: u8 = u8::from(*self);
        serializer.serialize_u8(byte_string)
    }
}

/// Response to magnetic sensor settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#responses-to-magnetic-sensor-settings>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResponseMagneticSensorData {
    result: bool,
}

impl ResponseMagneticSensorData {
    pub fn new(byte_data: Vec<u8>) -> Option<Self> {
        if byte_data.len() < 3 {
            return None;
        }
        if byte_data[0] == ConfigurationType::MagneticSensor.response() {
            Some(Self {
                result: byte_data[2] == 0x00u8,
            })
        } else {
            None
        }
    }
}

/// Posture angle detection settings
/// ref:<https://toio.github.io/toio-spec/en/docs/ble_configuration#posture-angle-detection-settings->

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetPostureAngleDetection {
    configuration_type: ConfigurationType,
    _reserved: u8,
    data_type: PostureDataType,
    interval: u8,
    condition: PostureAngleNotificationCondition,
}

impl SetPostureAngleDetection {
    pub fn new(data_type: PostureDataType, interval: u8, condition: PostureAngleNotificationCondition) -> Self {
        Self {
            configuration_type: ConfigurationType::PostureAngleDetection,
            _reserved: 0,
            data_type,
            interval,
            condition,
        }
    }
}


/// Posture notify condition

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PostureAngleNotificationCondition {
    Always,
    AtChange,
}

impl From<PostureAngleNotificationCondition> for u8 {
    fn from(condition: PostureAngleNotificationCondition) -> u8 {
        match condition {
            PostureAngleNotificationCondition::Always => 0x00u8,
            PostureAngleNotificationCondition::AtChange => 0x01u8,
        }
    }
}
