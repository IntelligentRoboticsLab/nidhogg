use crate::error::Result;
use crate::types::*;
use std::{
    io::{BufWriter, Read},
    os::unix::net::UnixStream,
};

use crate::NaoRobot;
use rmp_serde::from_slice;
use serde::{Deserialize, Serialize};

const ROBOCUP_PATH: &'static str = "/tmp/robocup";

// todo: Not sure whether i like this approach/naming scheme
pub struct LoLACommunicator {
    stream: UnixStream,
    buffer: [u8; 896],
}

impl NaoRobot for LoLACommunicator {
    type Backend = LoLACommunicator;

    fn connect() -> Result<Self::Backend> {
        let stream = UnixStream::connect(ROBOCUP_PATH)?;

        Ok(LoLACommunicator {
            stream,
            buffer: [0; 896],
        })
    }

    /// Converts update to the format required by LoLA and writes it to the socket.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{Nao, Update};
    ///
    /// let mut nao = Nao::connect().expect("Failed to connect to LoLA socket!");
    /// let update = Update::default();
    ///
    /// nao.write_update(update).expect("Failed to write update to LoLA socket!");
    /// ```
    fn write_update(&mut self, update: Update) -> Result<()> {
        let raw: RawUpdate = update.into();
        let mut writer = BufWriter::new(&mut self.stream);
        rmp_serde::encode::write_named(&mut writer, &raw)?;

        Ok(())
    }

    /// Reads the current [`RawState`] from the LoLA socket and converts it
    /// into the higher level [`State`].
    fn read_state(&mut self) -> Result<State> {
        Ok(self.read_raw()?.into())
    }
}

impl LoLACommunicator {
    /// Reads the hardware info from the current [`RawState`]
    pub fn read_hardware_info(&mut self) -> Result<HardwareInfo> {
        Ok(self.read_raw()?.into())
    }

    /// Read a [`RawState`] from the LoLA socket.
    fn read_raw(&mut self) -> Result<RawState> {
        // TODO: Can we remove hardcoded size?
        // println!("{}", size_of::<RawState>());

        self.stream.read_exact(&mut self.buffer)?;
        Ok(from_slice::<RawState>(&self.buffer)?)
    }
}

/// A trait that provides conversions from Nidhogg data to LoLA data
///
/// ## 🗒️ Note:
/// Like [`From`] does with [`Into`], this trait automatically provides an implementation for [`IntoLoLA`].
trait FromNidhogg<N> {
    fn from_nidhogg(value: N) -> Self;
}

/// A trait that provides conversions from Nidhogg data to LoLA data
///
/// ## ⚠️ Warning:
// This trait gets automatically implemented when implementing [`FromNidhogg`], so you should prefer implementing that.
trait IntoLoLA<L> {
    fn into_lola(self) -> L;
}

/// From<T> for U implies Into<U> for T
/// See: https://doc.rust-lang.org/std/convert/trait.From.html
impl<N, L: FromNidhogg<N>> IntoLoLA<L> for N {
    fn into_lola(self) -> L {
        L::from_nidhogg(self)
    }
}

/// A trait that provides conversions from LoLA data to Nidhogg data
///
/// ## 🗒️ Note:
/// Like [`From`] does with [`Into`], this trait automatically provides an implementation for [`IntoLoLA`].
trait FromLoLA<L> {
    fn from_lola(value: L) -> Self;
}

/// A trait that provides conversions from LoLA data to Nidhogg data
///
/// ## ⚠️ Warning:
// This trait gets automatically implemented when implementing [`FromNidhogg`], so you should prefer implementing that.
trait IntoNidhogg<N> {
    fn into_nidhogg(self) -> N;
}

/// From<T> for U implies Into<U> for T
/// See: https://doc.rust-lang.org/std/convert/trait.From.html
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

impl<T> FromLoLA<[T; 2]> for Sonar<T> {
    fn from_lola(value: [T; 2]) -> Self {
        let [left, right] = value;
        Sonar { left, right }
    }
}

impl<T> FromNidhogg<Sonar<T>> for [T; 2] {
    fn from_nidhogg(value: Sonar<T>) -> Self {
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
pub struct RawState<'a> {
    pub stiffness: [f32; 25],
    pub position: [f32; 25],
    pub temperature: [f32; 25],
    pub current: [f32; 25],
    pub battery: [f32; 4],
    pub accelerometer: [f32; 3],
    pub gyroscope: [f32; 3],
    pub angles: [f32; 2],
    pub sonar: [f32; 2],
    pub f_s_r: [f32; 8],
    pub touch: [f32; 14],
    pub status: [i32; 25],
    #[serde(borrow)]
    pub robot_config: [&'a str; 4],
}

impl From<RawState<'_>> for State {
    fn from(value: RawState) -> Self {
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

impl<'a> From<RawState<'a>> for HardwareInfo {
    fn from(value: RawState<'a>) -> Self {
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
pub struct RawUpdate {
    pub position: [f32; 25],
    pub stiffness: [f32; 25],
    pub r_ear: [f32; 10],
    pub l_ear: [f32; 10],
    pub chest: [f32; 3],
    pub l_eye: [f32; 24],
    pub r_eye: [f32; 24],
    pub l_foot: [f32; 3],
    pub r_foot: [f32; 3],
    pub skull: [f32; 12],
    pub sonar: [bool; 2],
}

impl From<Update> for RawUpdate {
    fn from(value: Update) -> Self {
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
