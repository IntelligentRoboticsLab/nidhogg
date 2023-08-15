//! Convenience types used to make interacting with the NAO more convenient.
//!

use nidhogg_derive::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Struct containing two values of type `T`
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Struct containing three values of type `T`
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Trait that introduces the [`fill`](`FillExt::fill`) method for a type, which allows filling in all fields with the same value.
pub trait FillExt<T> {
    /// Return a new instance of the type, with all fields set to the provided value.
    fn fill(value: T) -> Self;
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

impl FillExt<f32> for Skull {
    fn fill(intensity: f32) -> Skull {
        Skull {
            left_front_0: intensity,
            left_front_1: intensity,
            left_middle_0: intensity,
            left_rear_0: intensity,
            left_rear_1: intensity,
            left_rear_2: intensity,
            right_front_0: intensity,
            right_front_1: intensity,
            right_middle_0: intensity,
            right_rear_0: intensity,
            right_rear_1: intensity,
            right_rear_2: intensity,
        }
    }
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

impl FillExt<f32> for LeftEar {
    fn fill(intensity: f32) -> LeftEar {
        LeftEar {
            intensity_0_deg: intensity,
            intensity_36_deg: intensity,
            intensity_72_deg: intensity,
            intensity_108_deg: intensity,
            intensity_144_deg: intensity,
            intensity_180_deg: intensity,
            intensity_216_deg: intensity,
            intensity_252_deg: intensity,
            intensity_288_deg: intensity,
            intensity_324_deg: intensity,
        }
    }
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

impl FillExt<f32> for RightEar {
    fn fill(intensity: f32) -> RightEar {
        RightEar {
            intensity_0_deg: intensity,
            intensity_36_deg: intensity,
            intensity_72_deg: intensity,
            intensity_108_deg: intensity,
            intensity_144_deg: intensity,
            intensity_180_deg: intensity,
            intensity_216_deg: intensity,
            intensity_252_deg: intensity,
            intensity_288_deg: intensity,
            intensity_324_deg: intensity,
        }
    }
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

    /// Create a new color from three `u8` values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
        }
    }

    /// Create a new color from a u32 value.
    pub fn new_u32(color: u32) -> Self {
        Color::new_u8(
            ((color >> 16) & 0xFF) as u8,
            ((color >> 8) & 0xFF) as u8,
            (color & 0xFF) as u8,
        )
    }

    /// The color blue
    pub const BLUE: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };

    /// The color cyan
    pub const CYAN: Color = Color {
        red: 0.0,
        green: 1.0,
        blue: 1.0,
    };

    /// No color
    ///
    /// This color will result in the LEDs being turned off.
    pub const EMPTY: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    /// The color gray
    pub const GRAY: Color = Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
    };

    /// The color green
    pub const GREEN: Color = Color {
        red: 0.0,
        green: 0.5,
        blue: 0.0,
    };

    /// The color lime
    pub const LIME: Color = Color {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    };

    /// The color magenta
    pub const MAGENTA: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 1.0,
    };

    /// The color maroon
    pub const MAROON: Color = Color {
        red: 0.5,
        green: 0.0,
        blue: 0.0,
    };

    /// The color navy
    pub const NAVY: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.5,
    };

    /// The color olive
    pub const OLIVE: Color = Color {
        red: 0.5,
        green: 0.5,
        blue: 0.0,
    };

    /// The color purple
    pub const PURPLE: Color = Color {
        red: 0.5,
        green: 0.0,
        blue: 0.5,
    };

    /// The color red
    pub const RED: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };

    /// The color silver
    pub const SILVER: Color = Color {
        red: 0.75,
        green: 0.75,
        blue: 0.75,
    };

    /// The color teal
    pub const TEAL: Color = Color {
        red: 0.0,
        green: 0.5,
        blue: 0.5,
    };

    /// The color white
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    /// The color yellow
    pub const YELLOW: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 0.0,
    };
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

impl FillExt<Color> for LeftEye {
    fn fill(color: Color) -> LeftEye {
        LeftEye {
            color_0_deg: color,
            color_45_deg: color,
            color_90_deg: color,
            color_135_deg: color,
            color_180_deg: color,
            color_225_deg: color,
            color_270_deg: color,
            color_315_deg: color,
        }
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

impl FillExt<Color> for RightEye {
    fn fill(color: Color) -> RightEye {
        RightEye {
            color_0_deg: color,
            color_45_deg: color,
            color_90_deg: color,
            color_135_deg: color,
            color_180_deg: color,
            color_225_deg: color,
            color_270_deg: color,
            color_315_deg: color,
        }
    }
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

impl<T> JointArray<T> {
    /// Retrieves the left leg joints.
    pub fn left_leg_joints(&self) -> LeftLegJoints<&T> {
        LeftLegJoints {
            hip_yaw_pitch: &self.left_hip_yaw_pitch,
            hip_roll: &self.left_hip_roll,
            hip_pitch: &self.left_hip_pitch,
            knee_pitch: &self.left_knee_pitch,
            ankle_pitch: &self.left_ankle_pitch,
            ankle_roll: &self.left_ankle_roll,
        }
    }

    /// Retrieves the left arm joints.
    pub fn left_arm_joints(&self) -> LeftArmJoints<&T> {
        LeftArmJoints {
            shoulder_pitch: &self.left_shoulder_pitch,
            shoulder_roll: &self.left_shoulder_roll,
            elbow_yaw: &self.left_elbow_yaw,
            elbow_roll: &self.left_elbow_roll,
            wrist_yaw: &self.left_wrist_yaw,
            hand: &self.left_hand,
        }
    }

    /// Retrieves the right leg joints.
    pub fn right_leg_joints(&self) -> RightLegJoints<&T> {
        RightLegJoints {
            hip_roll: &self.right_hip_roll,
            hip_pitch: &self.right_hip_pitch,
            knee_pitch: &self.right_knee_pitch,
            ankle_pitch: &self.right_ankle_pitch,
            ankle_roll: &self.right_ankle_roll,
        }
    }

    /// Retrieves the right arm joints.
    pub fn right_arm_joints(&self) -> RightArmJoints<&T> {
        RightArmJoints {
            shoulder_pitch: &self.right_shoulder_pitch,
            shoulder_roll: &self.right_shoulder_roll,
            elbow_yaw: &self.right_elbow_yaw,
            elbow_roll: &self.right_elbow_roll,
            wrist_yaw: &self.right_wrist_yaw,
            hand: &self.right_hand,
        }
    }

    /// Retrieves the head joints.
    pub fn head_joints(&self) -> HeadJoints<&T> {
        HeadJoints {
            yaw: &self.head_yaw,
            pitch: &self.head_pitch,
        }
    }

    /// Returns a `JointArrayIterator`.
    pub fn iter(&self) -> JointArrayIterator<'_, T> {
        JointArrayIterator::new(self)
    }

    /// Initializes a `JointArray` from an iterator.
    fn from_iter<I>(mut iter: I) -> JointArray<T>
    where
        I: std::iter::Iterator<Item = T>,
    {
        Self {
            head_yaw: iter.next().unwrap(),
            head_pitch: iter.next().unwrap(),
            left_shoulder_pitch: iter.next().unwrap(),
            left_shoulder_roll: iter.next().unwrap(),
            left_elbow_yaw: iter.next().unwrap(),
            left_elbow_roll: iter.next().unwrap(),
            left_wrist_yaw: iter.next().unwrap(),
            left_hip_yaw_pitch: iter.next().unwrap(),
            left_hip_roll: iter.next().unwrap(),
            left_hip_pitch: iter.next().unwrap(),
            left_knee_pitch: iter.next().unwrap(),
            left_ankle_pitch: iter.next().unwrap(),
            left_ankle_roll: iter.next().unwrap(),
            right_shoulder_pitch: iter.next().unwrap(),
            right_shoulder_roll: iter.next().unwrap(),
            right_elbow_yaw: iter.next().unwrap(),
            right_elbow_roll: iter.next().unwrap(),
            right_wrist_yaw: iter.next().unwrap(),
            right_hip_roll: iter.next().unwrap(),
            right_hip_pitch: iter.next().unwrap(),
            right_knee_pitch: iter.next().unwrap(),
            right_ankle_pitch: iter.next().unwrap(),
            right_ankle_roll: iter.next().unwrap(),
            left_hand: iter.next().unwrap(),
            right_hand: iter.next().unwrap(),
        }
    }
}

impl<T: Clone> FillExt<T> for JointArray<T> {
    fn fill(value: T) -> JointArray<T> {
        JointArray::from_iter(std::iter::repeat(value))
    }
}

impl<T> std::iter::FromIterator<T> for JointArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let iter = iterable.into_iter();
        JointArray::from_iter(iter)
    }
}

#[derive(Debug)]
enum JointArrayField {
    HeadYaw,
    HeadPitch,
    LeftShoulderPitch,
    LeftShoulderRoll,
    LeftElbowYaw,
    LeftElbowRoll,
    LeftWristYaw,
    LeftHipYawPitch,
    LeftHipRoll,
    LeftHipPitch,
    LeftKneePitch,
    LeftAnklePitch,
    LeftAnkleRoll,
    RightShoulderPitch,
    RightShoulderRoll,
    RightElbowYaw,
    RightElbowRoll,
    RightWristYaw,
    RightHipRoll,
    RightHipPitch,
    RightKneePitch,
    RightAnklePitch,
    RightAnkleRoll,
    LeftHand,
    RightHand,
    End,
}

impl JointArrayField {
    fn next(&self) -> Self {
        match self {
            Self::HeadYaw => Self::HeadPitch,
            Self::HeadPitch => Self::LeftShoulderPitch,
            Self::LeftShoulderPitch => Self::LeftShoulderRoll,
            Self::LeftShoulderRoll => Self::LeftElbowYaw,
            Self::LeftElbowYaw => Self::LeftElbowRoll,
            Self::LeftElbowRoll => Self::LeftWristYaw,
            Self::LeftWristYaw => Self::LeftHipYawPitch,
            Self::LeftHipYawPitch => Self::LeftHipRoll,
            Self::LeftHipRoll => Self::LeftHipPitch,
            Self::LeftHipPitch => Self::LeftKneePitch,
            Self::LeftKneePitch => Self::LeftAnklePitch,
            Self::LeftAnklePitch => Self::LeftAnkleRoll,
            Self::LeftAnkleRoll => Self::RightShoulderPitch,
            Self::RightShoulderPitch => Self::RightShoulderRoll,
            Self::RightShoulderRoll => Self::RightElbowYaw,
            Self::RightElbowYaw => Self::RightElbowRoll,
            Self::RightElbowRoll => Self::RightWristYaw,
            Self::RightWristYaw => Self::RightHipRoll,
            Self::RightHipRoll => Self::RightHipPitch,
            Self::RightHipPitch => Self::RightKneePitch,
            Self::RightKneePitch => Self::RightAnklePitch,
            Self::RightAnklePitch => Self::RightAnkleRoll,
            Self::RightAnkleRoll => Self::LeftHand,
            Self::LeftHand => Self::RightHand,
            _ => Self::End,
        }
    }
}

#[derive(Debug)]
pub struct JointArrayIterator<'a, T>(JointArrayField, &'a JointArray<T>);

impl<'a, T> JointArrayIterator<'a, T> {
    fn new(joint_array: &'a JointArray<T>) -> Self {
        Self(JointArrayField::HeadYaw, joint_array)
    }
}

impl<'a, T> std::iter::Iterator for JointArrayIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let val = match self.0 {
            JointArrayField::HeadYaw => Some(&self.1.head_yaw),
            JointArrayField::HeadPitch => Some(&self.1.head_pitch),
            JointArrayField::LeftShoulderPitch => Some(&self.1.left_shoulder_pitch),
            JointArrayField::LeftShoulderRoll => Some(&self.1.left_shoulder_roll),
            JointArrayField::LeftElbowYaw => Some(&self.1.left_elbow_yaw),
            JointArrayField::LeftElbowRoll => Some(&self.1.left_elbow_roll),
            JointArrayField::LeftWristYaw => Some(&self.1.left_wrist_yaw),
            JointArrayField::LeftHipYawPitch => Some(&self.1.left_hip_yaw_pitch),
            JointArrayField::LeftHipRoll => Some(&self.1.left_hip_roll),
            JointArrayField::LeftHipPitch => Some(&self.1.left_hip_pitch),
            JointArrayField::LeftKneePitch => Some(&self.1.left_knee_pitch),
            JointArrayField::LeftAnklePitch => Some(&self.1.left_ankle_pitch),
            JointArrayField::LeftAnkleRoll => Some(&self.1.left_ankle_roll),
            JointArrayField::RightShoulderPitch => Some(&self.1.right_shoulder_pitch),
            JointArrayField::RightShoulderRoll => Some(&self.1.right_shoulder_roll),
            JointArrayField::RightElbowYaw => Some(&self.1.right_elbow_yaw),
            JointArrayField::RightElbowRoll => Some(&self.1.right_elbow_roll),
            JointArrayField::RightWristYaw => Some(&self.1.right_wrist_yaw),
            JointArrayField::RightHipRoll => Some(&self.1.right_hip_roll),
            JointArrayField::RightHipPitch => Some(&self.1.right_hip_pitch),
            JointArrayField::RightKneePitch => Some(&self.1.right_knee_pitch),
            JointArrayField::RightAnklePitch => Some(&self.1.right_ankle_pitch),
            JointArrayField::RightAnkleRoll => Some(&self.1.right_ankle_roll),
            JointArrayField::LeftHand => Some(&self.1.left_hand),
            JointArrayField::RightHand => Some(&self.1.right_hand),
            JointArrayField::End => None,
        };
        self.0 = self.0.next();
        val
    }
}

// impl<T: Clone> std::iter::IntoIterator for JointArray<T> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> std::vec::IntoIter<T> {
//         vec![
//             self.head_yaw.clone(),
//             self.head_pitch.clone(),
//             self.left_shoulder_pitch.clone(),
//             self.left_shoulder_roll.clone(),
//             self.left_elbow_yaw.clone(),
//             self.left_elbow_roll.clone(),
//             self.left_wrist_yaw.clone(),
//             self.left_hip_yaw_pitch.clone(),
//             self.left_hip_roll.clone(),
//             self.left_hip_pitch.clone(),
//             self.left_knee_pitch.clone(),
//             self.left_ankle_pitch.clone(),
//             self.left_ankle_roll.clone(),
//             self.right_shoulder_pitch.clone(),
//             self.right_shoulder_roll.clone(),
//             self.right_elbow_yaw.clone(),
//             self.right_elbow_roll.clone(),
//             self.right_wrist_yaw.clone(),
//             self.right_hip_roll.clone(),
//             self.right_hip_pitch.clone(),
//             self.right_knee_pitch.clone(),
//             self.right_ankle_pitch.clone(),
//             self.right_ankle_roll.clone(),
//             self.left_hand.clone(),
//             self.right_hand.clone(),
//         ]
//         .into_iter()
//     }
// }

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
    /// FSR values from the four sensors in the left foot.
    pub left_foot: ForceSensitiveResistorFoot,
    /// FSR values from the four sensors in the right foot.
    pub right_foot: ForceSensitiveResistorFoot,
}

impl ForceSensitiveResistors {
    /// Computes the sum of the FSR sensor values for both feet.
    pub fn sum(&self) -> f32 {
        self.left_foot.sum() + self.right_foot.sum()
    }

    /// Calculates the average weigth based on the measurement from the resistors in both feet.
    pub fn avg(&self) -> f32 {
        (self.left_foot.avg() + self.right_foot.avg()) / 2.0
    }
}

/// Struct representing the force sensitive resistors in one of the feet.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceSensitiveResistorFoot {
    /// FSR value representing the estimated weight in kilograms on the front left foot sensor.
    ///
    /// Please note that this value is approximate.
    pub front_left: f32,
    /// FSR value representing the estimated weight in kilograms on the front right foot sensor.
    ///
    /// Please note that this value is approximate.
    pub front_right: f32,
    /// FSR value representing the estimated weight in kilograms on the rear left foot sensor.
    ///
    /// Please note that this value is approximate.
    pub rear_left: f32,
    /// FSR value representing the estimated weight in kilograms on the rear right foot sensor.
    ///
    /// Please note that this value is approximate.
    pub rear_right: f32,
}

impl ForceSensitiveResistorFoot {
    /// Computes the sum of the FSR sensor values for the foot.
    pub fn sum(&self) -> f32 {
        self.front_left + self.front_right + self.rear_left + self.rear_right
    }

    /// Calculates the average weight on the foot.
    pub fn avg(&self) -> f32 {
        self.sum() / 4.0
    }
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

/// Wrapper struct containing the head joints of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct HeadJoints<T> {
    pub yaw: T,
    pub pitch: T,
}

impl<T: Clone> FillExt<T> for HeadJoints<T> {
    fn fill(value: T) -> HeadJoints<T> {
        HeadJoints {
            yaw: value.clone(),
            pitch: value.clone(),
        }
    }
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

impl<T: Clone> FillExt<T> for LeftLegJoints<T> {
    fn fill(value: T) -> LeftLegJoints<T> {
        LeftLegJoints {
            hip_yaw_pitch: value.clone(),
            hip_roll: value.clone(),
            hip_pitch: value.clone(),
            knee_pitch: value.clone(),
            ankle_pitch: value.clone(),
            ankle_roll: value.clone(),
        }
    }
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

impl<T: Clone> FillExt<T> for RightLegJoints<T> {
    fn fill(value: T) -> RightLegJoints<T> {
        RightLegJoints {
            // hip_yaw_pitch: value.clone(),
            hip_roll: value.clone(),
            hip_pitch: value.clone(),
            knee_pitch: value.clone(),
            ankle_pitch: value.clone(),
            ankle_roll: value.clone(),
        }
    }
}

/// Wrapper struct containing joint values for both legs of the robot.
#[derive(Builder, Clone, Debug, Default)]
pub struct LegJoints<T> {
    pub left_leg: LeftLegJoints<T>,
    pub right_leg: RightLegJoints<T>,
}

impl<T: Clone> FillExt<T> for LegJoints<T> {
    fn fill(value: T) -> LegJoints<T> {
        LegJoints {
            left_leg: LeftLegJoints::fill(value.clone()),
            right_leg: RightLegJoints::fill(value.clone()),
        }
    }
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

impl<T: Clone> FillExt<T> for SingleArmJoints<T> {
    fn fill(value: T) -> SingleArmJoints<T> {
        SingleArmJoints {
            shoulder_pitch: value.clone(),
            shoulder_roll: value.clone(),
            elbow_yaw: value.clone(),
            elbow_roll: value.clone(),
            wrist_yaw: value.clone(),
            hand: value.clone(),
        }
    }
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
    /// Set all the joint values to the corresponding values from the provided [`JointArray`].
    pub fn from_joint_array(mut self, joints: JointArray<T>) {
        self.head_pitch = Some(joints.head_pitch);
        self.head_yaw = Some(joints.head_yaw);

        self.left_hip_yaw_pitch = Some(joints.left_hip_yaw_pitch);
        self.left_hip_roll = Some(joints.left_hip_roll);
        self.left_hip_pitch = Some(joints.left_hip_pitch);
        self.left_knee_pitch = Some(joints.left_knee_pitch);
        self.left_ankle_pitch = Some(joints.left_ankle_pitch);
        self.left_ankle_roll = Some(joints.left_ankle_roll);

        self.right_hip_roll = Some(joints.right_hip_roll);
        self.right_hip_pitch = Some(joints.right_hip_pitch);
        self.right_knee_pitch = Some(joints.right_knee_pitch);
        self.right_ankle_pitch = Some(joints.right_ankle_pitch);
        self.right_ankle_roll = Some(joints.right_ankle_roll);

        self.left_shoulder_pitch = Some(joints.left_shoulder_pitch);
        self.left_shoulder_roll = Some(joints.left_shoulder_roll);
        self.left_elbow_yaw = Some(joints.left_elbow_yaw);
        self.left_elbow_roll = Some(joints.left_elbow_roll);
        self.left_wrist_yaw = Some(joints.left_wrist_yaw);
        self.left_hand = Some(joints.left_hand);

        self.right_shoulder_pitch = Some(joints.right_shoulder_pitch);
        self.right_shoulder_roll = Some(joints.right_shoulder_roll);
        self.right_elbow_yaw = Some(joints.right_elbow_yaw);
        self.right_elbow_roll = Some(joints.right_elbow_roll);
        self.right_wrist_yaw = Some(joints.right_wrist_yaw);
        self.right_hand = Some(joints.right_hand);
    }

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

#[cfg(test)]
mod tests {
    use crate::types::JointArray;
    use crate::types::{Color, FillExt, LeftEye};

    use super::ForceSensitiveResistors;
    use super::{
        ForceSensitiveResistorFoot, HeadJoints, LeftArmJoints, LeftLegJoints, RightArmJoints,
        RightLegJoints,
    };

    #[test]
    fn test_average_force_feet() {
        let foot1 = ForceSensitiveResistorFoot {
            front_left: 0.0,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.76,
        };
        let foot2 = ForceSensitiveResistorFoot {
            front_left: 0.54,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.95,
        };
        let feet = ForceSensitiveResistors {
            left_foot: foot1,
            right_foot: foot2,
        };
        assert_eq!(feet.avg(), 0.61125);
    }

    #[test]
    fn test_average_weight_foot() {
        let foot = ForceSensitiveResistorFoot {
            front_left: 0.0,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.76,
        };
        assert_eq!(foot.avg(), 0.52);
    }

    #[test]
    fn test_color_new() {
        let color = Color::new(0.5, 0.5, 0.5);
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.5);
        assert_eq!(color.blue, 0.5);
    }

    #[test]
    fn test_color_new_u8() {
        let color = Color::new_u8(255, 255, 255);
        assert_eq!(color.red, 1.0);
        assert_eq!(color.green, 1.0);
        assert_eq!(color.blue, 1.0);
    }

    #[test]
    fn test_color_new_int() {
        let color = Color::new_u32(0xFFFFFF);
        assert_eq!(color.red, 1.0);
        assert_eq!(color.green, 1.0);
        assert_eq!(color.blue, 1.0);
    }

    #[test]
    fn test_color_fill() {
        let color = LeftEye::fill(Color::new(0.5, 0.5, 0.5));
        assert_eq!(color.color_0_deg.red, 0.5);
        assert_eq!(color.color_0_deg.green, 0.5);
        assert_eq!(color.color_0_deg.blue, 0.5);
    }

    #[test]
    fn test_joint_array() {
        let t1 = JointArray::builder()
            .left_arm_joints(LeftArmJoints::fill(1))
            .left_leg_joints(LeftLegJoints::fill(2))
            .right_arm_joints(RightArmJoints::fill(3))
            .right_leg_joints(RightLegJoints::fill(4))
            .head_joints(HeadJoints::fill(5))
            .build();

        let t2: Vec<i32> = vec![
            5, 5, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 1, 3,
        ];

        for (i, elem) in t1.iter().enumerate() {
            println!("elems {} {}", i, elem);
            assert_eq!(*elem, t2[i])
        }

        let _: JointArray<i32> = t2.into_iter().collect();
        let _: JointArray<i32> = t1.iter().map(|x| x + 1).collect();
    }
}
