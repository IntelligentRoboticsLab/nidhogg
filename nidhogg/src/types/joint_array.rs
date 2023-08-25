//! Implements `JointArray` type and associated functions, for manipulating joint values.
//!

use crate::types::{
    ArmJoints, FillExt, HeadJoints, LeftArmJoints, LeftLegJoints, LegJoints, RightArmJoints,
    RightLegJoints,
};
use nidhogg_derive::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

    /// Applies a function to all joint values.
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

    /// Zips two joint arrays together.
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
}