//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

use crate::characteristic::characteristic_uuid::CoreCubeUuid;
use crate::device_interface::CubeInterface;
use anyhow::Result;
pub mod acceleration;
pub mod control;
pub mod def;
mod response;
pub mod speed;
pub mod target;

pub use crate::characteristic::motor::response::MotorResponse;

pub async fn write(interface: &dyn CubeInterface, bytes: &[u8]) -> Result<bool> {
    interface.write(CoreCubeUuid::MotorCtrl.into(), bytes).await
}

#[cfg(test)]
mod test {
    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
    }
}
