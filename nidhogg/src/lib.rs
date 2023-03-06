//! Abstraction layer for interfacing with RoboCup SPL NAO V6 robots.  
//!
pub mod backends;
mod error;
pub mod types;

pub use error::{Error, Result};
pub use types::{HardwareInfo, NaoControlMsg, NaoState};

/// Generic backend trait used for implementing your own NAO interface
pub trait NaoBackend: Sized {
    /// Connects to the Nao backend
    fn connect() -> Result<Self>;

    /// Converts update to the format required by the backend and writes it to that backend.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{backends::LolaBackend, NaoBackend, Update};
    ///
    /// let mut nao = LolaBackend::connect().expect("Failed to connect to LoLA socket!");
    ///
    /// // create a new update
    /// let update = Update::default();
    /// nao.write_update(update).expect("Failed to write update to LoLA socket!");
    /// ```
    fn send_control_msg(&mut self, update: NaoControlMsg) -> Result<()>;

    /// Reads the current [`NaoState`] from the chosen backend
    fn read_nao_state(&mut self) -> Result<NaoState>;
}
