//! Implements [`JointArray`] type and associated functions, for manipulating joint values.

use std::ops::Sub;

use crate::types::{
    ArmJoints, FillExt, HeadJoints, LeftArmJoints, LeftLegJoints, LegJoints, RightArmJoints,
    RightLegJoints,
};
use nidhogg_derive::Builder;
use num::Signed;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Struct containing values of type `T` for all the joints
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq)]
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
    /// Returns a reference to the joint value at the specified index.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let joints = JointArray::<i32>::default();
    /// assert_eq!(*joints.get(0).unwrap(), 0); // head_yaw
    /// assert_eq!(*joints.get(1).unwrap(), 0); // head_pitch
    /// assert!(joints.get(25).is_none()); // out of bounds
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        match index {
            0 => Some(&self.head_yaw),
            1 => Some(&self.head_pitch),
            2 => Some(&self.left_shoulder_pitch),
            3 => Some(&self.left_shoulder_roll),
            4 => Some(&self.left_elbow_yaw),
            5 => Some(&self.left_elbow_roll),
            6 => Some(&self.left_wrist_yaw),
            7 => Some(&self.left_hip_yaw_pitch),
            8 => Some(&self.left_hip_roll),
            9 => Some(&self.left_hip_pitch),
            10 => Some(&self.left_knee_pitch),
            11 => Some(&self.left_ankle_pitch),
            12 => Some(&self.left_ankle_roll),
            13 => Some(&self.right_shoulder_pitch),
            14 => Some(&self.right_shoulder_roll),
            15 => Some(&self.right_elbow_yaw),
            16 => Some(&self.right_elbow_roll),
            17 => Some(&self.right_wrist_yaw),
            18 => Some(&self.right_hip_roll),
            19 => Some(&self.right_hip_pitch),
            20 => Some(&self.right_knee_pitch),
            21 => Some(&self.right_ankle_pitch),
            22 => Some(&self.right_ankle_roll),
            23 => Some(&self.left_hand),
            24 => Some(&self.right_hand),
            _ => None,
        }
    }

    /// Returns a mutable reference to the joint value at the specified index.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let mut joints = JointArray::<i32>::default();
    ///
    /// if let Some(value) = joints.get_mut(0) {
    ///     *value = 42;
    /// }
    ///
    /// assert_eq!(joints.head_yaw, 42);
    /// assert!(joints.get_mut(25).is_none()); // out of bounds
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index {
            0 => Some(&mut self.head_yaw),
            1 => Some(&mut self.head_pitch),
            2 => Some(&mut self.left_shoulder_pitch),
            3 => Some(&mut self.left_shoulder_roll),
            4 => Some(&mut self.left_elbow_yaw),
            5 => Some(&mut self.left_elbow_roll),
            6 => Some(&mut self.left_wrist_yaw),
            7 => Some(&mut self.left_hip_yaw_pitch),
            8 => Some(&mut self.left_hip_roll),
            9 => Some(&mut self.left_hip_pitch),
            10 => Some(&mut self.left_knee_pitch),
            11 => Some(&mut self.left_ankle_pitch),
            12 => Some(&mut self.left_ankle_roll),
            13 => Some(&mut self.right_shoulder_pitch),
            14 => Some(&mut self.right_shoulder_roll),
            15 => Some(&mut self.right_elbow_yaw),
            16 => Some(&mut self.right_elbow_roll),
            17 => Some(&mut self.right_wrist_yaw),
            18 => Some(&mut self.right_hip_roll),
            19 => Some(&mut self.right_hip_pitch),
            20 => Some(&mut self.right_knee_pitch),
            21 => Some(&mut self.right_ankle_pitch),
            22 => Some(&mut self.right_ankle_roll),
            23 => Some(&mut self.left_hand),
            24 => Some(&mut self.right_hand),
            _ => None,
        }
    }

    /// Returns all joint values as a fixed-size array.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let joints = JointArray::<i32>::default();
    /// let values = joints.as_array();
    /// assert_eq!(values.len(), 25);
    /// assert!(values.iter().all(|&v| v == 0));
    /// ```
    pub fn as_array(self) -> [T; 25] {
        [
            self.head_yaw,
            self.head_pitch,
            self.left_shoulder_pitch,
            self.left_shoulder_roll,
            self.left_elbow_yaw,
            self.left_elbow_roll,
            self.left_wrist_yaw,
            self.left_hip_yaw_pitch,
            self.left_hip_roll,
            self.left_hip_pitch,
            self.left_knee_pitch,
            self.left_ankle_pitch,
            self.left_ankle_roll,
            self.right_shoulder_pitch,
            self.right_shoulder_roll,
            self.right_elbow_yaw,
            self.right_elbow_roll,
            self.right_wrist_yaw,
            self.right_hip_roll,
            self.right_hip_pitch,
            self.right_knee_pitch,
            self.right_ankle_pitch,
            self.right_ankle_roll,
            self.left_hand,
            self.right_hand,
        ]
    }

    /// Returns a reference to all joint values as a fixed-size array.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let joints = JointArray::<i32>::default();
    /// let values = joints.as_array_ref();
    /// assert_eq!(values.len(), 25);
    /// assert!(values.iter().all(|&v| v == 0));
    /// ```
    pub fn as_array_ref(&self) -> [&T; 25] {
        [
            &self.head_yaw,
            &self.head_pitch,
            &self.left_shoulder_pitch,
            &self.left_shoulder_roll,
            &self.left_elbow_yaw,
            &self.left_elbow_roll,
            &self.left_wrist_yaw,
            &self.left_hip_yaw_pitch,
            &self.left_hip_roll,
            &self.left_hip_pitch,
            &self.left_knee_pitch,
            &self.left_ankle_pitch,
            &self.left_ankle_roll,
            &self.right_shoulder_pitch,
            &self.right_shoulder_roll,
            &self.right_elbow_yaw,
            &self.right_elbow_roll,
            &self.right_wrist_yaw,
            &self.right_hip_roll,
            &self.right_hip_pitch,
            &self.right_knee_pitch,
            &self.right_ankle_pitch,
            &self.right_ankle_roll,
            &self.left_hand,
            &self.right_hand,
        ]
    }

    /// Returns a mutable reference to all joint values as a fixed-size array.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let mut joints = JointArray::<i32>::default();
    /// for joint in joints.as_mut() {
    ///     *joint = 42;
    /// }
    /// assert!(joints.as_ref().iter().all(|&v| v == 42));
    /// ```
    pub fn as_array_mut(&mut self) -> [&mut T; 25] {
        [
            &mut self.head_yaw,
            &mut self.head_pitch,
            &mut self.left_shoulder_pitch,
            &mut self.left_shoulder_roll,
            &mut self.left_elbow_yaw,
            &mut self.left_elbow_roll,
            &mut self.left_wrist_yaw,
            &mut self.left_hip_yaw_pitch,
            &mut self.left_hip_roll,
            &mut self.left_hip_pitch,
            &mut self.left_knee_pitch,
            &mut self.left_ankle_pitch,
            &mut self.left_ankle_roll,
            &mut self.right_shoulder_pitch,
            &mut self.right_shoulder_roll,
            &mut self.right_elbow_yaw,
            &mut self.right_elbow_roll,
            &mut self.right_wrist_yaw,
            &mut self.right_hip_roll,
            &mut self.right_hip_pitch,
            &mut self.right_knee_pitch,
            &mut self.right_ankle_pitch,
            &mut self.right_ankle_roll,
            &mut self.left_hand,
            &mut self.right_hand,
        ]
    }

    /// Transforms each element in the [`JointArray`] using the provided closure `f`,
    /// producing a new [`JointArray`] with the transformed values.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let joints = JointArray::<u32>::default();
    ///
    /// let transformed = joints.map(|x| x + 1);
    ///
    /// assert_eq!(transformed.head_yaw, 1);
    /// ```
    pub fn map<F, U>(self, mut f: F) -> JointArray<U>
    where
        F: FnMut(T) -> U,
    {
        JointArray {
            head_yaw: f(self.head_yaw),
            head_pitch: f(self.head_pitch),
            left_shoulder_pitch: f(self.left_shoulder_pitch),
            left_shoulder_roll: f(self.left_shoulder_roll),
            left_elbow_yaw: f(self.left_elbow_yaw),
            left_elbow_roll: f(self.left_elbow_roll),
            left_wrist_yaw: f(self.left_wrist_yaw),
            left_hip_yaw_pitch: f(self.left_hip_yaw_pitch),
            left_hip_roll: f(self.left_hip_roll),
            left_hip_pitch: f(self.left_hip_pitch),
            left_knee_pitch: f(self.left_knee_pitch),
            left_ankle_pitch: f(self.left_ankle_pitch),
            left_ankle_roll: f(self.left_ankle_roll),
            right_shoulder_pitch: f(self.right_shoulder_pitch),
            right_shoulder_roll: f(self.right_shoulder_roll),
            right_elbow_yaw: f(self.right_elbow_yaw),
            right_elbow_roll: f(self.right_elbow_roll),
            right_wrist_yaw: f(self.right_wrist_yaw),
            right_hip_roll: f(self.right_hip_roll),
            right_hip_pitch: f(self.right_hip_pitch),
            right_knee_pitch: f(self.right_knee_pitch),
            right_ankle_pitch: f(self.right_ankle_pitch),
            right_ankle_roll: f(self.right_ankle_roll),
            left_hand: f(self.left_hand),
            right_hand: f(self.right_hand),
        }
    }

    /// Zips two [`JointArray`] instances element-wise, creating a new [`JointArray`]
    /// containing tuples of corresponding elements from the two arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let zipped = JointArray::<u32>::default().zip(JointArray::<f32>::default());
    ///
    /// assert_eq!(zipped.head_yaw, (0_u32, 0_f32));
    /// ```
    pub fn zip<U>(self, other: JointArray<U>) -> JointArray<(T, U)> {
        JointArray {
            head_yaw: (self.head_yaw, other.head_yaw),
            head_pitch: (self.head_pitch, other.head_pitch),
            left_shoulder_pitch: (self.left_shoulder_pitch, other.left_shoulder_pitch),
            left_shoulder_roll: (self.left_shoulder_roll, other.left_shoulder_roll),
            left_elbow_yaw: (self.left_elbow_yaw, other.left_elbow_yaw),
            left_elbow_roll: (self.left_elbow_roll, other.left_elbow_roll),
            left_wrist_yaw: (self.left_wrist_yaw, other.left_wrist_yaw),
            left_hip_yaw_pitch: (self.left_hip_yaw_pitch, other.left_hip_yaw_pitch),
            left_hip_roll: (self.left_hip_roll, other.left_hip_roll),
            left_hip_pitch: (self.left_hip_pitch, other.left_hip_pitch),
            left_knee_pitch: (self.left_knee_pitch, other.left_knee_pitch),
            left_ankle_pitch: (self.left_ankle_pitch, other.left_ankle_pitch),
            left_ankle_roll: (self.left_ankle_roll, other.left_ankle_roll),
            right_shoulder_pitch: (self.right_shoulder_pitch, other.right_shoulder_pitch),
            right_shoulder_roll: (self.right_shoulder_roll, other.right_shoulder_roll),
            right_elbow_yaw: (self.right_elbow_yaw, other.right_elbow_yaw),
            right_elbow_roll: (self.right_elbow_roll, other.right_elbow_roll),
            right_wrist_yaw: (self.right_wrist_yaw, other.right_wrist_yaw),
            right_hip_roll: (self.right_hip_roll, other.right_hip_roll),
            right_hip_pitch: (self.right_hip_pitch, other.right_hip_pitch),
            right_knee_pitch: (self.right_knee_pitch, other.right_knee_pitch),
            right_ankle_pitch: (self.right_ankle_pitch, other.right_ankle_pitch),
            right_ankle_roll: (self.right_ankle_roll, other.right_ankle_roll),
            left_hand: (self.left_hand, other.left_hand),
            right_hand: (self.right_hand, other.right_hand),
        }
    }

    /// Checks if all elements of a joint array satisfy a certain condition.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let mut t1: JointArray<i32> = JointArray::default();
    /// assert_eq!(t1.clone().all(|elem| elem > -1), true);
    ///
    /// t1.right_hand = -2;
    /// assert_eq!(t1.all(|elem| elem > -1), false);
    /// ```
    pub fn all<F>(self, mut f: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        !self.any(|elem| !f(elem))
    }

    /// Checks if any elements of a joint array satisfy a certain condition.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let mut t1: JointArray<i32> = JointArray::default();
    /// assert_eq!(t1.clone().any(|elem| elem > 2), false);
    ///
    /// t1.head_pitch = 3;
    /// assert_eq!(t1.any(|elem| elem > 2), true);
    /// ```
    pub fn any<F>(self, f: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        let t = self.map(f);

        t.head_yaw
            || t.head_pitch
            || t.left_shoulder_pitch
            || t.left_shoulder_roll
            || t.left_elbow_yaw
            || t.left_elbow_roll
            || t.left_wrist_yaw
            || t.left_hip_yaw_pitch
            || t.left_hip_roll
            || t.left_hip_pitch
            || t.left_knee_pitch
            || t.left_ankle_pitch
            || t.left_ankle_roll
            || t.right_shoulder_pitch
            || t.right_shoulder_roll
            || t.right_elbow_yaw
            || t.right_elbow_roll
            || t.right_wrist_yaw
            || t.right_hip_roll
            || t.right_hip_pitch
            || t.right_knee_pitch
            || t.right_ankle_pitch
            || t.right_ankle_roll
            || t.left_hand
            || t.right_hand
    }

    /// Calculates the absolute difference between two joint arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    /// use crate::nidhogg::types::FillExt;
    ///
    /// let t1: JointArray<f32> = JointArray::<f32>::fill(1.0);
    /// let t2: JointArray<f32> = JointArray::<f32>::fill(2.0);
    /// let t3: JointArray<f32> = JointArray::<f32>::fill(3.0);
    ///
    /// assert_eq!(t3.diff(t2).zip(t1).any(|(elem1, elem2)| elem1 != elem2), false);
    /// ```
    pub fn diff(&self, other: JointArray<T>) -> JointArray<T>
    where
        T: Sub<Output = T> + Signed + Clone,
    {
        self.clone()
            .zip(other.clone())
            .map(|(curr, target)| (curr - target).abs())
    }

    /// Creates a new [`JointArray`] containing references to each joint value.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let joints = JointArray::<i32>::default();
    /// let refs = joints.as_ref();
    /// assert_eq!(*refs.head_yaw, 0);
    /// assert_eq!(*refs.left_hand, 0);
    /// ```
    pub fn as_ref(&self) -> JointArray<&T> {
        JointArray {
            head_yaw: &self.head_yaw,
            head_pitch: &self.head_pitch,
            left_shoulder_pitch: &self.left_shoulder_pitch,
            left_shoulder_roll: &self.left_shoulder_roll,
            left_elbow_yaw: &self.left_elbow_yaw,
            left_elbow_roll: &self.left_elbow_roll,
            left_wrist_yaw: &self.left_wrist_yaw,
            left_hip_yaw_pitch: &self.left_hip_yaw_pitch,
            left_hip_roll: &self.left_hip_roll,
            left_hip_pitch: &self.left_hip_pitch,
            left_knee_pitch: &self.left_knee_pitch,
            left_ankle_pitch: &self.left_ankle_pitch,
            left_ankle_roll: &self.left_ankle_roll,
            right_shoulder_pitch: &self.right_shoulder_pitch,
            right_shoulder_roll: &self.right_shoulder_roll,
            right_elbow_yaw: &self.right_elbow_yaw,
            right_elbow_roll: &self.right_elbow_roll,
            right_wrist_yaw: &self.right_wrist_yaw,
            right_hip_roll: &self.right_hip_roll,
            right_hip_pitch: &self.right_hip_pitch,
            right_knee_pitch: &self.right_knee_pitch,
            right_ankle_pitch: &self.right_ankle_pitch,
            right_ankle_roll: &self.right_ankle_roll,
            left_hand: &self.left_hand,
            right_hand: &self.right_hand,
        }
    }

    /// Creates a new [`JointArray`] containing mutable references to each joint value.
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    ///
    /// let mut joints = JointArray::<i32>::default();
    /// let mut refs = joints.as_mut();
    /// *refs.head_yaw = 42;
    /// *refs.left_hand = 17;
    ///
    /// assert_eq!(joints.head_yaw, 42);
    /// assert_eq!(joints.left_hand, 17);
    /// ```
    pub fn as_mut(&mut self) -> JointArray<&mut T> {
        JointArray {
            head_yaw: &mut self.head_yaw,
            head_pitch: &mut self.head_pitch,
            left_shoulder_pitch: &mut self.left_shoulder_pitch,
            left_shoulder_roll: &mut self.left_shoulder_roll,
            left_elbow_yaw: &mut self.left_elbow_yaw,
            left_elbow_roll: &mut self.left_elbow_roll,
            left_wrist_yaw: &mut self.left_wrist_yaw,
            left_hip_yaw_pitch: &mut self.left_hip_yaw_pitch,
            left_hip_roll: &mut self.left_hip_roll,
            left_hip_pitch: &mut self.left_hip_pitch,
            left_knee_pitch: &mut self.left_knee_pitch,
            left_ankle_pitch: &mut self.left_ankle_pitch,
            left_ankle_roll: &mut self.left_ankle_roll,
            right_shoulder_pitch: &mut self.right_shoulder_pitch,
            right_shoulder_roll: &mut self.right_shoulder_roll,
            right_elbow_yaw: &mut self.right_elbow_yaw,
            right_elbow_roll: &mut self.right_elbow_roll,
            right_wrist_yaw: &mut self.right_wrist_yaw,
            right_hip_roll: &mut self.right_hip_roll,
            right_hip_pitch: &mut self.right_hip_pitch,
            right_knee_pitch: &mut self.right_knee_pitch,
            right_ankle_pitch: &mut self.right_ankle_pitch,
            right_ankle_roll: &mut self.right_ankle_roll,
            left_hand: &mut self.left_hand,
            right_hand: &mut self.right_hand,
        }
    }

    /// Converts the [`JointArray<T>`] into a [`Vec<T>`]
    ///
    /// # Example
    ///
    /// ```
    /// use nidhogg::types::JointArray;
    /// use nidhogg::types::FillExt;
    ///
    /// let joints = JointArray::<i32>::fill(5);
    /// let vec = joints.to_vec();
    ///
    /// assert_eq!(vec.len(), 25);
    /// assert!(vec.iter().all(|&v| v == 5));
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        vec![
            self.head_yaw,
            self.head_pitch,
            self.left_shoulder_pitch,
            self.left_shoulder_roll,
            self.left_elbow_yaw,
            self.left_elbow_roll,
            self.left_wrist_yaw,
            self.left_hip_yaw_pitch,
            self.left_hip_roll,
            self.left_hip_pitch,
            self.left_knee_pitch,
            self.left_ankle_pitch,
            self.left_ankle_roll,
            self.right_shoulder_pitch,
            self.right_shoulder_roll,
            self.right_elbow_yaw,
            self.right_elbow_roll,
            self.right_wrist_yaw,
            self.right_hip_roll,
            self.right_hip_pitch,
            self.right_knee_pitch,
            self.right_ankle_pitch,
            self.right_ankle_roll,
            self.left_hand,
            self.right_hand,
        ]
    }
}

impl<'a, T> From<&'a JointArray<T>> for JointArray<&'a T> {
    fn from(value: &'a JointArray<T>) -> Self {
        value.as_ref()
    }
}

impl<'a, T> From<&'a mut JointArray<T>> for JointArray<&'a mut T> {
    fn from(value: &'a mut JointArray<T>) -> Self {
        value.as_mut()
    }
}

impl<T: Clone> JointArray<T> {
    /// Retrieves leg joints for both left and right legs.
    pub fn leg_joints(&self) -> LegJoints<T> {
        LegJoints {
            left_leg: self.left_leg_joints(),
            right_leg: self.right_leg_joints(),
        }
    }

    /// Retrieves arm joints for both left and right arms.
    pub fn arm_joints(&self) -> ArmJoints<T> {
        ArmJoints {
            left_arm: self.left_arm_joints(),
            right_arm: self.right_arm_joints(),
        }
    }

    /// Retrieves the left leg joints.
    pub fn left_leg_joints(&self) -> LeftLegJoints<T> {
        LeftLegJoints {
            hip_yaw_pitch: self.left_hip_yaw_pitch.clone(),
            hip_roll: self.left_hip_roll.clone(),
            hip_pitch: self.left_hip_pitch.clone(),
            knee_pitch: self.left_knee_pitch.clone(),
            ankle_pitch: self.left_ankle_pitch.clone(),
            ankle_roll: self.left_ankle_roll.clone(),
        }
    }

    /// Retrieves the left arm joints.
    pub fn left_arm_joints(&self) -> LeftArmJoints<T> {
        LeftArmJoints {
            shoulder_pitch: self.left_shoulder_pitch.clone(),
            shoulder_roll: self.left_shoulder_roll.clone(),
            elbow_yaw: self.left_elbow_yaw.clone(),
            elbow_roll: self.left_elbow_roll.clone(),
            wrist_yaw: self.left_wrist_yaw.clone(),
            hand: self.left_hand.clone(),
        }
    }

    /// Retrieves the right leg joints.
    pub fn right_leg_joints(&self) -> RightLegJoints<T> {
        RightLegJoints {
            hip_roll: self.right_hip_roll.clone(),
            hip_pitch: self.right_hip_pitch.clone(),
            knee_pitch: self.right_knee_pitch.clone(),
            ankle_pitch: self.right_ankle_pitch.clone(),
            ankle_roll: self.right_ankle_roll.clone(),
        }
    }

    /// Retrieves the right arm joints.
    pub fn right_arm_joints(&self) -> RightArmJoints<T> {
        RightArmJoints {
            shoulder_pitch: self.right_shoulder_pitch.clone(),
            shoulder_roll: self.right_shoulder_roll.clone(),
            elbow_yaw: self.right_elbow_yaw.clone(),
            elbow_roll: self.right_elbow_roll.clone(),
            wrist_yaw: self.right_wrist_yaw.clone(),
            hand: self.right_hand.clone(),
        }
    }

    /// Retrieves the head joints.
    pub fn head_joints(&self) -> HeadJoints<T> {
        HeadJoints {
            yaw: self.head_yaw.clone(),
            pitch: self.head_pitch.clone(),
        }
    }
}

impl<T: Clone> FillExt<T> for JointArray<T> {
    fn fill(value: T) -> JointArray<T> {
        JointArray {
            head_yaw: value.clone(),
            head_pitch: value.clone(),
            left_shoulder_pitch: value.clone(),
            left_shoulder_roll: value.clone(),
            left_elbow_yaw: value.clone(),
            left_elbow_roll: value.clone(),
            left_wrist_yaw: value.clone(),
            left_hip_yaw_pitch: value.clone(),
            left_hip_roll: value.clone(),
            left_hip_pitch: value.clone(),
            left_knee_pitch: value.clone(),
            left_ankle_pitch: value.clone(),
            left_ankle_roll: value.clone(),
            right_shoulder_pitch: value.clone(),
            right_shoulder_roll: value.clone(),
            right_elbow_yaw: value.clone(),
            right_elbow_roll: value.clone(),
            right_wrist_yaw: value.clone(),
            right_hip_roll: value.clone(),
            right_hip_pitch: value.clone(),
            right_knee_pitch: value.clone(),
            right_ankle_pitch: value.clone(),
            right_ankle_roll: value.clone(),
            left_hand: value.clone(),
            right_hand: value.clone(),
        }
    }
}

impl<T: Clone> TryFrom<&[T]> for JointArray<T> {
    type Error = &'static str;

    fn try_from(values: &[T]) -> Result<Self, Self::Error> {
        if values.len() != 25 {
            return Err("Slice must contain exactly 25 elements to convert to JointArray");
        }

        Ok(JointArray {
            head_yaw: values[0].clone(),
            head_pitch: values[1].clone(),
            left_shoulder_pitch: values[2].clone(),
            left_shoulder_roll: values[3].clone(),
            left_elbow_yaw: values[4].clone(),
            left_elbow_roll: values[5].clone(),
            left_wrist_yaw: values[6].clone(),
            left_hip_yaw_pitch: values[7].clone(),
            left_hip_roll: values[8].clone(),
            left_hip_pitch: values[9].clone(),
            left_knee_pitch: values[10].clone(),
            left_ankle_pitch: values[11].clone(),
            left_ankle_roll: values[12].clone(),
            right_shoulder_pitch: values[13].clone(),
            right_shoulder_roll: values[14].clone(),
            right_elbow_yaw: values[15].clone(),
            right_elbow_roll: values[16].clone(),
            right_wrist_yaw: values[17].clone(),
            right_hip_roll: values[18].clone(),
            right_hip_pitch: values[19].clone(),
            right_knee_pitch: values[20].clone(),
            right_ankle_pitch: values[21].clone(),
            right_ankle_roll: values[22].clone(),
            left_hand: values[23].clone(),
            right_hand: values[24].clone(),
        })
    }
}

impl<T> JointArrayBuilder<T> {
    /// Set all the joint values to the corresponding values from the provided [`JointArray`].
    pub fn joints(mut self, joints: JointArray<T>) -> Self {
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

        self
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

impl<T> IntoIterator for JointArray<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 25>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a JointArray<T> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 25>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array_ref().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::FillExt;
    use crate::types::JointArray;

    #[test]
    fn test_joint_array_map() {
        let t1 = JointArray::fill(1);
        let t2 = t1.map(|elem| elem + 1);
        assert_eq!(t2.head_pitch, 2);
        assert_eq!(t2.left_elbow_yaw, 2);
    }

    #[test]
    fn test_joint_array_zip() {
        let t1 = JointArray::fill(1);
        let t2 = JointArray::fill(2);

        let t3 = t1.zip(t2);
        assert_eq!(t3.head_pitch, (1, 2));
        assert_eq!(t3.left_elbow_yaw, (1, 2));
    }

    #[test]
    fn test_get() {
        let joints = JointArray::<i32>::fill(5);

        // Test valid indices
        assert_eq!(*joints.get(0).unwrap(), 5); // head_yaw
        assert_eq!(*joints.get(7).unwrap(), 5); // left_hip_yaw_pitch
        assert_eq!(*joints.get(24).unwrap(), 5); // right_hand

        assert_eq!(*joints.get(0).unwrap(), 5); // First element
        assert_eq!(*joints.get(24).unwrap(), 5); // Last element
        assert!(joints.get(25).is_none()); // Out of bounds
        assert!(joints.get(100).is_none()); // Far out of bounds
    }

    #[test]
    fn test_get_mut() {
        let mut joints = JointArray::<i32>::fill(5);

        if let Some(value) = joints.get_mut(0) {
            *value = 10;
        }
        assert_eq!(joints.head_yaw, 10);
        if let Some(value) = joints.get_mut(13) {
            *value = 20;
        }

        assert_eq!(joints.right_shoulder_pitch, 20);
        assert!(joints.get_mut(25).is_none());
    }
    #[test]
    fn test_as_array() {
        let joints = JointArray::<i32>::fill(3);
        let array = joints.as_array();

        assert_eq!(array.len(), 25);
        assert!(array.iter().all(|&v| v == 3));
    }

    #[test]
    fn test_to_vec() {
        let custom_joints = JointArray::<i32> {
            head_yaw: 10,
            left_hand: 20,
            right_knee_pitch: 30,
            ..Default::default()
        };
        let vec = custom_joints.to_vec();

        assert_eq!(vec[0], 10); // head_yaw
        assert_eq!(vec[23], 20); // left_hand
        assert_eq!(vec[20], 30); // right_knee_pitch
    }

    #[test]
    fn test_try_from_slice_too_long() {
        let long_slice = [3; 26];
        let long_result = JointArray::<i32>::try_from(&long_slice[..]);
        assert!(long_result.is_err());
        assert_eq!(
            long_result.unwrap_err(),
            "Slice must contain exactly 25 elements to convert to JointArray"
        );
    }

    #[test]
    fn test_try_from_slice_custom_values() {
        // Test with custom values
        let custom_slice = [
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34,
        ];
        let custom_result = JointArray::<i32>::try_from(&custom_slice[..]);
        assert!(custom_result.is_ok());
        let custom_joints = custom_result.unwrap();

        assert_eq!(custom_joints.head_yaw, 10);
        assert_eq!(custom_joints.head_pitch, 11);
        assert_eq!(custom_joints.left_shoulder_pitch, 12);
        assert_eq!(custom_joints.right_hand, 34);
    }

    #[test]
    fn test_get_specific_joints() {
        let joints = JointArray::<i32> {
            head_yaw: 1,
            head_pitch: 2,
            left_shoulder_pitch: 3,
            left_hand: 4,
            right_hand: 5,
            ..Default::default()
        };

        assert_eq!(*joints.get(0).unwrap(), 1); // head_yaw
        assert_eq!(*joints.get(1).unwrap(), 2); // head_pitch
        assert_eq!(*joints.get(2).unwrap(), 3); // left_shoulder_pitch
        assert_eq!(*joints.get(23).unwrap(), 4); // left_hand
        assert_eq!(*joints.get(24).unwrap(), 5); // right_hand
    }

    #[test]
    fn test_to_vec_and_try_from_roundtrip() {
        let original = JointArray::<i32> {
            head_yaw: 42,
            left_knee_pitch: 100,
            right_hand: 200,
            ..Default::default()
        };

        let vec = original.clone().to_vec();
        let reconstructed = JointArray::<i32>::try_from(&vec[..]).unwrap();

        // Verify the reconstructed array matches the original
        assert_eq!(reconstructed.head_yaw, 42);
        assert_eq!(reconstructed.left_knee_pitch, 100);
        assert_eq!(reconstructed.right_hand, 200);

        // Verify all fields are equal
        for i in 0..25 {
            assert_eq!(original.get(i), reconstructed.get(i));
        }
    }
}
