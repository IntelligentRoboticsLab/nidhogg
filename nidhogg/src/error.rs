use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
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
    #[diagnostic()]
    MsgPackDecodeError(#[from] rmp_serde::decode::Error),

    #[cfg(feature = "lola")]
    #[error("Failed to encode MessagePack message")]
    #[diagnostic()]
    MsgPackEncodeError(#[from] rmp_serde::encode::Error),

    // todo: fork zmq and impl Display for `zmq_remote_api::RemoteApiError` so we can make it transparant.
    #[cfg(feature = "coppelia")]
    #[error("Failed to connect to Coppelia simulator!")]
    #[diagnostic()]
    CoppeliaConnectError(String),
}
