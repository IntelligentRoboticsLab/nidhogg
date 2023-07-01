//! `LoLA` backend that communicates through the socket at `/tmp/robocup`.  
//!

use crate::{
    types::{
        Battery, Color, ForceSensitiveResistorFoot, ForceSensitiveResistors, JointArray, LeftEar,
        LeftEye, RightEar, RightEye, Skull, SonarEnabled, SonarValues, Touch, Vector2, Vector3,
    },
    Error, HardwareInfo, NaoBackend, NaoControlMessage, NaoState, Result,
};
use std::{
    io::{BufWriter, Read},
    os::unix::net::UnixStream,
    thread, time,
};

use rmp_serde::{encode, from_slice};
use serde::{Deserialize, Serialize};
use tracing::info;

const ROBOCUP_SOCKET_PATH: &str = "/tmp/robocup";
const LOLA_BUFFER_SIZE: usize = 896;

/// `LoLA` backend that communicates with a real NAO V6 through the socket at `/tmp/robocup`
#[derive(Debug)]
pub struct LolaBackend(UnixStream);

impl NaoBackend for LolaBackend {
    /// Connects to a NAO backend
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backends::LolaBackend};
    ///
    /// // We connect to a real NAO using the `LoLA` backend
    /// let mut nao = LolaBackend::connect().expect("Could not connect to the NAO! 😪");
    /// ```
    fn connect() -> Result<Self> {
        let stream = UnixStream::connect(ROBOCUP_SOCKET_PATH).map_err(Error::NoLoLAConnection)?;

        Ok(LolaBackend(stream))
    }

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
    fn send_control_msg(&mut self, control_msg: NaoControlMessage) -> Result<()> {
        let raw: LolaControlMsg = control_msg.into();

        // convert to MessagePack and write to the socket in a buffer
        let mut buf = BufWriter::new(&mut self.0);
        encode::write_named(&mut buf, &raw).map_err(Error::MsgPackEncodeError)
    }

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
    fn read_nao_state(&mut self) -> Result<NaoState> {
        let mut buf = [0; LOLA_BUFFER_SIZE];

        Ok(self.read_lola_nao_state(&mut buf)?.into())
    }
}

impl LolaBackend {
    /// Connects to a NAO by trying multiple times with an interval in between.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backends::LolaBackend};
    /// use std::time::Duration;
    ///
    /// // Try to connect, potentially retrying 10 times, with a 1 second interval
    /// let mut nao = LolaBackend::connect_with_retry(10, Duration::from_secs(1))
    ///     .expect("Could not connect to the NAO! 😪");
    /// ```
    pub fn connect_with_retry(retry_count: u32, retry_interval: time::Duration) -> Result<Self> {
        for i in 0..=retry_count {
            info!("[{}/{}] Connecting to LoLA socket", i + 1, retry_count + 1);

            let maybe_backend = Self::connect();

            // We connected or this was the last try
            if maybe_backend.is_ok() || i == retry_count {
                return maybe_backend;
            }

            thread::sleep(retry_interval);
        }

        unreachable!()
    }

    /// Reads the [`HardwareInfo`] of the NAO
    ///
    /// The hardware info includes serial numbers and versions of the physical parts, which can be useful for finding out which robot you're connected to!
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backends::LolaBackend};
    /// use std::time::Duration;
    ///
    /// let mut nao = LolaBackend::connect().unwrap();
    ///
    /// nao.read_hardware_info().expect("Failed to get hardware info!");
    /// ```
    pub fn read_hardware_info(&mut self) -> Result<HardwareInfo> {
        let mut buf = [0; LOLA_BUFFER_SIZE];

        self.read_lola_nao_state(&mut buf).map(LolaNaoState::into)
    }

    /// Read a [`LolaNaoState`] from the `LoLA` socket.
    fn read_lola_nao_state<'a>(
        &mut self,
        buf: &'a mut [u8; LOLA_BUFFER_SIZE],
    ) -> Result<LolaNaoState<'a>> {
        self.0.read_exact(buf).unwrap();
        from_slice::<LolaNaoState<'_>>(buf).map_err(Error::MsgPackDecodeError)
    }
}

/// A trait that provides conversions from nidhogg data to `LoLA` data
///
/// ## 🗒️ Note:
/// Like [`From`] does with [`Into`], this trait automatically provides an implementation for [`IntoLoLA`].
trait FromNidhogg<N> {
    fn from_nidhogg(value: N) -> Self;
}

/// A trait that provides conversions from `nihogg` data to `LoLA` data
///
/// ## ⚠️ Warning:
// This trait gets automatically implemented when implementing [`FromNidhogg`], so you should prefer implementing that.
trait IntoLoLA<L> {
    fn into_lola(self) -> L;
}

/// From<T> for U implies Into<U> for T
/// See: <https://doc.rust-lang.org/std/convert/trait.From.html>
impl<N, L: FromNidhogg<N>> IntoLoLA<L> for N {
    fn into_lola(self) -> L {
        L::from_nidhogg(self)
    }
}

/// A trait that provides conversions from `LoLA` data to nidhogg data
///
/// ## 🗒️ Note:
/// Like [`From`] does with [`Into`], this trait automatically provides an implementation for [`IntoLoLA`].
trait FromLoLA<L> {
    fn from_lola(value: L) -> Self;
}

/// A trait that provides conversions from `LoLA` data to nidhogg data
///
/// ## ⚠️ Warning:
// This trait gets automatically implemented when implementing [`FromNidhogg`], so you should prefer implementing that.
trait IntoNidhogg<N> {
    fn into_nidhogg(self) -> N;
}

/// From<T> for U implies Into<U> for T
/// See: <https://doc.rust-lang.org/std/convert/trait.From.html>
impl<L, N: FromLoLA<L>> IntoNidhogg<N> for L {
    fn into_nidhogg(self) -> N {
        N::from_lola(self)
    }
}

impl FromNidhogg<LeftEar> for [f32; 10] {
    fn from_nidhogg(value: LeftEar) -> Self {
        [
            value.intensity_0_deg,
            value.intensity_36_deg,
            value.intensity_72_deg,
            value.intensity_108_deg,
            value.intensity_144_deg,
            value.intensity_180_deg,
            value.intensity_216_deg,
            value.intensity_252_deg,
            value.intensity_288_deg,
            value.intensity_324_deg,
        ]
    }
}

impl FromNidhogg<RightEar> for [f32; 10] {
    fn from_nidhogg(value: RightEar) -> Self {
        [
            value.intensity_324_deg,
            value.intensity_288_deg,
            value.intensity_252_deg,
            value.intensity_216_deg,
            value.intensity_180_deg,
            value.intensity_144_deg,
            value.intensity_108_deg,
            value.intensity_72_deg,
            value.intensity_36_deg,
            value.intensity_0_deg,
        ]
    }
}

impl FromNidhogg<Color> for [f32; 3] {
    fn from_nidhogg(value: Color) -> Self {
        [value.red, value.green, value.blue]
    }
}

impl FromNidhogg<LeftEye> for [f32; 24] {
    fn from_nidhogg(value: LeftEye) -> Self {
        [
            value.color_45_deg.red,
            value.color_0_deg.red,
            value.color_315_deg.red,
            value.color_270_deg.red,
            value.color_225_deg.red,
            value.color_180_deg.red,
            value.color_135_deg.red,
            value.color_90_deg.red,
            // bad rustfmt
            value.color_45_deg.green,
            value.color_0_deg.green,
            value.color_315_deg.green,
            value.color_270_deg.green,
            value.color_225_deg.green,
            value.color_180_deg.green,
            value.color_135_deg.green,
            value.color_90_deg.green,
            // bad rustfmt
            value.color_45_deg.blue,
            value.color_0_deg.blue,
            value.color_315_deg.blue,
            value.color_270_deg.blue,
            value.color_225_deg.blue,
            value.color_180_deg.blue,
            value.color_135_deg.blue,
            value.color_90_deg.blue,
        ]
    }
}

impl FromNidhogg<RightEye> for [f32; 24] {
    fn from_nidhogg(value: RightEye) -> Self {
        [
            value.color_0_deg.red,
            value.color_45_deg.red,
            value.color_90_deg.red,
            value.color_135_deg.red,
            value.color_180_deg.red,
            value.color_225_deg.red,
            value.color_270_deg.red,
            value.color_315_deg.red,
            // bad rustfmt
            value.color_0_deg.green,
            value.color_45_deg.green,
            value.color_90_deg.green,
            value.color_135_deg.green,
            value.color_180_deg.green,
            value.color_225_deg.green,
            value.color_270_deg.green,
            value.color_315_deg.green,
            // bad rustfmt
            value.color_0_deg.blue,
            value.color_45_deg.blue,
            value.color_90_deg.blue,
            value.color_135_deg.blue,
            value.color_180_deg.blue,
            value.color_225_deg.blue,
            value.color_270_deg.blue,
            value.color_315_deg.blue,
        ]
    }
}

impl FromNidhogg<Skull> for [f32; 12] {
    fn from_nidhogg(value: Skull) -> Self {
        [
            value.left_front_0,
            value.left_front_1,
            value.left_middle_0,
            value.left_rear_0,
            value.left_rear_1,
            value.left_rear_2,
            value.right_rear_2,
            value.right_rear_1,
            value.right_rear_0,
            value.right_middle_0,
            value.right_front_0,
            value.right_front_1,
        ]
    }
}

impl<T> FromLoLA<[T; 25]> for JointArray<T> {
    fn from_lola(value: [T; 25]) -> Self {
        let [head_yaw, head_pitch, left_shoulder_pitch, left_shoulder_roll, left_elbow_yaw, // bad rustfmt
             left_elbow_roll, left_wrist_yaw, left_hip_yaw_pitch, left_hip_roll, left_hip_pitch,
             left_knee_pitch, left_ankle_pitch, left_ankle_roll, right_hip_roll, right_hip_pitch,
             right_knee_pitch, right_ankle_pitch, right_ankle_roll, right_shoulder_pitch, right_shoulder_roll,
             right_elbow_yaw, right_elbow_roll, right_wrist_yaw, left_hand, right_hand] = value;

        Self {
            head_yaw,
            head_pitch,

            left_shoulder_pitch,
            left_shoulder_roll,
            left_elbow_yaw,
            left_elbow_roll,
            left_wrist_yaw,
            left_hip_yaw_pitch,
            left_hip_roll,
            left_hip_pitch,
            left_knee_pitch,
            left_ankle_pitch,
            left_ankle_roll,

            right_hip_roll,
            right_hip_pitch,
            right_knee_pitch,
            right_ankle_pitch,
            right_ankle_roll,
            right_shoulder_pitch,
            right_shoulder_roll,
            right_elbow_yaw,
            right_elbow_roll,
            right_wrist_yaw,

            left_hand,
            right_hand,
        }
    }
}

impl<T> FromNidhogg<JointArray<T>> for [T; 25] {
    fn from_nidhogg(value: JointArray<T>) -> Self {
        [
            value.head_yaw,
            value.head_pitch,
            value.left_shoulder_pitch,
            value.left_shoulder_roll,
            value.left_elbow_yaw,
            value.left_elbow_roll,
            value.left_wrist_yaw,
            value.left_hip_yaw_pitch,
            value.left_hip_roll,
            value.left_hip_pitch,
            value.left_knee_pitch,
            value.left_ankle_pitch,
            value.left_ankle_roll,
            value.right_hip_roll,
            value.right_hip_pitch,
            value.right_knee_pitch,
            value.right_ankle_pitch,
            value.right_ankle_roll,
            value.right_shoulder_pitch,
            value.right_shoulder_roll,
            value.right_elbow_yaw,
            value.right_elbow_roll,
            value.right_wrist_yaw,
            value.left_hand,
            value.right_hand,
        ]
    }
}

impl FromLoLA<[f32; 4]> for Battery {
    fn from_lola(value: [f32; 4]) -> Self {
        Battery {
            charge: value[0],
            current: value[1],
            status: value[2],
            temperature: value[3],
        }
    }
}

impl FromLoLA<[f32; 8]> for ForceSensitiveResistors {
    fn from_lola(value: [f32; 8]) -> Self {
        let left: [f32; 4] = value[..4].try_into().unwrap();
        let right: [f32; 4] = value[4..].try_into().unwrap();

        Self {
            left_foot: left.into_nidhogg(),
            right_foot: right.into_nidhogg(),
        }
    }
}

impl FromLoLA<[f32; 4]> for ForceSensitiveResistorFoot {
    fn from_lola(value: [f32; 4]) -> Self {
        Self {
            front_left: value[0],
            front_right: value[1],
            rear_left: value[2],
            rear_right: value[3],
        }
    }
}

impl FromLoLA<[f32; 2]> for SonarValues {
    fn from_lola(value: [f32; 2]) -> Self {
        let [left, right] = value;
        SonarValues { left, right }
    }
}

impl FromNidhogg<SonarValues> for [f32; 2] {
    fn from_nidhogg(value: SonarValues) -> Self {
        [value.left, value.right]
    }
}

impl FromLoLA<[bool; 2]> for SonarEnabled {
    fn from_lola(value: [bool; 2]) -> Self {
        let [left, right] = value;
        SonarEnabled { left, right }
    }
}

impl FromNidhogg<SonarEnabled> for [bool; 2] {
    fn from_nidhogg(value: SonarEnabled) -> Self {
        [value.left, value.right]
    }
}

impl FromLoLA<[f32; 14]> for Touch {
    fn from_lola(value: [f32; 14]) -> Self {
        Self {
            chest_board: value[0],
            head_front: value[1],
            head_middle: value[2],
            head_rear: value[3],
            left_foot_left: value[4],
            left_foot_right: value[5],
            left_hand_back: value[6],
            left_hand_left: value[7],
            left_hand_right: value[8],
            right_foot_left: value[9],
            right_foot_right: value[10],
            right_hand_back: value[11],
            right_hand_left: value[12],
            right_hand_right: value[13],
        }
    }
}

impl FromLoLA<[f32; 2]> for Vector2<f32> {
    fn from_lola(value: [f32; 2]) -> Self {
        Vector2 {
            x: value[0],
            y: value[1],
        }
    }
}

impl FromLoLA<[f32; 3]> for Vector3<f32> {
    fn from_lola(value: [f32; 3]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LolaNaoState<'a> {
    stiffness: [f32; 25],
    position: [f32; 25],
    temperature: [f32; 25],
    current: [f32; 25],
    battery: [f32; 4],
    accelerometer: [f32; 3],
    gyroscope: [f32; 3],
    angles: [f32; 2],
    sonar: [f32; 2],
    f_s_r: [f32; 8],
    touch: [f32; 14],
    status: [i32; 25],
    #[serde(borrow)]
    robot_config: [&'a str; 4],
}

impl From<LolaNaoState<'_>> for NaoState {
    fn from(value: LolaNaoState<'_>) -> Self {
        Self {
            stiffness: value.stiffness.into_nidhogg(),
            position: value.position.into_nidhogg(),
            temperature: value.temperature.into_nidhogg(),
            current: value.current.into_nidhogg(),
            battery: value.battery.into_nidhogg(),
            accelerometer: value.accelerometer.into_nidhogg(),
            gyroscope: value.gyroscope.into_nidhogg(),
            angles: value.angles.into_nidhogg(),
            sonar: value.sonar.into_nidhogg(),
            force_sensitive_resistors: value.f_s_r.into_nidhogg(),
            touch: value.touch.into_nidhogg(),
            status: value.status.into_nidhogg(),
        }
    }
}

impl From<LolaNaoState<'_>> for HardwareInfo {
    fn from(value: LolaNaoState<'_>) -> Self {
        Self {
            body_id: value.robot_config[0].to_string(),
            body_version: value.robot_config[1].to_string(),
            head_id: value.robot_config[2].to_string(),
            head_version: value.robot_config[3].to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct LolaControlMsg {
    position: [f32; 25],
    stiffness: [f32; 25],
    r_ear: [f32; 10],
    l_ear: [f32; 10],
    chest: [f32; 3],
    l_eye: [f32; 24],
    r_eye: [f32; 24],
    l_foot: [f32; 3],
    r_foot: [f32; 3],
    skull: [f32; 12],
    sonar: [bool; 2],
}

impl From<NaoControlMessage> for LolaControlMsg {
    fn from(value: NaoControlMessage) -> Self {
        Self {
            position: value.position.into_lola(),
            stiffness: value.stiffness.into_lola(),
            r_ear: value.right_ear.into_lola(),
            l_ear: value.left_ear.into_lola(),
            chest: value.chest.into_lola(),
            l_eye: value.left_eye.into_lola(),
            r_eye: value.right_eye.into_lola(),
            l_foot: value.left_foot.into_lola(),
            r_foot: value.right_foot.into_lola(),
            skull: value.skull.into_lola(),
            sonar: value.sonar.into_lola(),
        }
    }
}
