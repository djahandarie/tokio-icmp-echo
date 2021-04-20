#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid procotol")]
    InvalidProtocol,
    #[error("internal error")]
    InternalError,
    #[error("io error: {0:?}")]
    IoError(#[from] std::io::Error),
}
