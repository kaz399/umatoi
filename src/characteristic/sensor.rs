pub(crate) mod commands;
pub(crate) mod def;
pub(crate) mod events;

pub use self::commands::*;
pub use self::def::*;
pub use self::events::*;

#[cfg(test)]
mod test {
    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sensor() {
        _setup();
    }
}
