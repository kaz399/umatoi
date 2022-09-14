//! Simple API

use crate::cube::characteristic_uuid::CoreCubeUuid;
use crate::cube::motor::acceleration::{Acceleration, AngleVelocity, Priority};
use crate::cube::motor::control::{MotorControl, MotorControlWithSpecifiedDuration};
use crate::cube::motor::def::Period;
use crate::cube::motor::target::Target;
use crate::cube::{CoreCube, CoreCubeBasicFunction};
use crate::device_interface::DeviceInterface;
use crate::payload::ToPayload;
use async_trait::async_trait;
use std::error::Error;

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
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specified target
    async fn go_to(
        &self,
        speed: i16,
        target: Target,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with multiple targets
    async fn go_along(
        &self,
        speed: i16,
        target_list: Vec<Target>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specified acceleration
    async fn accelerate(
        &self,
        accel: Acceleration,
        angle_velocity: AngleVelocity,
        period: Period,
        priority: Priority,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Stop
    async fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // --------------------------------------------------------------------------------
    // Position ID

    // --------------------------------------------------------------------------------
}

#[async_trait]
impl<'device_life, T> Simple for CoreCube<'device_life, T>
where
    T: DeviceInterface<'device_life> + Default + Sync + Send + 'static,
{
    async fn go(
        &self,
        left: i16,
        right: i16,
        period_ms: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match period_ms {
            0 => {
                let motor: MotorControl = MotorControl::from_primitive(left, right)?;
                self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
                    .await?;
            }
            _ => {
                let motor: MotorControlWithSpecifiedDuration =
                    MotorControlWithSpecifiedDuration::from_primitive(left, right, period_ms)?;
                self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
                    .await?;
            }
        };
        Ok(())
    }

    async fn go_to(
        &self,
        _speed: i16,
        _target: Target,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }

    async fn go_along(
        &self,
        _speed: i16,
        _target_list: Vec<Target>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }

    async fn accelerate(
        &self,
        _accel: Acceleration,
        _angle_velocity: AngleVelocity,
        _period: Period,
        _priority: Priority,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.go(0, 0, 0).await?;
        Ok(())
    }
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
