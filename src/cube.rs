use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreCubeError {
    #[error("toio core cube is not found")]
    CubeNotFound,
    #[error("no bluetooth peripherals")]
    WrongParameter,
    #[error("wrong parameter")]
    NoBlePeripherals,
    #[error("inteface is not defined")]
    NoInterface,
    #[error("internal error of cube.rs")]
    FoundBug,
}
