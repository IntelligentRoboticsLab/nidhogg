// TODO: disallow missing docs
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(unreachable_pub, rust_2018_idioms)]

//! A high level abstraction layer for interfacing with NAO V6 robots.
//!
//! ## About
//!
//! ## Backends
//! nidhogg works by connecting to a backend that implements the [`NaoBackend`] trait.
//!
//! Backends can be enabled with features, by default the `lola` feature is enabled.
//!
//! | Backend | Supported | Feature name |
//! |-|-|-|
//! | `LoLA` | ✅ | `lola` |
//! | `CoppeliaSim` | 🚧 | `coppelia` |
//!
//! ✅: Fully supported!  
//! 🚧: Work in progress
//!
//! # Example
//! ```no_run
//! use nidhogg::{
//!     backends::LolaBackend,
//!     NaoBackend,
//! };
//!
//! // We use the LoLA backend to connect to a LoLA socket on a real NAO V6.
//! let mut nao = LolaBackend::connect().unwrap();
//!
//! // We can now get the current state of the robot!
//! let state = nao.read_nao_state().expect("Failed to retrieve sensor data!");
//! ```
//!

pub mod backend;
mod error;
pub mod types;

pub use error::*;
use nidhogg_derive::Builder;
use serde::Serialize;
use types::{
    Battery, Color, ForceSensitiveResistors, JointArray, LeftEar, LeftEye, RightEar, RightEye,
    Skull, SonarEnabled, SonarValues, Touch, Vector2, Vector3,
};

/// Generic backend trait used for implementing a NAO interface.
pub trait NaoBackend: Sized {
    /// Connects to a NAO backend
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backends::LolaBackend};
    ///
    /// // We connect to a real NAO using the LoLA backend
    /// let mut nao = LolaBackend::connect().expect("Could not connect to the NAO! 😪");
    /// ```
    fn connect() -> Result<Self>;

    /// Converts a control message to the format required by the backend and writes it to that backend.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, NaoControlMessage, backends::LolaBackend, types::Color};
    ///
    /// let mut nao = LolaBackend::connect().unwrap();
    ///
    /// // First, create a new control message where we set the chest color
    /// let msg = NaoControlMessage::builder().chest(Color::new(0.8, 0.2, 0.5)).build();
    ///
    /// // Now we send it to the NAO!
    /// nao.send_control_msg(msg).expect("Failed to write control message to backend!");
    /// ```
    fn send_control_msg(&mut self, update: NaoControlMessage) -> Result<()>;

    /// Reads the current sensor data from the chosen backend
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backends::LolaBackend};
    ///
    /// let mut nao = LolaBackend::connect().unwrap();
    ///
    /// // Get the current state of the robot
    /// let state = nao.read_nao_state().expect("Failed to retrieve sensor data!");
    /// ```
    fn read_nao_state(&mut self) -> Result<NaoState>;
}


/// High level representation of the `LoLA` state message.
#[derive(Debug, Clone, Serialize)]
pub struct NaoState {
    pub position: JointArray<f32>,
    pub stiffness: JointArray<f32>,
    pub accelerometer: Vector3<f32>,
    pub gyroscope: Vector3<f32>,
    pub angles: Vector2<f32>,
    pub sonar: SonarValues,
    pub force_sensitive_resistors: ForceSensitiveResistors,
    pub touch: Touch,

    // Diagnostics
    pub battery: Battery,
    pub temperature: JointArray<f32>,
    pub current: JointArray<f32>,
    pub status: JointArray<i32>,
}

/// High level representation of the `LoLA` update message.
#[derive(Builder, Clone, Debug, Default)]
pub struct NaoControlMessage {
    pub position: JointArray<f32>,
    pub stiffness: JointArray<f32>,
    pub sonar: SonarEnabled,

    // LEDs
    pub left_ear: LeftEar,
    pub right_ear: RightEar,
    pub chest: Color,
    pub left_eye: LeftEye,
    pub right_eye: RightEye,
    pub left_foot: Color,
    pub right_foot: Color,
    pub skull: Skull,
}

/// Struct containing the hardware identifiers for the NAO V6 robot.
#[cfg(feature = "lola")]
#[derive(Debug)]
pub struct HardwareInfo {
    pub body_id: String,
    pub body_version: String,
    pub head_id: String,
    pub head_version: String,
}
