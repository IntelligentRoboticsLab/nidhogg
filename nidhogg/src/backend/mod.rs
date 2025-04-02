//! # Supported NAO backends
//!
//! This module provides support for various NAO backends.
//! It also includes several traits that enhance the functionality of types that implement [`NaoBackend`].

#[cfg(feature = "lola")]
mod lola;
pub use lola::{LolaBackend, LolaControlMsg, LolaNaoState};

use std::any::type_name;
use std::thread;
use std::time::Duration;

use crate::{error::Result, HardwareInfo, NaoBackend};
use tracing::info;

/// Trait that introduces [`ConnectWithRetry::connect_with_retry`] to a type that implements [`NaoBackend`].
pub trait ConnectWithRetry: NaoBackend {
    /// Connects to a NAO by trying multiple times with an interval in between.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backend::{LolaBackend, ConnectWithRetry}};
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
                i,
                retry_count,
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

/// Trait that introduces [`ReadHardwareInfo::read_hardware_info`] to a type that implements [`NaoBackend`].
pub trait ReadHardwareInfo: NaoBackend {
    /// Reads the [`HardwareInfo`] of the NAO.
    ///
    /// The hardware info includes serial numbers and versions of the physical parts, which can be useful for finding out which robot you're connected to!
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backend::{LolaBackend, ReadHardwareInfo}};
    /// use std::time::Duration;
    ///
    /// let mut nao = LolaBackend::connect().unwrap();
    ///
    /// nao.read_hardware_info().expect("Failed to get hardware info!");
    /// ```
    fn read_hardware_info(&mut self) -> Result<HardwareInfo>;
}
