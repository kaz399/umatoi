//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_sensor>

use std::time;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MotionDetectResult {
    Unknown,
    NotDetected,
    Detected,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Posture {
    Unknown = 0,
    Top = 1,
    Bottom = 2,
    Rear = 3,
    Front = 4,
    Right = 5,
    Left = 6,
}

#[derive(Debug, Copy, Clone)]
pub struct SensorInfo {
    pub time: time::Instant,
    pub holizontal: MotionDetectResult,
    pub collision: MotionDetectResult,
    pub double_tap: MotionDetectResult,
    pub posture: Posture,
    pub shaking: usize,
}

impl Default for SensorInfo {
    fn default() -> Self {
        Self {
            time: time::Instant::now(),
            holizontal: MotionDetectResult::Unknown,
            collision: MotionDetectResult::Unknown,
            double_tap: MotionDetectResult::Unknown,
            posture: Posture::Unknown,
            shaking: 0,
        }
    }
}
