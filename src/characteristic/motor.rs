//! Official Specification: <https://toio.github.io/toio-spec/en/docs/ble_motor>

pub mod command;
pub mod def;
pub mod information;

pub use self::command::*;
pub use self::def::*;
pub use self::information::*;

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
