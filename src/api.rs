//! Simple API

use crate::cube::motor::acceleration::{Acceleration, AngleVelocity, Priority};
use crate::cube::motor::def::{Period, Velocity};
use crate::cube::motor::target::Target;
use std::error::Error;

pub trait Simple {
    // --------------------------------------------------------------------------------
    // Motor Control

    /// Motor control with specified duration
    fn go(
        left: Velocity,
        right: Velocity,
        period: Period,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specifiled target
    fn go_to(speed: Velocity, target: Target)
        -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with multiple targets
    fn go_along(
        speed: Velocity,
        target_list: Vec<Target>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specified acceleration
    fn accelerate(
        accel: Acceleration,
        angle_velocity: AngleVelocity,
        period: Period,
        priority: Priority,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn stop() -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // --------------------------------------------------------------------------------
    // Position ID

    // --------------------------------------------------------------------------------
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn api_simple_test1() {
        _setup();
    }
}
