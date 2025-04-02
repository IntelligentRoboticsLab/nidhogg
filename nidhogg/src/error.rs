use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "lola")]
    #[error("Could not connect to LoLA socket")]
    #[diagnostic(help("- Are you trying to connect to the simulation? This backend only supports real NAOs!
- Are you running the code locally? Connecting with LoLA only works when ran on a NAO!
- Are you using `LoLABackend::connect_with_retry` instead of `LoLABackend::connect`? You might not always get a connection the first time!"))]
    NoLoLAConnection(#[from] std::io::Error),

    #[cfg(feature = "lola")]
    #[error("Failed to decode MessagePack message")]
    MsgPackDecodeError(#[from] rmp_serde::decode::Error),

    #[cfg(feature = "lola")]
    #[error("Failed to encode MessagePack message")]
    MsgPackEncodeError(#[from] rmp_serde::encode::Error),
}
