//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

mod response;

pub mod acceleration;
pub mod control;
pub mod def;
pub mod speed;
pub mod target;

pub use crate::characteristic::motor::response::MotorResponse;

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
