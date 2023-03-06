use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "lola")]
    #[error("Could not connect to LoLA socket")]
    #[diagnostic(help(
        "Are you using `LoLABackend::connect_retry`? You might not always get a connection the first time!"
    ))]
    NoLoLAConnection(#[from] std::io::Error),

    #[cfg(feature = "lola")]
    #[error("Failed to decode MessagePack message")]
    MsgPackDecodeError(#[from] rmp_serde::decode::Error),

    #[cfg(feature = "lola")]
    #[error("Failed to encode MessagePack message")]
    MsgPackEncodeError(#[from] rmp_serde::encode::Error),

    // todo: fork zmq and impl Display for `zmq_remote_api::RemoteApiError` so we can make it
    // todo: transparant
    #[cfg(feature = "coppelia")]
    #[error("Failed to connect to Coppelia simulator!")]
    CoppelliaConnectError(String),
}
