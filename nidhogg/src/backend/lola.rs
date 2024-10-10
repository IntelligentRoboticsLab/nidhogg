//! `LoLA` backend that communicates through the socket at `/tmp/robocup`.
//!

use crate::{
    types::{
        Battery, ForceSensitiveResistorFoot, ForceSensitiveResistors, JointArray, LeftEar, LeftEye,
        Rgb, RgbF32, RightEar, RightEye, Skull, SonarEnabled, SonarValues, Touch,
    },
    DisconnectExt, Error, HardwareInfo, NaoBackend, NaoControlMessage, NaoState, Result,
};

use nalgebra::{vector, Vector2, Vector3};

use rmp_serde::{encode, from_slice};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Read, Write},
    os::unix::net::UnixStream,
    time::Duration,
};

use super::{ConnectWithRetry, ReadHardwareInfo};
use std::any::type_name;
use std::thread;
use tracing::info;

const ROBOCUP_SOCKET_PATH: &str = "/tmp/robocup";
const LOLA_BUFFER_SIZE: usize = 896;

/// `LoLA` backend that communicates with a real NAO V6 through the socket at `/tmp/robocup`
#[derive(Debug)]
pub struct LolaBackend(UnixStream);

impl LolaBackend {
    fn connect_with_path(socket_path: &str) -> Result<Self> {
        let stream = UnixStream::connect(socket_path).map_err(Error::NoLoLAConnection)?;

        Ok(LolaBackend(stream))
    }

    pub fn connect_with_path_with_retry(
        retry_count: u32,
        retry_interval: Duration,
        socket_path: &str,
    ) -> Result<Self> {
        for i in 0..=retry_count {
            info!(
                "[{}/{}] Connecting to {}",
                i,
                retry_count,
                type_name::<Self>()
            );

            let maybe_backend = Self::connect_with_path(socket_path);

            // We connected or this was the last try
            if maybe_backend.is_ok() || i == retry_count {
                return maybe_backend;
            }

            thread::sleep(retry_interval);
        }

        unreachable!()
    }
}

impl NaoBackend for LolaBackend {
    /// Connects to a NAO backend
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, backend::LolaBackend};
    ///
    /// // We connect to a real NAO using the `LoLA` backend
    /// let mut nao = LolaBackend::connect().expect("Could not connect to the NAO! 😪");
    /// ```
    fn connect() -> Result<Self> {
        Self::connect_with_path(ROBOCUP_SOCKET_PATH)
    }

    /// Converts a control message to the format required by the backend and writes it to that backend.
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{NaoBackend, NaoControlMessage, backend::LolaBackend, types::color};
    ///
    /// let mut nao = LolaBackend::connect().unwrap();
    ///
    /// // First, create a new control message where we set the chest color
    /// let msg = NaoControlMessage::builder().chest(color::f32::MAGENTA).build();
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
    /// use nidhogg::{NaoBackend, backend::LolaBackend};
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

impl DisconnectExt for LolaBackend {
    /// Disconnects a NAO backend
    ///
    /// # Examples
    /// ```no_run
    /// use nidhogg::{DisconnectExt, NaoBackend, backend::LolaBackend};
    ///
    /// // We connect to a real NAO using the `LoLA` backend
    /// let mut nao = LolaBackend::connect().expect("Could not connect to the NAO! 😪");
    ///
    /// // Now we can disconnect using the [`DisconnectExt`].
    /// nao.disconnect().expect("Failed to shutdown connection!");
    /// ```
    fn disconnect(self) -> Result<()> {
        Ok(self.0.shutdown(std::net::Shutdown::Both)?)
    }
}

impl ConnectWithRetry for LolaBackend {}

impl ReadHardwareInfo for LolaBackend {
    fn read_hardware_info(&mut self) -> Result<HardwareInfo> {
        let mut buf = [0; LOLA_BUFFER_SIZE];

        self.read_lola_nao_state(&mut buf).map(LolaNaoState::into)
    }
}

impl LolaBackend {
    /// Read a [`LolaNaoState`] from the `LoLA` socket.
    fn read_lola_nao_state<'a>(
        &mut self,
        buf: &'a mut [u8; LOLA_BUFFER_SIZE],
    ) -> Result<LolaNaoState<'a>> {
        self.0.read_exact(buf)?;
        from_slice::<LolaNaoState<'_>>(buf).map_err(Error::MsgPackDecodeError)
    }
}

impl Read for LolaBackend {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for LolaBackend {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
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
            value.l0, value.l1, value.l2, value.l3, value.l4, value.l5, value.l6, value.l7,
            value.l8, value.l9,
        ]
    }
}

impl FromNidhogg<RightEar> for [f32; 10] {
    fn from_nidhogg(value: RightEar) -> Self {
        [
            value.r9, value.r8, value.r7, value.r6, value.r5, value.r4, value.r3, value.r2,
            value.r1, value.r0,
        ]
    }
}

impl FromNidhogg<RgbF32> for [f32; 3] {
    fn from_nidhogg(value: RgbF32) -> Self {
        [value.red, value.green, value.blue]
    }
}

impl FromNidhogg<LeftEye> for [f32; 24] {
    fn from_nidhogg(value: LeftEye) -> Self {
        [
            value.l7.red,
            value.l0.red,
            value.l1.red,
            value.l2.red,
            value.l3.red,
            value.l4.red,
            value.l5.red,
            value.l6.red,
            // bad rustfmt
            value.l7.green,
            value.l0.green,
            value.l1.green,
            value.l2.green,
            value.l3.green,
            value.l4.green,
            value.l5.green,
            value.l6.green,
            // bad rustfmt
            value.l7.blue,
            value.l0.blue,
            value.l1.blue,
            value.l2.blue,
            value.l3.blue,
            value.l4.blue,
            value.l5.blue,
            value.l6.blue,
        ]
    }
}

impl FromNidhogg<RightEye> for [f32; 24] {
    fn from_nidhogg(value: RightEye) -> Self {
        [
            value.r0.red,
            value.r7.red,
            value.r6.red,
            value.r5.red,
            value.r4.red,
            value.r3.red,
            value.r2.red,
            value.r1.red,
            // bad rustfmt
            value.r0.green,
            value.r7.green,
            value.r6.green,
            value.r5.green,
            value.r4.green,
            value.r3.green,
            value.r2.green,
            value.r1.green,
            // bad rustfmt
            value.r0.blue,
            value.r7.blue,
            value.r6.blue,
            value.r5.blue,
            value.r4.blue,
            value.r3.blue,
            value.r2.blue,
            value.r1.blue,
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

            right_shoulder_pitch,
            right_shoulder_roll,
            right_elbow_yaw,
            right_elbow_roll,
            right_wrist_yaw,

            right_hip_roll,
            right_hip_pitch,
            right_knee_pitch,
            right_ankle_pitch,
            right_ankle_roll,

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
        vector![value[0], value[1]]
    }
}

impl FromLoLA<[f32; 3]> for Vector3<f32> {
    fn from_lola(value: [f32; 3]) -> Self {
        vector![value[0], value[1], value[2]]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolaControlMsg {
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

impl From<LolaControlMsg> for NaoControlMessage {
    fn from(value: LolaControlMsg) -> Self {
        Self {
            position: value.position.into_nidhogg(),
            stiffness: value.stiffness.into_nidhogg(),
            right_ear: value.r_ear.into_nidhogg(),
            left_ear: value.l_ear.into_nidhogg(),
            chest: value.chest.into_nidhogg(),
            left_eye: value.l_eye.into_nidhogg(),
            right_eye: value.r_eye.into_nidhogg(),
            left_foot: value.l_foot.into_nidhogg(),
            right_foot: value.r_foot.into_nidhogg(),
            skull: value.skull.into_nidhogg(),
            sonar: value.sonar.into_nidhogg(),
        }
    }
}

impl FromLoLA<[f32; 10]> for LeftEar {
    fn from_lola(value: [f32; 10]) -> LeftEar {
        LeftEar {
            l0: value[0],
            l1: value[1],
            l2: value[2],
            l3: value[3],
            l4: value[4],
            l5: value[5],
            l6: value[6],
            l7: value[7],
            l8: value[8],
            l9: value[9],
        }
    }
}

impl FromLoLA<[f32; 10]> for RightEar {
    fn from_lola(value: [f32; 10]) -> RightEar {
        RightEar {
            r0: value[9],
            r1: value[8],
            r2: value[7],
            r3: value[6],
            r4: value[5],
            r5: value[4],
            r6: value[3],
            r7: value[2],
            r8: value[1],
            r9: value[0],
        }
    }
}

impl FromLoLA<[f32; 3]> for Rgb<f32> {
    fn from_lola(value: [f32; 3]) -> Self {
        Rgb {
            red: value[0],
            green: value[1],
            blue: value[2],
        }
    }
}

impl FromLoLA<[f32; 24]> for LeftEye {
    fn from_lola(value: [f32; 24]) -> LeftEye {
        let [
            l7_r,
            l0_r,
            l1_r,
            l2_r,
            l3_r,
            l4_r,
            l5_r,
            l6_r,
            l7_g,
            l0_g,
            l1_g,
            l2_g,
            l3_g,
            l4_g,
            l5_g,
            l6_g,
            l7_b,
            l0_b,
            l1_b,
            l2_b,
            l3_b,
            l4_b,
            l5_b,
            l6_b,
            // bad rustfmt
        ] = value;

        LeftEye {
            l0: Rgb {
                red: l0_r,
                green: l0_g,
                blue: l0_b,
            },
            l1: Rgb {
                red: l1_r,
                green: l1_g,
                blue: l1_b,
            },
            l2: Rgb {
                red: l2_r,
                green: l2_g,
                blue: l2_b,
            },
            l3: Rgb {
                red: l3_r,
                green: l3_g,
                blue: l3_b,
            },
            l4: Rgb {
                red: l4_r,
                green: l4_g,
                blue: l4_b,
            },
            l5: Rgb {
                red: l5_r,
                green: l5_g,
                blue: l5_b,
            },
            l6: Rgb {
                red: l6_r,
                green: l6_g,
                blue: l6_b,
            },
            l7: Rgb {
                red: l7_r,
                green: l7_g,
                blue: l7_b,
            },
        }
    }
}

impl FromLoLA<[f32; 24]> for RightEye {
    fn from_lola(value: [f32; 24]) -> RightEye {
        let [
            r7_r,
            r6_r,
            r5_r,
            r4_r,
            r3_r,
            r2_r,
            r1_r,
            r0_r,
            r7_g,
            r6_g,
            r5_g,
            r4_g,
            r3_g,
            r2_g,
            r1_g,
            r0_g,
            r7_b,
            r6_b,
            r5_b,
            r4_b,
            r3_b,
            r2_b,
            r1_b,
            r0_b
            // bad rustfmt
        ] = value;

        RightEye {
            r0: Rgb {
                red: r0_r,
                green: r0_g,
                blue: r0_b,
            },
            r1: Rgb {
                red: r1_r,
                green: r1_g,
                blue: r1_b,
            },
            r2: Rgb {
                red: r2_r,
                green: r2_g,
                blue: r2_b,
            },
            r3: Rgb {
                red: r3_r,
                green: r3_g,
                blue: r3_b,
            },
            r4: Rgb {
                red: r4_r,
                green: r4_g,
                blue: r4_b,
            },
            r5: Rgb {
                red: r5_r,
                green: r5_g,
                blue: r5_b,
            },
            r6: Rgb {
                red: r6_r,
                green: r6_g,
                blue: r6_b,
            },
            r7: Rgb {
                red: r7_r,
                green: r7_g,
                blue: r7_b,
            },
        }
    }
}

impl FromLoLA<[f32; 12]> for Skull {
    fn from_lola(value: [f32; 12]) -> Skull {
        let [
            left_front_1,
            left_front_0,
            left_middle_0,
            left_rear_0,
            left_rear_1,
            left_rear_2,
            right_rear_2,
            right_rear_1,
            right_rear_0,
            right_middle_0,
            right_front_0,
            right_front_1,
            // bad rustfmt
        ] = value;

        Skull {
            left_front_0,
            left_front_1,
            left_middle_0,
            left_rear_0,
            left_rear_1,
            left_rear_2,
            right_front_0,
            right_front_1,
            right_middle_0,
            right_rear_0,
            right_rear_1,
            right_rear_2,
        }
    }
}
