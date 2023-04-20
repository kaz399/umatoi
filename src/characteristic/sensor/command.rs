pub(crate) mod magnetic_command;
pub(crate) mod motion_command;
pub(crate) mod posture_angle_command;

pub use self::magnetic_command::RequestMagneticSensor;
pub use self::motion_command::RequestMotionDetection;
pub use self::posture_angle_command::RequestPostureAngleDetection;
