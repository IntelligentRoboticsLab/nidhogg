//! Abstraction layer on top of the LoLA socket for RoboCup SPL NAO V6 robots.  
//!

mod error;
mod lola;
pub mod types;

use std::{
    io::{BufWriter, Read},
    mem::size_of,
    os::unix::net::UnixStream,
    thread,
    time::Duration,
};

use rmp_serde::from_slice;
use tracing::info;

use lola::{RawState, RawUpdate};

pub use error::{Error, Result};
pub use types::{HardwareInfo, State, Update};

/// Wrapper around a [`UnixStream`] containing methods to interact with
/// the LoLA socket on the NAO 6 robot.
pub struct Nao {
    stream: UnixStream,
    buffer: [u8; 896],
}

impl Nao {
    const ROBOCUP_PATH: &'static str = "/tmp/robocup";

    /// Attempt to connect to the LoLA socket.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::Nao;
    ///
    /// let nao = Nao::connect()?;
    /// ```
    pub fn connect() -> Result<Self> {
        let stream = UnixStream::connect(Self::ROBOCUP_PATH)?;

        Ok(Nao {
            stream,
            buffer: [0; 896],
        })
    }

    /// Attempt to connect to the LoLA socket, with a specified amount of retries.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::Nao;
    /// use std::time::Duration;
    ///
    /// let nao = Nao::connect_retry(10, Duration::from_secs(10))?;
    /// ```
    pub fn connect_retry(retry_count: usize, retry_interval: Duration) -> Result<Self> {
        for i in 0..retry_count {
            let try_number = i + 1;

            match Self::connect() {
                sock if sock.is_ok() => {
                    return sock;
                }
                // return the last error if we didn't connect succesfully
                Err(e) if try_number == retry_count => return Err(e),
                _ => (),
            }

            info!("({}/{retry_count}) Connecting to LoLA Socket...", i + 1);
            thread::sleep(retry_interval);
        }

        unreachable!()
    }

    /// Reads the hardware info from the current [`RawState`]
    pub fn read_hardware_info(&mut self) -> Result<HardwareInfo> {
        Ok(self.read_raw()?.into())
    }

    /// Reads the current [`RawState`] from the LoLA socket and converts it
    /// into the higher level [`State`].
    pub fn read_state(&mut self) -> Result<State> {
        Ok(self.read_raw()?.into())
    }

    /// Converts update to the format required by LoLA and writes it to the socket.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg:{Nao, Update};
    ///
    /// let mut nao = Nao::connect()?;
    /// let update = Update::default();
    ///
    /// nao.write_update(update)?;
    /// ```
    pub fn write_update(&mut self, update: Update) -> Result<()> {
        let raw: RawUpdate = update.into();
        let mut writer = BufWriter::new(&mut self.stream);
        rmp_serde::encode::write_named(&mut writer, &raw)?;

        Ok(())
    }

    /// Read a [`RawState`] from the LoLA socket.
    fn read_raw(&mut self) -> Result<RawState> {
        // TODO: Can we remove hardcoded size?
        println!("{}", size_of::<RawState>());

        self.stream.read_exact(&mut self.buffer)?;
        Ok(from_slice::<RawState>(&self.buffer)?)
    }
}
