mod acceleration;
mod control;
mod target;

pub use self::acceleration::MotorControlAcceleration;
pub use self::control::{MotorControl, MotorControlWithSpecifiedDuration};
pub use self::target::{MotorControlMultipleTargets, MotorControlTarget};
