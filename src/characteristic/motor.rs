//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

pub mod commands;
pub mod def;
pub mod events;

pub use self::commands::*;
pub use self::def::*;
pub use self::events::*;

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
