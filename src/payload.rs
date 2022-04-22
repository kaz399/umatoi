//! convert a struct of cube API to ble payload.

pub trait ToPayload<T> {
    fn to_payload(self) -> T;
}

pub trait FromPayload<T> {
    fn from_payload(payload: T) -> Self;
}
