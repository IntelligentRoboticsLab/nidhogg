use std::fmt::Display;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Could not connect to LoLA socket")]
    NoLoLAConnection,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "lola")]
    #[error(transparent)]
    MsgPackDecodeError(#[from] rmp_serde::decode::Error),
    #[cfg(feature = "lola")]
    #[error(transparent)]
    MsgPackEncodeError(#[from] rmp_serde::encode::Error),
    // todo: fork zmq and impl Display for `zmq_remote_api::RemoteApiError` so we can make it
    // todo: transparant
    #[cfg(feature = "coppelia")]
    #[error("Failed to connect to Coppelia simulator!")]
    CoppelliaConnectError(String),
}
