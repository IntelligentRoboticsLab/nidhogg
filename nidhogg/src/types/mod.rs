//! Convenience types used to make interacting with the NAO more convenient.
//!

use std::ops::{Add, Div, Mul, Neg, Sub};

use nidhogg_derive::{Builder, Filler};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Resource;

pub mod color;
mod joint_array;

pub use color::{Rgb, RgbF32, RgbU8};
pub use joint_array::JointArray;

/// Trait that introduces the [`fill`](`FillExt::fill`) method for a type, which allows filling in all fields with the same value.
pub trait FillExt<T> {
    /// Return a new instance of the type, with all fields set to the provided value.
    fn fill(value: T) -> Self;
}

/// Struct representing the LEDs on top of the NAO robot's head.
///
/// Each value represents the intensity of a white LED.
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
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
///
/// ![Left Ear](https://cdn.dutchnao.team/nidhogg/hardware_led_left_ear.png)
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct LeftEar {
    pub l0: f32,
    pub l1: f32,
    pub l2: f32,
    pub l3: f32,
    pub l4: f32,
    pub l5: f32,
    pub l6: f32,
    pub l7: f32,
    pub l8: f32,
    pub l9: f32,
}

/// Struct representing the LED intensities in the right ear of the robot.
///
/// ## LED order:
/// These LEDs are placed in the following order:
///
/// ![Right Ear](https://cdn.dutchnao.team/nidhogg/hardware_led_right_ear.png)
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct RightEar {
    pub r0: f32,
    pub r1: f32,
    pub r2: f32,
    pub r3: f32,
    pub r4: f32,
    pub r5: f32,
    pub r6: f32,
    pub r7: f32,
    pub r8: f32,
    pub r9: f32,
}

/// Struct representing the RGB LEDs in the left eye of the robot.
/// ## LED order:
/// These LEDs are placed in the following order:
///
/// ![Left Eye](https://cdn.dutchnao.team/nidhogg/hardware_led_left_eye.png)
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct LeftEye {
    pub l0: RgbF32,
    pub l1: RgbF32,
    pub l2: RgbF32,
    pub l3: RgbF32,
    pub l4: RgbF32,
    pub l5: RgbF32,
    pub l6: RgbF32,
    pub l7: RgbF32,
}

/// Struct representing the RGB LEDs in the left eye of the robot.
/// ## LED order:
/// These LEDs are placed in the following order:
///
/// ![Right Eye](https://cdn.dutchnao.team/nidhogg/hardware_led_right_eye.png)
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct RightEye {
    pub r0: RgbF32,
    pub r1: RgbF32,
    pub r2: RgbF32,
    pub r3: RgbF32,
    pub r4: RgbF32,
    pub r5: RgbF32,
    pub r6: RgbF32,
    pub r7: RgbF32,
}

/// Struct representing the battery status of the robot.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
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

/// Struct containing the [`FsrFoot`] value for each foot.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct Fsr {
    /// FSR values from the four sensors in the left foot.
    pub left_foot: FsrFoot,
    /// FSR values from the four sensors in the right foot.
    pub right_foot: FsrFoot,
}

impl Fsr {
    /// Computes the sum of the FSR sensor values for both feet.
    pub fn sum(&self) -> f32 {
        self.left_foot.sum() + self.right_foot.sum()
    }

    /// Compute the sum of the FSR sensor values, weighted by the provided weights.
    pub fn weighted_sum(&self, weights: &Fsr) -> f32 {
        self.left_foot.weighted_sum(&weights.left_foot)
            + self.right_foot.weighted_sum(&weights.right_foot)
    }

    /// Calculates the average weight based on the measurement from the resistors in both feet.
    pub fn avg(&self) -> f32 {
        (self.left_foot.avg() + self.right_foot.avg()) / 2.0
    }
}

impl Add for Fsr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            left_foot: self.left_foot + rhs.left_foot,
            right_foot: self.right_foot + rhs.right_foot,
        }
    }
}

impl Sub for Fsr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            left_foot: self.left_foot - rhs.left_foot,
            right_foot: self.right_foot - rhs.right_foot,
        }
    }
}

impl Mul for Fsr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            left_foot: self.left_foot * rhs.left_foot,
            right_foot: self.right_foot * rhs.right_foot,
        }
    }
}

impl Div for Fsr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            left_foot: self.left_foot / rhs.left_foot,
            right_foot: self.right_foot / rhs.right_foot,
        }
    }
}

impl Neg for Fsr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            left_foot: -self.left_foot,
            right_foot: -self.right_foot,
        }
    }
}

/// Struct representing the force sensitive resistors in one of the feet.
#[derive(Clone, Debug, Default, PartialEq, Filler)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct FsrFoot {
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

impl FsrFoot {
    /// Computes the sum of the FSR sensor values for the foot.
    pub fn sum(&self) -> f32 {
        self.front_left + self.front_right + self.rear_left + self.rear_right
    }

    /// Compute the sum of the FSR sensor values, weighted by the provided weights.
    pub fn weighted_sum(&self, weights: &FsrFoot) -> f32 {
        (weights.front_left * self.front_left)
            + (weights.front_right * self.front_right)
            + (weights.rear_left * self.rear_left)
            + (weights.rear_right * self.rear_right)
    }

    /// Calculates the average weight on the foot.
    pub fn avg(&self) -> f32 {
        self.sum() / 4.0
    }

    /// Computes the total pressure on the front sensors of the foot.
    ///
    /// # Note
    ///
    /// Since this value is the sum of two sensors, it can be up to twice as large
    /// as the reading from a single sensor.
    pub fn forward_pressure(&self) -> f32 {
        self.front_left + self.front_right
    }

    /// Computes the total pressure on the rear sensors of the foot.
    ///
    /// # Note
    ///
    /// Since this value is the sum of two sensors, it can be up to twice as large
    /// as the reading from a single sensor.
    pub fn backward_pressure(&self) -> f32 {
        self.rear_left + self.rear_right
    }

    /// Computes the total pressure on the left sensors of the foot.
    ///
    /// # Note
    ///
    /// Since this value is the sum of two sensors, it can be up to twice as large
    /// as the reading from a single sensor.
    pub fn left_pressure(&self) -> f32 {
        self.front_left + self.rear_left
    }

    /// Computes the total pressure on the right sensors of the foot.
    ///
    /// # Note
    ///
    /// Since this value is the sum of two sensors, it can be up to twice as large
    /// as the reading from a single sensor.
    pub fn right_pressure(&self) -> f32 {
        self.front_right + self.rear_right
    }

    /// Compute the supremum (element-wise maximum) for each sensor value.
    pub fn sup(&self, other: &FsrFoot) -> Self {
        Self {
            front_left: self.front_left.max(other.front_left),
            front_right: self.front_right.max(other.front_right),
            rear_left: self.rear_left.max(other.rear_left),
            rear_right: self.rear_right.max(other.rear_right),
        }
    }

    /// Compute the element-wise maximum for each sensor value.
    ///
    /// # Note
    ///
    /// This is an alias for [`Self::sup`].
    pub fn max_per_sensor(&self, other: &FsrFoot) -> Self {
        self.sup(other)
    }

    /// Compute the infimum (element-wise minimum) for each sensor value.
    pub fn inf(&self, other: &FsrFoot) -> Self {
        Self {
            front_left: self.front_left.min(other.front_left),
            front_right: self.front_right.min(other.front_right),
            rear_left: self.rear_left.min(other.rear_left),
            rear_right: self.rear_right.min(other.rear_right),
        }
    }

    /// Compute the element-wise minimum for each sensor value.
    ///
    /// # Note
    ///
    /// This is an alias for [`Self::inf`].
    pub fn min_per_sensor(&self, other: &FsrFoot) -> Self {
        self.inf(other)
    }
}

impl Add for FsrFoot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            front_left: self.front_left + rhs.front_left,
            front_right: self.front_right + rhs.front_right,
            rear_left: self.rear_left + rhs.rear_left,
            rear_right: self.rear_right + rhs.rear_right,
        }
    }
}

impl Sub for FsrFoot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            front_left: self.front_left - rhs.front_left,
            front_right: self.front_right - rhs.front_right,
            rear_left: self.rear_left - rhs.rear_left,
            rear_right: self.rear_right - rhs.rear_right,
        }
    }
}

impl Mul for FsrFoot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            front_left: self.front_left * rhs.front_left,
            front_right: self.front_right * rhs.front_right,
            rear_left: self.rear_left * rhs.rear_left,
            rear_right: self.rear_right * rhs.rear_right,
        }
    }
}

impl Div for FsrFoot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            front_left: self.front_left / rhs.front_left,
            front_right: self.front_right / rhs.front_right,
            rear_left: self.rear_left / rhs.rear_left,
            rear_right: self.rear_right / rhs.rear_right,
        }
    }
}

impl Neg for FsrFoot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            front_left: -self.front_left,
            front_right: -self.front_right,
            rear_left: -self.rear_left,
            rear_right: -self.rear_right,
        }
    }
}

/// Values read by the left and right sonar sensor.
#[derive(Builder, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct SonarValues {
    /// Left Sonar Value.
    ///
    /// The value ranges from 0 to 5 meters.
    /// A value of 0 means an error.
    /// A value equal to the max detection range means no echo.
    ///
    /// Be aware that:
    /// - The ground will likely be detected before the maximum distance for detection is reached.
    /// - Robot arms might be detected.
    pub left: f32,
    /// Right Sonar Value.
    ///
    /// The value ranges from 0 to 5 meters.
    /// A value of 0 means an error.
    /// A value equal to the max detection range means no echo.
    ///
    /// Be aware that:
    /// - The ground will likely be detected before the maximum distance for detection is reached.
    /// - Robot arms might be detected.
    pub right: f32,
}

/// Enabled state of the left and right sonar sensors.
#[derive(Builder, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct SonarEnabled {
    pub left: bool,
    pub right: bool,
}

/// Struct containing the touch activation value for each touch sensor on the robot.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Resource))]
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
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq, Eq)]
pub struct HeadJoints<T> {
    pub yaw: T,
    pub pitch: T,
}

/// Wrapper struct containing the left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq, Eq)]
pub struct LeftLegJoints<T> {
    pub hip_yaw_pitch: T,
    pub hip_roll: T,
    pub hip_pitch: T,
    pub knee_pitch: T,
    pub ankle_pitch: T,
    pub ankle_roll: T,
}

impl<T> LeftLegJoints<T> {
    /// Transforms each element in the [`LeftLegJoints`] using the provided closure `f`,
    /// producing a new [`LeftLegJoints`] with the transformed values.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::LeftLegJoints;
    ///
    /// let joints = LeftLegJoints::<u32>::default();
    ///
    /// let transformed = joints.map(|x| x + 1);
    ///
    /// assert_eq!(transformed.head_yaw, 1);
    /// ```
    pub fn map<F, U>(self, f: &mut F) -> LeftLegJoints<U>
    where
        F: FnMut(T) -> U,
    {
        LeftLegJoints {
            hip_yaw_pitch: f(self.hip_yaw_pitch),
            hip_roll: f(self.hip_roll),
            hip_pitch: f(self.hip_pitch),
            knee_pitch: f(self.knee_pitch),
            ankle_pitch: f(self.ankle_pitch),
            ankle_roll: f(self.ankle_roll),
        }
    }

    /// Zips two [`LeftLegJoints`] instances element-wise, creating a new [`LeftLegJoints`]
    /// containing tuples of corresponding elements from the two arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let zipped = LeftLegJoints::<f32>::default().zip(LeftLegJoints::<f32>::default());
    ///
    /// assert_eq!(zipped.head_yaw, (0_u32, 0_f32));
    /// ```
    pub fn zip<U>(self, other: LeftLegJoints<U>) -> LeftLegJoints<(T, U)> {
        LeftLegJoints {
            hip_yaw_pitch: (self.hip_yaw_pitch, other.hip_yaw_pitch),
            hip_roll: (self.hip_roll, other.hip_roll),
            hip_pitch: (self.hip_pitch, other.hip_pitch),
            knee_pitch: (self.knee_pitch, other.knee_pitch),
            ankle_pitch: (self.ankle_pitch, other.ankle_pitch),
            ankle_roll: (self.ankle_roll, other.ankle_roll),
        }
    }
}

/// Wrapper struct containing right left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq, Eq)]
pub struct RightLegJoints<T> {
    // This value does not exist
    // pub hip_yaw_pitch: T,
    pub hip_roll: T,
    pub hip_pitch: T,
    pub knee_pitch: T,
    pub ankle_pitch: T,
    pub ankle_roll: T,
}

impl<T> RightLegJoints<T> {
    /// Transforms each element in the [`RightLegJoints`] using the provided closure `f`,
    /// producing a new [`RightLegJoints`] with the transformed values.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::RightLegJoints;
    ///
    /// let joints = RightLegJoints::<u32>::default();
    ///
    /// let transformed = joints.map(|x| x + 1);
    ///
    /// assert_eq!(transformed.head_yaw, 1);
    /// ```
    pub fn map<F, U>(self, f: &mut F) -> RightLegJoints<U>
    where
        F: FnMut(T) -> U,
    {
        RightLegJoints {
            hip_roll: f(self.hip_roll),
            hip_pitch: f(self.hip_pitch),
            knee_pitch: f(self.knee_pitch),
            ankle_pitch: f(self.ankle_pitch),
            ankle_roll: f(self.ankle_roll),
        }
    }

    /// Zips two [`RightLegJoints`] instances element-wise, creating a new [`RightLegJoints`]
    /// containing tuples of corresponding elements from the two arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let zipped = RightLegJoints::<f32>::default().zip(RightLegJoints::<f32>::default());
    ///
    /// assert_eq!(zipped.head_yaw, (0_u32, 0_f32));
    /// ```
    pub fn zip<U>(self, other: RightLegJoints<U>) -> RightLegJoints<(T, U)> {
        RightLegJoints {
            hip_roll: (self.hip_roll, other.hip_roll),
            hip_pitch: (self.hip_pitch, other.hip_pitch),
            knee_pitch: (self.knee_pitch, other.knee_pitch),
            ankle_pitch: (self.ankle_pitch, other.ankle_pitch),
            ankle_roll: (self.ankle_roll, other.ankle_roll),
        }
    }
}

/// Wrapper struct containing joint values for both legs of the robot.
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq)]
pub struct LegJoints<T> {
    pub left_leg: LeftLegJoints<T>,
    pub right_leg: RightLegJoints<T>,
}

impl<T> LegJoints<T> {
    /// Transforms each element in the [`LegJoints`] using the provided closure `f`,
    /// producing a new [`LegJoints`] with the transformed values.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::LegJoints;
    ///
    /// let joints = LegJoints::<u32>::default();
    ///
    /// let transformed = joints.map(|x| x + 1);
    ///
    /// assert_eq!(transformed.head_yaw, 1);
    /// ```
    pub fn map<F, U>(self, mut f: F) -> LegJoints<U>
    where
        F: FnMut(T) -> U,
    {
        LegJoints {
            left_leg: self.left_leg.map(&mut f),
            right_leg: self.right_leg.map(&mut f),
        }
    }

    /// Zips two [`LegJoints`] instances element-wise, creating a new [`LegJoints`]
    /// containing tuples of corresponding elements from the two arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let zipped = LegJoints::<f32>::default().zip(LegJoints::<f32>::default());
    ///
    /// assert_eq!(zipped.head_yaw, (0_u32, 0_f32));
    /// ```
    pub fn zip<U>(self, other: LegJoints<U>) -> LegJoints<(T, U)> {
        LegJoints {
            left_leg: self.left_leg.zip(other.left_leg),
            right_leg: self.right_leg.zip(other.right_leg),
        }
    }
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
#[derive(Builder, Clone, Debug, Default, Filler, PartialEq, Eq)]
pub struct SingleArmJoints<T> {
    pub shoulder_pitch: T,
    pub shoulder_roll: T,
    pub elbow_yaw: T,
    pub elbow_roll: T,
    pub wrist_yaw: T,
    pub hand: T,
}

/// Type definition for the left arm joints of the robot.
/// Introduced for api consistency with [`LeftLegJoints`].
pub type LeftArmJoints<T> = SingleArmJoints<T>;

/// Type definition for the right arm joints of the robot.
/// Introduced for api consistency with [`RightLegJoints`].
pub type RightArmJoints<T> = SingleArmJoints<T>;

/// Wrapper struct containing the arm joints of the robot.
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq)]
pub struct ArmJoints<T> {
    pub left_arm: SingleArmJoints<T>,
    pub right_arm: SingleArmJoints<T>,
}

impl<T: Clone> FillExt<T> for ArmJoints<T> {
    fn fill(value: T) -> ArmJoints<T> {
        ArmJoints {
            left_arm: LeftArmJoints::fill(value.clone()),
            right_arm: RightArmJoints::fill(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FillExt, LeftEye, RgbF32};

    #[test]
    fn test_average_force_feet() {
        let foot1 = FsrFoot {
            front_left: 0.0,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.76,
        };
        let foot2 = FsrFoot {
            front_left: 0.54,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.95,
        };
        let feet = Fsr {
            left_foot: foot1,
            right_foot: foot2,
        };
        assert_eq!(feet.avg(), 0.61125);
    }

    #[test]
    fn test_average_weight_foot() {
        let foot = FsrFoot {
            front_left: 0.0,
            front_right: 1.0,
            rear_left: 0.32,
            rear_right: 0.76,
        };
        assert_eq!(foot.avg(), 0.52);
    }

    #[test]
    fn test_color_new() {
        let color = RgbF32::new(0.5, 0.5, 0.5);
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.5);
        assert_eq!(color.blue, 0.5);
    }

    #[test]
    fn test_color_new_u8() {
        let color = RgbU8::new(255, 255, 255);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
    }

    #[test]
    fn test_color_new_int() {
        let color = RgbU8::from(0xFFFFFF);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
    }

    #[test]
    fn test_color_fill() {
        let color = LeftEye::fill(RgbF32::new(0.5, 0.5, 0.5));
        assert_eq!(color.l0.red, 0.5);
        assert_eq!(color.l0.green, 0.5);
        assert_eq!(color.l0.blue, 0.5);
    }
}
