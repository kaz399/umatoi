//! Simple API

use anyhow::Result;
use crate::characteristic::motor::acceleration::{Acceleration, AngleVelocity, Priority};
use crate::characteristic::motor::def::Period;
use crate::characteristic::motor::target::TargetPosition;
use async_trait::async_trait;

#[async_trait]
pub trait Simple {
    // --------------------------------------------------------------------------------
    // Motor Control

    /// Motor control with specified duration
    async fn go(
        &self,
        left: i16,
        right: i16,
        period_ms: u64,
    ) -> Result<()>;

    /// Motor control with specified target
    async fn go_to(
        &self,
        speed: i16,
        target: TargetPosition,
    ) -> Result<()>;

    /// Motor control with multiple targets
    async fn go_along(
        &self,
        speed: i16,
        target_list: Vec<TargetPosition>,
    ) -> Result<()>;

    /// Motor control with specified acceleration
    async fn accelerate(
        &self,
        accel: Acceleration,
        angle_velocity: AngleVelocity,
        period: Period,
        priority: Priority,
    ) -> Result<()>;

    /// Stop
    async fn stop(&self) -> Result<()>;

    // --------------------------------------------------------------------------------
    // Position ID

    // --------------------------------------------------------------------------------
}
#[cfg(test)]
mod test {
    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn api_simple_test1() {
        _setup();
    }
}
