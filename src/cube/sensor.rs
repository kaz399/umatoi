pub mod def;
pub mod magnetic;
pub mod motion;
pub mod posture_angle;
pub mod response;

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
