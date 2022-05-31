//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

pub mod acceleration;
pub mod control;
pub mod def;
pub mod response;
pub mod target;
pub mod speed;

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn motor_bytedecode1() {
        _setup();
    }
}
