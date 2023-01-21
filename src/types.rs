//! Types used to make interacting with the LoLA socket more convenient.
//!

use std::marker::PhantomData;

use nidhogg_derive::Builder;

/// Struct containing two values of type `T`
#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Struct containing three values of type `T`
#[derive(Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// High level representation of the LoLA state message.
#[derive(Debug)]
pub struct State {
    pub position: JointArray<f32>,
    pub stiffness: JointArray<f32>,

    // Inertial measurement unit
    pub accelerometer: Vector3<f32>,
    pub gyroscope: Vector3<f32>,
    pub angles: Vector2<f32>,

    pub sonar: SonarValues,
    pub force_sensitive_resistors: ForceSensitiveResistors,
    pub touch: Touch,
    pub battery: Battery,
    pub temperature: JointArray<f32>,
    pub current: JointArray<f32>,
    pub status: JointArray<i32>,
}

/// High level representation of the LoLA update message.
#[derive(Builder, Clone, Debug)]
pub struct Update {
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

impl Default for Update {
    fn default() -> Self {
        Self {
            position: Default::default(),
            stiffness: Default::default(),
            sonar: Sonar {
                left: true,
                right: true,
            },
            left_ear: Default::default(),
            right_ear: Default::default(),
            chest: Default::default(),
            left_eye: Default::default(),
            right_eye: Default::default(),
            left_foot: Default::default(),
            right_foot: Default::default(),
            skull: Default::default(),
        }
    }
}

/// Struct representing the LEDs on top of the NAO robot's head.  
///
/// Each value represents the intensity of a white LED.
#[derive(Builder, Clone, Debug, Default)]
pub struct Skull {
    pub left_front_0: f32,
    pub left_front_1: f32,
    pub left_middle_0: f32,
    pub left_rear_0: f32,
    pub left_rear_1: f32,
    pub left_rear_2: f32,

    pub right_front_0: f32,
    pub right_front_1: f32,
    pub right_middle_0: f32,
    pub right_rear_0: f32,
    pub right_rear_1: f32,
    pub right_rear_2: f32,
}

/// Marker struct indicating the left side.
#[derive(Clone, Debug, Default)]
pub struct Left;

/// Marker struct indicating the right side.
#[derive(Clone, Debug, Default)]
pub struct Right;

/// Struct representing the LED intensities in the ear of the robot.
/// ## ⚠️ Warning:
/// You should construct the [`LeftEar`] and [`RightEar`] types instead of using [`Ear`] directly.
///
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```no_run
///        0
///    324  36
///  288     72
/// 252     108
///  216  144
///    180
/// ```  
/// TODO: image
#[derive(Builder, Clone, Debug, Default)]
pub struct Ear<Side> {
    pub intensity_0_deg: f32,
    pub intensity_36_deg: f32,
    pub intensity_72_deg: f32,
    pub intensity_108_deg: f32,
    pub intensity_144_deg: f32,
    pub intensity_180_deg: f32,
    pub intensity_216_deg: f32,
    pub intensity_252_deg: f32,
    pub intensity_288_deg: f32,
    pub intensity_324_deg: f32,

    pub _marker: PhantomData<Side>,
}

/// Type alias for the left ear of the robot.
pub type LeftEar = Ear<Left>;
/// Type alias for the right ear of the robot
pub type RightEar = Ear<Right>;

/// Struct representing an RGB color.
#[derive(Debug, Default, Clone, Copy, Builder)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

/// Struct representing the RGB LEDs in the eye of the robot.
///
/// ## ⚠️ Warning:
/// You should construct the [`LeftEye`] and [`RightEye`] types instead of using [`Eye`] directly.
///
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```no_run
///     0
///  45    315
/// 90      270
///  135   225
///    180
/// ```  
/// TODO: image
#[derive(Builder, Clone, Debug, Default)]
pub struct Eye<Side> {
    pub color_0_deg: Color,
    pub color_45_deg: Color,
    pub color_90_deg: Color,
    pub color_135_deg: Color,
    pub color_180_deg: Color,
    pub color_225_deg: Color,
    pub color_270_deg: Color,
    pub color_315_deg: Color,

    _marker: PhantomData<Side>,
}

/// Type alias for the left eye of the robot.
pub type LeftEye = Eye<Left>;

/// Type alias for the right eye of the robot.
pub type RightEye = Eye<Right>;

/// Struct containing the hardware identifiers for the NAO 6 robot.
#[derive(Debug)]
pub struct HardwareInfo {
    pub body_id: String,
    pub body_version: String,
    pub head_id: String,
    pub head_version: String,
}

/// Struct containing values of type `T` for all the joints
#[derive(Builder, Clone, Debug, Default)]
pub struct JointArray<T> {
    pub head_yaw: T,
    pub head_pitch: T,

    pub left_shoulder_pitch: T,
    pub left_shoulder_roll: T,
    pub left_elbow_yaw: T,
    pub left_elbow_roll: T,
    pub left_wrist_yaw: T,
    pub left_hip_yaw_pitch: T,
    pub left_hip_roll: T,
    pub left_hip_pitch: T,
    pub left_knee_pitch: T,
    pub left_ankle_pitch: T,
    pub left_ankle_roll: T,

    // There is no `right_hip_yaw_pitch` in LoLA
    // can't have shit in Ohio
    pub right_hip_roll: T,
    pub right_hip_pitch: T,
    pub right_knee_pitch: T,
    pub right_ankle_pitch: T,
    pub right_ankle_roll: T,
    pub right_shoulder_pitch: T,
    pub right_shoulder_roll: T,
    pub right_elbow_yaw: T,
    pub right_elbow_roll: T,
    pub right_wrist_yaw: T,

    pub left_hand: T,
    pub right_hand: T,
}

/// Struct representing the battery status of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct Battery {
    /// The battery percentage
    pub charge: f32,
    /// Current emitted by battery
    pub current: f32,
    /// Unknown field
    pub status: f32,
    /// Temperature of the battery
    pub temperature: f32,
}

/// Struct containing the [`ForceSensitiveResistorFoot`] value for each foot.
#[derive(Builder, Clone, Debug, Default)]
pub struct ForceSensitiveResistors {
    pub left_foot: ForceSensitiveResistorFoot,
    pub right_foot: ForceSensitiveResistorFoot,
}

/// Struct representing the force sensitive resistors in one of the feet.
#[derive(Builder, Clone, Debug, Default)]
pub struct ForceSensitiveResistorFoot {
    pub front_left: f32,
    pub front_right: f32,
    pub rear_left: f32,
    pub rear_right: f32,
}

/// Struct containing values of type `T` for the left and the right sonar.
///
/// ## ⚠️ Warning:
/// You should construct the [`SonarValues`] and [`SonarEnabled`] types instead of using [`Sonar`] directly.
#[derive(Builder, Clone, Debug, Default)]
pub struct Sonar<T> {
    pub left: T,
    pub right: T,
}

/// Values read by the left and right sonar.
///
/// **Because this is a type alias, the fields aren't on this page**. To view them see [`Sonar`].
pub type SonarValues = Sonar<f32>;

/// Enabled state of the left and right sonar.
///
/// **Because this is a type alias, the fields aren't on this page**. To view them see [`Sonar`].
pub type SonarEnabled = Sonar<bool>;

/// Struct containing the touch activiation value for each touch sensor on the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct Touch {
    pub chest_board: f32,
    pub head_front: f32,
    pub head_middle: f32,
    pub head_rear: f32,
    pub left_foot_left: f32,
    pub left_foot_right: f32,
    pub left_hand_back: f32,
    pub left_hand_left: f32,
    pub left_hand_right: f32,
    pub right_foot_left: f32,
    pub right_foot_right: f32,
    pub right_hand_back: f32,
    pub right_hand_left: f32,
    pub right_hand_right: f32,
}
