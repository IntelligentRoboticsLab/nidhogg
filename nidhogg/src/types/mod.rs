//! Convenience types used to make interacting with the NAO more convenient.
//!

use nidhogg_derive::{Builder, Filler};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

mod joint_array;
pub use joint_array::JointArray;

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

impl Add for Vector3<f32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3<f32> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vector3<f32> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Mul for Vector3<f32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Trait that introduces the [`fill`](`FillExt::fill`) method for a type, which allows filling in all fields with the same value.
pub trait FillExt<T> {
    /// Return a new instance of the type, with all fields set to the provided value.
    fn fill(value: T) -> Self;
}

/// Struct representing the LEDs on top of the NAO robot's head.  
///
/// Each value represents the intensity of a white LED.
#[derive(Builder, Clone, Debug, Default, Filler)]
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
#[derive(Builder, Clone, Debug, Default, Filler)]
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
#[derive(Builder, Clone, Debug, Default, Filler)]
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
#[derive(Builder, Clone, Debug, Default, Filler)]
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
#[derive(Builder, Clone, Debug, Default, Filler)]
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
#[derive(Builder, Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SonarEnabled {
    pub left: bool,
    pub right: bool,
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
#[derive(Builder, Clone, Debug, Default, Filler)]
pub struct HeadJoints<T> {
    pub yaw: T,
    pub pitch: T,
}

/// Wrapper struct containing the left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default, Filler)]
pub struct LeftLegJoints<T> {
    pub hip_yaw_pitch: T,
    pub hip_roll: T,
    pub hip_pitch: T,
    pub knee_pitch: T,
    pub ankle_pitch: T,
    pub ankle_roll: T,
}

/// Wrapper struct containing right left leg joints of the robot.
#[derive(Builder, Clone, Debug, Default, Filler)]
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

impl<T: Clone> FillExt<T> for LegJoints<T> {
    fn fill(value: T) -> LegJoints<T> {
        LegJoints {
            left_leg: LeftLegJoints::fill(value.clone()),
            right_leg: RightLegJoints::fill(value.clone()),
        }
    }
}

/// Wrapper struct containing the joints for a single arm of the robot.
#[derive(Builder, Clone, Debug, Default, Filler)]
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

#[cfg(test)]
mod tests {
    use crate::types::{Color, FillExt, LeftEye};

    use super::ForceSensitiveResistorFoot;
    use super::ForceSensitiveResistors;

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
}
