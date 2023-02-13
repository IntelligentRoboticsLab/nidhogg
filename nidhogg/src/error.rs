use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Could not connect to LoLA socket")]
    NoLoLAConnection,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    MsgPackDecodeError(#[from] rmp_serde::decode::Error),
    #[error(transparent)]
    MsgPackEncodeError(#[from] rmp_serde::encode::Error),
}
