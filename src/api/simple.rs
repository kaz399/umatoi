//! Simple API

use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::characteristic::motor::acceleration::{
    Acceleration, AngleVelocity, MotorControlAcceleration, MovingDirection, Priority,
};
use crate::characteristic::motor::control::{MotorControl, MotorControlWithSpecifiedDuration};
use crate::characteristic::motor::def::Period;
use crate::characteristic::motor::target::TargetPosition;
use crate::characteristic::motor::target::{
    MotorControlMultipleTargets, MotorControlTarget, Speed,
};
use crate::device_interface::CubeInterface;
use crate::integer_converter::{i_to_i16, i_to_u8};
use crate::payload::ToPayload;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Simple {
    // --------------------------------------------------------------------------------
    // Motor Control

    /// Motor control with specified duration
    async fn motor_control(
        &self,
        left: isize,
        right: isize,
        period_ms: usize,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specified target
    async fn motor_control_target(
        &self,
        speed: isize,
        target: TargetPosition,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with multiple targets
    async fn motor_control_multiple_targets(
        &self,
        speed: isize,
        target_list: Vec<TargetPosition>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Motor control with specified acceleration
    async fn motor_control_acceleration(
        &self,
        accel: Acceleration,
        angle_velocity: AngleVelocity,
        moving_direction: MovingDirection,
        period_ms: usize,
        priority: Priority,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    /// Stop
    async fn motor_stop(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    // --------------------------------------------------------------------------------
    // Position ID

    // --------------------------------------------------------------------------------
}

#[async_trait]
impl<T: CubeInterface + Send + Sync + 'static> Simple for T {
    async fn motor_control(
        &self,
        left: isize,
        right: isize,
        period_ms: usize,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match period_ms {
            0 => {
                let motor: MotorControl =
                    MotorControl::from_primitive(i_to_i16(left), i_to_i16(right))?;
                self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
                    .await?;
            }
            _ => {
                let motor: MotorControlWithSpecifiedDuration =
                    MotorControlWithSpecifiedDuration::from_primitive(
                        i_to_i16(left),
                        i_to_i16(right),
                        period_ms,
                    )?;
                self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
                    .await?;
            }
        }
        Ok(())
    }

    async fn motor_control_target(
        &self,
        speed: isize,
        target: TargetPosition,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let speed: Speed = Speed {
            max: i_to_u8(speed),
            ..Speed::default()
        };
        let motor: MotorControlTarget = MotorControlTarget {
            speed,
            target,
            ..MotorControlTarget::default()
        };
        self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
            .await?;
        Ok(())
    }

    async fn motor_control_multiple_targets(
        &self,
        speed: isize,
        target_list: Vec<TargetPosition>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let speed: Speed = Speed {
            max: i_to_u8(speed),
            ..Speed::default()
        };
        let motor: MotorControlMultipleTargets = MotorControlMultipleTargets {
            speed,
            target_list,
            ..MotorControlMultipleTargets::default()
        };
        self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
            .await?;
        Ok(())
    }

    async fn motor_control_acceleration(
        &self,
        acceleration: Acceleration,
        angle_velocity: AngleVelocity,
        moving_direction: MovingDirection,
        period_ms: usize,
        priority: Priority,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let period: Period = Period::from_millis(period_ms);
        let motor: MotorControlAcceleration = MotorControlAcceleration {
            acceleration,
            angle_velocity,
            moving_direction,
            priority,
            period,
            ..MotorControlAcceleration::default()
        };
        self.write(CoreCubeUuid::MotorCtrl.uuid(), &motor.to_payload())
            .await?;
        Ok(())
    }

    async fn motor_stop(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.motor_control(0, 0, 0).await?;
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
