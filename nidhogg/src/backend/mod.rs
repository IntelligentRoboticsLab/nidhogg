//! All supported NAO backends
//!

#[cfg(feature = "coppelia")]
mod coppelia;

#[cfg(feature = "coppelia")]
pub use coppelia::CoppeliaBackend;

#[cfg(feature = "lola")]
mod lola;

#[cfg(feature = "lola")]
pub use lola::LolaBackend;

use std::any::type_name;
use std::thread;
use std::time::Duration;

use tracing::info;

use crate::error::Result;
use crate::NaoBackend;

/// Trait that introduces [`ConnectWithDelay::connect_with_retry`] to a [`NaoBackend`].
pub trait ConnectWithRetryExt: NaoBackend {
    /// Connects to a NAO by trying multiple times with an interval in between.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backend::{LolaBackend, ConnectWithRetryExt}};
    /// use std::time::Duration;
    ///
    /// // Try to connect, potentially retrying 10 times, with a 1 second interval
    /// let mut nao = LolaBackend::connect_with_retry(10, Duration::from_secs(1))
    ///     .expect("Could not connect to the NAO! 😪");
    /// ```
    fn connect_with_retry(retry_count: u32, retry_interval: Duration) -> Result<Self> {
        for i in 0..=retry_count {
            info!(
                "[{}/{}] Connecting to {}",
                i + 1,
                retry_count + 1,
                type_name::<Self>()
            );

            let maybe_backend = Self::connect();

            // We connected or this was the last try
            if maybe_backend.is_ok() || i == retry_count {
                return maybe_backend;
            }

            thread::sleep(retry_interval);
        }

        unreachable!()
    }
}
