pub(crate) mod command;
pub(crate) mod def;
pub(crate) mod information;

pub use self::command::*;
pub use self::def::*;
pub use self::information::*;

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
