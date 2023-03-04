//! Abstraction layer on top of the LoLA socket for RoboCup SPL NAO V6 robots.  
//!
pub mod backend;
mod error;
mod lola;
pub mod types;

use std::{
    io::{BufWriter, Read},
    os::unix::net::UnixStream,
    thread,
    time::Duration,
};

use lola::{RawState, RawUpdate};

pub use error::{Error, Result};
pub use types::{HardwareInfo, State, Update};

pub trait NaoRobot {
    // todo: don't think this is the way to go...
    type Backend;

    fn connect() -> Result<Self::Backend>;

    fn connect_retry(retry_count: usize, retry_interval: Duration) -> Result<Self::Backend> {
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

            thread::sleep(retry_interval);
        }

        unreachable!()
    }

    fn write_update(&mut self, update: Update) -> Result<()>;
    fn read_state(&mut self) -> Result<State>;
}
