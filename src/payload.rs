//! convert a struct of cube API to ble payload.

pub trait ToPayload<T> {
    fn to_payload(self) -> Vec<T>;
}
