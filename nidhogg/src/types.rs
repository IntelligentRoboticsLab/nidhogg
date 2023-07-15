//! Convenience types used to make interacting with the NAO more convenient.
//!

use nidhogg_derive::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Struct containing two values of type `T`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Struct containing three values of type `T`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Struct representing the LEDs on top of the NAO robot's head.  
///
/// Each value represents the intensity of a white LED.
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// Struct representing the LED intensities in the left ear of the robot.
///
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```text
///        0
///    324  36
///  288     72
/// 252     108
///  216  144
///    180
/// ```  
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeftEar {
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
}

/// Struct representing the LED intensities in the right ear of the robot.
///
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```text
///        0
///    324  36
///  288     72
/// 252     108
///  216  144
///    180
/// ```  
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RightEar {
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
}

/// Struct representing an RGB color.
#[derive(Debug, Default, Clone, Copy, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    #[must_use]
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

/// Struct representing the RGB LEDs in the left eye of the robot.
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```text
///     0
///  45    315
/// 90      270
///  135   225
///    180
/// ```  
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeftEye {
    pub color_0_deg: Color,
    pub color_45_deg: Color,
    pub color_90_deg: Color,
    pub color_135_deg: Color,
    pub color_180_deg: Color,
    pub color_225_deg: Color,
    pub color_270_deg: Color,
    pub color_315_deg: Color,
}

/// Struct representing the RGB LEDs in the left eye of the robot.
/// ## LED order:
/// These LEDs are placed in the following order:
/// ```text
///     0
///  45    315
/// 90      270
///  135   225
///    180
/// ```  
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RightEye {
    pub color_0_deg: Color,
    pub color_45_deg: Color,
    pub color_90_deg: Color,
    pub color_135_deg: Color,
    pub color_180_deg: Color,
    pub color_225_deg: Color,
    pub color_270_deg: Color,
    pub color_315_deg: Color,
}

/// Struct containing values of type `T` for all the joints
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JointArray<T> {
    /// The yaw joint of the robot's head, allowing rotation horizontally.
    pub head_yaw: T,

    /// The pitch joint of the robot's head, allowing tilting up and down.
    pub head_pitch: T,

    /// The pitch joint of the left shoulder, controlling its vertical movement.
    pub left_shoulder_pitch: T,

    /// The roll joint of the left shoulder, controlling its horizontal movement.
    pub left_shoulder_roll: T,

    /// The yaw joint of the left elbow, allowing rotation.
    pub left_elbow_yaw: T,

    /// The roll joint of the left elbow, controlling its horizontal movement.
    pub left_elbow_roll: T,

    /// The yaw joint of the left wrist, allowing rotation.
    pub left_wrist_yaw: T,

    /// The yaw-pitch joint of the left hip, controlling horizontal and vertical movement.
    pub left_hip_yaw_pitch: T,

    /// The roll joint of the left hip, controlling its horizontal movement.
    pub left_hip_roll: T,

    /// The pitch joint of the left hip, controlling its vertical movement.
    pub left_hip_pitch: T,

    /// The pitch joint of the left knee, controlling its bending movement.
    pub left_knee_pitch: T,

    /// The pitch joint of the left ankle, controlling its bending movement.
    pub left_ankle_pitch: T,

    /// The roll joint of the left ankle, controlling its horizontal movement.
    pub left_ankle_roll: T,

    /// The pitch joint of the right shoulder, controlling its vertical movement.
    pub right_shoulder_pitch: T,

    /// The roll joint of the right shoulder, controlling its horizontal movement.
    pub right_shoulder_roll: T,

    /// The yaw joint of the right elbow, allowing rotation.
    pub right_elbow_yaw: T,

    /// The roll joint of the right elbow, controlling its horizontal movement.
    pub right_elbow_roll: T,

    /// The yaw joint of the right wrist, allowing rotation.
    pub right_wrist_yaw: T,

    /// The roll joint of the right hip, controlling its horizontal movement.
    pub right_hip_roll: T,

    /// The pitch joint of the right hip, controlling its vertical movement.
    pub right_hip_pitch: T,

    /// The pitch joint of the right knee, controlling its bending movement.
    pub right_knee_pitch: T,

    /// The pitch joint of the right ankle, controlling its bending movement.
    pub right_ankle_pitch: T,

    /// The roll joint of the right ankle, controlling its horizontal movement.
    pub right_ankle_roll: T,

    /// The joint representing the left hand.
    pub left_hand: T,

    /// The joint representing the right hand.
    pub right_hand: T,
}

/// Struct representing the battery status of the robot.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Battery {
    /// The battery percentage
    pub charge: f32,
    /// Current emitted by battery
    pub current: f32,
    /// Unknown field
    // todo: test whether this is charging state
    pub status: f32,
    /// Temperature of the battery
    pub temperature: f32,
}

/// Struct containing the [`ForceSensitiveResistorFoot`] value for each foot.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceSensitiveResistors {
    pub left_foot: ForceSensitiveResistorFoot,
    pub right_foot: ForceSensitiveResistorFoot,
}

/// Struct representing the force sensitive resistors in one of the feet.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceSensitiveResistorFoot {
    pub front_left: f32,
    pub front_right: f32,
    pub rear_left: f32,
    pub rear_right: f32,
}

/// Values read by the left and right sonar sensor.
#[derive(Builder, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SonarValues {
    pub left: f32,
    pub right: f32,
}

/// Enabled state of the left and right sonar sensor.
#[derive(Builder, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SonarEnabled {
    pub left: bool,
    pub right: bool,
}

impl Default for SonarEnabled {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
        }
    }
}

/// Struct containing the touch activiation value for each touch sensor on the robot.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Touch<T> {
    pub chest_board: T,
    pub head_front: T,
    pub head_middle: T,
    pub head_rear: T,
    pub left_foot_left: T,
    pub left_foot_right: T,
    pub left_hand_back: T,
    pub left_hand_left: T,
    pub left_hand_right: T,
    pub right_foot_left: T,
    pub right_foot_right: T,
    pub right_hand_back: T,
    pub right_hand_left: T,
    pub right_hand_right: T,
}

/// Wrapper struct containing the head joints of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct HeadJoints<T> {
    pub yaw: T,
    pub pitch: T,
}

/// Wrapper struct containing the left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct LeftLegJoints<T> {
    pub hip_yaw_pitch: T,
    pub hip_roll: T,
    pub hip_pitch: T,
    pub knee_pitch: T,
    pub ankle_pitch: T,
    pub ankle_roll: T,
}

/// Wrapper struct containing right left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct RightLegJoints<T> {
    // This value does not exist
    // pub hip_yaw_pitch: T,
    pub hip_roll: T,
    pub hip_pitch: T,
    pub knee_pitch: T,
    pub ankle_pitch: T,
    pub ankle_roll: T,
}

/// Wrapper struct containing joint values for both legs of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct LegJoints<T> {
    pub left_leg: LeftLegJoints<T>,
    pub right_leg: RightLegJoints<T>,
}

/// Wrapper struct containing the joints for a single arm of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct SingleArmJoints<T> {
    pub shoulder_pitch: T,
    pub shoulder_roll: T,
    pub elbow_yaw: T,
    pub elbow_roll: T,
    pub wrist_yaw: T,
    pub hand: T,
}

/// Type definition for the left arm joints of the robot.
/// Introduced for api consistenty with [`LeftLegJoints`].
pub type LeftArmJoints<T> = SingleArmJoints<T>;

/// Type definition for the right arm joints of the robot.
/// Introduced for api consistenty with [`RightLegJoints`].
pub type RightArmJoints<T> = SingleArmJoints<T>;

/// Wrapper struct containing the arm joints of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct ArmJoints<T> {
    pub left_arm: SingleArmJoints<T>,
    pub right_arm: SingleArmJoints<T>,
}

impl<T> JointArrayBuilder<T> {
    /// Set the `head_pitch` and `head_yaw` values to the corresponding values from the provided [`HeadJoints`].
    pub fn head_joints(mut self, joints: HeadJoints<T>) -> Self {
        self.head_pitch = Some(joints.pitch);
        self.head_yaw = Some(joints.yaw);
        self
    }

    /// Set the values for the left leg joints to the corresponding values from the provided [`LeftLegJoints`].
    pub fn left_leg_joints(mut self, joints: LeftLegJoints<T>) -> Self {
        self.left_hip_yaw_pitch = Some(joints.hip_yaw_pitch);
        self.left_hip_roll = Some(joints.hip_roll);
        self.left_hip_pitch = Some(joints.hip_pitch);
        self.left_knee_pitch = Some(joints.knee_pitch);
        self.left_ankle_pitch = Some(joints.ankle_pitch);
        self.left_ankle_roll = Some(joints.ankle_roll);
        self
    }

    /// Set the values for the right leg joints to the corresponding values from the provided [`RightLegJoints`].
    pub fn right_leg_joints(mut self, joints: RightLegJoints<T>) -> Self {
        self.right_hip_roll = Some(joints.hip_roll);
        self.right_hip_pitch = Some(joints.hip_pitch);
        self.right_knee_pitch = Some(joints.knee_pitch);
        self.right_ankle_pitch = Some(joints.ankle_pitch);
        self.right_ankle_roll = Some(joints.ankle_roll);
        self
    }

    /// Set the values for the leg joints to the corresponding values from the provided [`LegJoints`].
    pub fn leg_joints(mut self, joints: LegJoints<T>) -> Self {
        self.left_hip_yaw_pitch = Some(joints.left_leg.hip_yaw_pitch);
        self.left_hip_roll = Some(joints.left_leg.hip_roll);
        self.left_hip_pitch = Some(joints.left_leg.hip_pitch);
        self.left_knee_pitch = Some(joints.left_leg.knee_pitch);
        self.left_ankle_pitch = Some(joints.left_leg.ankle_pitch);
        self.left_ankle_roll = Some(joints.left_leg.ankle_roll);
        self.right_hip_roll = Some(joints.right_leg.hip_roll);
        self.right_hip_pitch = Some(joints.right_leg.hip_pitch);
        self.right_ankle_pitch = Some(joints.right_leg.ankle_pitch);
        self.right_knee_pitch = Some(joints.right_leg.knee_pitch);
        self.right_ankle_roll = Some(joints.right_leg.ankle_roll);
        self
    }

    /// Set the values for the left arm joints to the corresponding values from the provided [`LeftArmJoints`].
    pub fn left_arm_joints(mut self, joints: LeftArmJoints<T>) -> Self {
        self.left_shoulder_pitch = Some(joints.shoulder_pitch);
        self.left_shoulder_roll = Some(joints.shoulder_roll);
        self.left_elbow_yaw = Some(joints.elbow_yaw);
        self.left_elbow_roll = Some(joints.elbow_roll);
        self.left_wrist_yaw = Some(joints.wrist_yaw);
        self.left_hand = Some(joints.hand);
        self
    }

    /// Set the values for the right arm joints to the corresponding values from the provided [`RightArmJoints`].
    pub fn right_arm_joints(mut self, joints: RightArmJoints<T>) -> Self {
        self.right_shoulder_pitch = Some(joints.shoulder_pitch);
        self.right_shoulder_roll = Some(joints.shoulder_roll);
        self.right_elbow_yaw = Some(joints.elbow_yaw);
        self.right_elbow_roll = Some(joints.elbow_roll);
        self.right_wrist_yaw = Some(joints.wrist_yaw);
        self.right_hand = Some(joints.hand);
        self
    }

    /// Set the values for the arm joints to the corresponding values from the provided [`ArmJoints`].
    pub fn arm_joints(mut self, joints: ArmJoints<T>) -> Self {
        self.left_shoulder_pitch = Some(joints.left_arm.shoulder_pitch);
        self.left_shoulder_roll = Some(joints.left_arm.shoulder_roll);
        self.left_elbow_yaw = Some(joints.left_arm.elbow_yaw);
        self.left_elbow_roll = Some(joints.left_arm.elbow_roll);
        self.left_wrist_yaw = Some(joints.left_arm.wrist_yaw);
        self.left_hand = Some(joints.left_arm.hand);
        self.right_shoulder_pitch = Some(joints.right_arm.shoulder_pitch);
        self.right_shoulder_roll = Some(joints.right_arm.shoulder_roll);
        self.right_elbow_yaw = Some(joints.right_arm.elbow_yaw);
        self.right_elbow_roll = Some(joints.right_arm.elbow_roll);
        self.right_wrist_yaw = Some(joints.right_arm.wrist_yaw);
        self.right_hand = Some(joints.right_arm.hand);
        self
    }
}
