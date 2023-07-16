use rubullet::nalgebra::Isometry3;
use rubullet::{BodyId, ItemId, JointInfo, LoadModelFlags, PhysicsClient, UrdfOptions};
use std::collections::HashMap;

use crate::types::{ForceSensitiveResistorFoot, ForceSensitiveResistors, JointArray, Touch};
use crate::Result;

macro_rules! to_nidhogg {
    ($target: ident, "HeadYaw") => {
        $target.head_yaw
    };
    ($target: ident, "HeadPitch") => {
        $target.head_pitch
    };
    ($target: ident, "LHipYawPitch") => {
        $target.left_hip_yaw_pitch
    };
    ($target: ident, "LHipRoll") => {
        $target.left_hip_roll
    };
    ($target: ident, "LHipPitch") => {
        $target.left_hip_pitch
    };
    ($target: ident, "LKneePitch") => {
        $target.left_knee_pitch
    };
    ($target: ident, "LAnklePitch") => {
        $target.left_ankle_pitch
    };
    ($target: ident, "LAnkleRoll") => {
        $target.left_ankle_roll
    };
    ($target: ident, "RHipYawPitch") => {
        $target.left_hip_yaw_pitch
    };
    ($target: ident, "RHipRoll") => {
        $target.right_hip_roll
    };
    ($target: ident, "RHipPitch") => {
        $target.right_hip_pitch
    };
    ($target: ident, "RKneePitch") => {
        $target.right_knee_pitch
    };
    ($target: ident, "RAnklePitch") => {
        $target.right_ankle_pitch
    };
    ($target: ident, "RAnkleRoll") => {
        $target.right_ankle_roll
    };
    ($target: ident, "LShoulderPitch") => {
        $target.left_shoulder_pitch
    };
    ($target: ident, "LShoulderRoll") => {
        $target.left_shoulder_roll
    };
    ($target: ident, "LElbowYaw") => {
        $target.left_elbow_yaw
    };
    ($target: ident, "LElbowRoll") => {
        $target.left_elbow_roll
    };
    ($target: ident, "LWristYaw") => {
        $target.left_wrist_yaw
    };
    ($target: ident, "LHand") => {
        $target.left_hand
    };
    ($target: ident, "RShoulderPitch") => {
        $target.right_shoulder_pitch
    };
    ($target: ident, "RShoulderRoll") => {
        $target.right_shoulder_roll
    };
    ($target: ident, "RElbowYaw") => {
        $target.right_elbow_yaw
    };
    ($target: ident, "RElbowRoll") => {
        $target.right_elbow_roll
    };
    ($target: ident, "RWristYaw") => {
        $target.right_wrist_yaw
    };
    ($target: ident, "RHand") => {
        $target.right_hand
    };
}

macro_rules! control_command {
    ($physics_client: ident, $self: ident, $joint_name: tt, $positions: ident, $stiffness: ident) => {
        $physics_client.set_joint_motor_control(
            $self.id,
            $self.joint_map.get($joint_name).unwrap().0.joint_index,
            rubullet::ControlCommand::Position(to_nidhogg!($positions, $joint_name) as f64),
            Some(to_nidhogg!($stiffness, $joint_name) as f64),
        );
    };
}

#[derive(Debug)]
pub struct BulletNao {
    pub id: BodyId,
    pub link_map: HashMap<String, JointInfo>,
    pub joint_map: HashMap<String, BulletJoint>,
    pub touch_input: Touch<ItemId>,
}

impl BulletNao {
    pub fn create(
        physics_client: &mut PhysicsClient,
        start_position: Isometry3<f64>,
    ) -> Result<Self> {
        let id = physics_client.load_urdf(
            "nao.urdf",
            UrdfOptions {
                base_transform: start_position,
                // use self collision and provided nao texture
                flags: LoadModelFlags::URDF_USE_SELF_COLLISION
                    | LoadModelFlags::URDF_USE_MATERIAL_COLORS_FROM_MTL,
                ..Default::default()
            },
        )?;

        let balance_constraint = physics_client.create_constraint(
            id,
            None,
            None,
            None,
            rubullet::JointType::Fixed,
            [0.0; 3],
            Isometry3::identity(),
            start_position,
        )?;

        let mut link_ids = HashMap::new();
        let mut joint_ids = HashMap::new();
        build_link_id_map(physics_client, id, &mut link_ids, &mut joint_ids);

        // TODO: needs more idiomatic rust!
        physics_client.set_collision_filter_pair(
            id,
            id,
            link_ids.get("torso").unwrap().joint_index,
            link_ids.get("Head").unwrap().joint_index,
            false,
        );

        for side in ["L", "R"] {
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Thigh")).unwrap().joint_index,
                link_ids.get(&format!("{side}Hip")).unwrap().joint_index,
                false,
            );
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Bicep")).unwrap().joint_index,
                link_ids.get(&format!("{side}ForeArm")).unwrap().joint_index,
                false,
            );
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Pelvis")).unwrap().joint_index,
                link_ids.get(&format!("{side}Thigh")).unwrap().joint_index,
                false,
            );
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Tibia")).unwrap().joint_index,
                link_ids
                    .get(&format!("{}_ankle", side.to_lowercase()))
                    .unwrap()
                    .joint_index,
                false,
            );
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids
                    .get(&format!("{side}Finger11_link"))
                    .unwrap()
                    .joint_index,
                link_ids
                    .get(&format!("{side}Finger13_link"))
                    .unwrap()
                    .joint_index,
                false,
            );
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids
                    .get(&format!("{side}Finger21_link"))
                    .unwrap()
                    .joint_index,
                link_ids
                    .get(&format!("{side}Finger23_link"))
                    .unwrap()
                    .joint_index,
                false,
            );
        }

        for base_link in ["RThigh", "LThigh", "RBicep", "LBicep"] {
            physics_client.set_collision_filter_pair(
                id,
                id,
                link_ids.get("torso").unwrap().joint_index,
                link_ids.get(base_link).unwrap().joint_index,
                false,
            );
        }

        link_ids.iter().for_each(|(name, link)| {
            for wrist in ["r_wrist", "l_wrist"] {
                let first = format!("{}finger", wrist.chars().next().unwrap());
                let second = format!("{}thumb", wrist.chars().next().unwrap());
                if name.to_lowercase().contains(first.as_str())
                    || name.to_lowercase().contains(second.as_str())
                {
                    physics_client.set_collision_filter_pair(
                        id,
                        id,
                        link_ids.get(wrist).unwrap().joint_index,
                        link.joint_index,
                        false,
                    );
                }
            }
        });

        for joint in joint_ids.values() {
            physics_client.reset_joint_state(id, joint.0.joint_index, 0.0, None)?;
        }

        physics_client.remove_constraint(balance_constraint);

        Ok(BulletNao {
            id,
            link_map: link_ids,
            joint_map: joint_ids,
            touch_input: Touch {
                chest_board: physics_client.add_user_debug_parameter(
                    "chest_board",
                    0.0,
                    1.0,
                    0.0,
                )?,
                head_front: physics_client.add_user_debug_parameter("head_front", 0.0, 1.0, 0.0)?,
                head_middle: physics_client.add_user_debug_parameter(
                    "head_middle",
                    0.0,
                    1.0,
                    0.0,
                )?,
                head_rear: physics_client.add_user_debug_parameter("head_rear", 0.0, 1.0, 0.0)?,
                left_foot_left: physics_client.add_user_debug_parameter(
                    "left_foot_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_foot_right: physics_client.add_user_debug_parameter(
                    "left_foot_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_back: physics_client.add_user_debug_parameter(
                    "left_hand_back",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_left: physics_client.add_user_debug_parameter(
                    "left_hand_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_right: physics_client.add_user_debug_parameter(
                    "left_hand_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_foot_left: physics_client.add_user_debug_parameter(
                    "right_foot_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_foot_right: physics_client.add_user_debug_parameter(
                    "right_foot_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_back: physics_client.add_user_debug_parameter(
                    "right_hand_back",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_left: physics_client.add_user_debug_parameter(
                    "right_hand_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_right: physics_client.add_user_debug_parameter(
                    "right_hand_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
            },
        })
    }

    pub fn set_angles(
        &self,
        physics_client: &mut PhysicsClient,
        positions: JointArray<f32>,
        stiffness: JointArray<f32>,
    ) {
        control_command!(physics_client, self, "HeadYaw", positions, stiffness);
        control_command!(physics_client, self, "HeadPitch", positions, stiffness);
        control_command!(physics_client, self, "LHipYawPitch", positions, stiffness);
        control_command!(physics_client, self, "LHipRoll", positions, stiffness);
        control_command!(physics_client, self, "LHipPitch", positions, stiffness);
        control_command!(physics_client, self, "LKneePitch", positions, stiffness);
        control_command!(physics_client, self, "LAnklePitch", positions, stiffness);
        control_command!(physics_client, self, "LAnkleRoll", positions, stiffness);
        control_command!(physics_client, self, "RHipYawPitch", positions, stiffness);
        control_command!(physics_client, self, "RHipRoll", positions, stiffness);
        control_command!(physics_client, self, "RHipPitch", positions, stiffness);
        control_command!(physics_client, self, "RKneePitch", positions, stiffness);
        control_command!(physics_client, self, "RAnklePitch", positions, stiffness);
        control_command!(physics_client, self, "RAnkleRoll", positions, stiffness);
        control_command!(physics_client, self, "LShoulderPitch", positions, stiffness);
        control_command!(physics_client, self, "LShoulderRoll", positions, stiffness);
        control_command!(physics_client, self, "LElbowYaw", positions, stiffness);
        control_command!(physics_client, self, "LElbowRoll", positions, stiffness);
        control_command!(physics_client, self, "LWristYaw", positions, stiffness);
        control_command!(physics_client, self, "LHand", positions, stiffness);
        control_command!(physics_client, self, "RShoulderPitch", positions, stiffness);
        control_command!(physics_client, self, "RShoulderRoll", positions, stiffness);
        control_command!(physics_client, self, "RElbowYaw", positions, stiffness);
        control_command!(physics_client, self, "RElbowRoll", positions, stiffness);
        control_command!(physics_client, self, "RWristYaw", positions, stiffness);
        control_command!(physics_client, self, "RHand", positions, stiffness);
    }

    pub fn get_touch(&self, physics_client: &mut PhysicsClient) -> Result<Touch<f32>> {
        Ok(Touch {
            chest_board: physics_client
                .read_user_debug_parameter(self.touch_input.chest_board)
                .unwrap_or_default() as f32,
            head_front: physics_client
                .read_user_debug_parameter(self.touch_input.head_front)
                .unwrap_or_default() as f32,
            head_middle: physics_client
                .read_user_debug_parameter(self.touch_input.head_middle)
                .unwrap_or_default() as f32,
            head_rear: physics_client
                .read_user_debug_parameter(self.touch_input.head_rear)
                .unwrap_or_default() as f32,
            left_foot_left: physics_client
                .read_user_debug_parameter(self.touch_input.left_foot_left)
                .unwrap_or_default() as f32,
            left_foot_right: physics_client
                .read_user_debug_parameter(self.touch_input.left_foot_right)
                .unwrap_or_default() as f32,
            left_hand_back: physics_client
                .read_user_debug_parameter(self.touch_input.left_hand_back)
                .unwrap_or_default() as f32,
            left_hand_left: physics_client
                .read_user_debug_parameter(self.touch_input.left_hand_left)
                .unwrap_or_default() as f32,
            left_hand_right: physics_client
                .read_user_debug_parameter(self.touch_input.left_hand_right)
                .unwrap_or_default() as f32,
            right_foot_left: physics_client
                .read_user_debug_parameter(self.touch_input.right_foot_left)
                .unwrap_or_default() as f32,
            right_foot_right: physics_client
                .read_user_debug_parameter(self.touch_input.right_foot_right)
                .unwrap_or_default() as f32,
            right_hand_back: physics_client
                .read_user_debug_parameter(self.touch_input.right_hand_back)
                .unwrap_or_default() as f32,
            right_hand_left: physics_client
                .read_user_debug_parameter(self.touch_input.right_hand_left)
                .unwrap_or_default() as f32,
            right_hand_right: physics_client
                .read_user_debug_parameter(self.touch_input.right_hand_right)
                .unwrap_or_default() as f32,
        })
    }

    pub fn get_fsr(&self, physics_client: &mut PhysicsClient) -> Result<ForceSensitiveResistors> {
        Ok(ForceSensitiveResistors {
            left_foot: ForceSensitiveResistorFoot {
                front_left: self.get_fsr_value(physics_client, "LFsrFL_frame")?,
                front_right: self.get_fsr_value(physics_client, "LFsrFR_frame")?,
                rear_left: self.get_fsr_value(physics_client, "LFsrRL_frame")?,
                rear_right: self.get_fsr_value(physics_client, "LFsrRR_frame")?,
            },
            right_foot: ForceSensitiveResistorFoot {
                front_left: self.get_fsr_value(physics_client, "RFsrFL_frame")?,
                front_right: self.get_fsr_value(physics_client, "RFsrFR_frame")?,
                rear_left: self.get_fsr_value(physics_client, "RFsrRL_frame")?,
                rear_right: self.get_fsr_value(physics_client, "RFsrRR_frame")?,
            },
        })
    }

    fn get_fsr_value(&self, physics_client: &mut PhysicsClient, fsr_link: &str) -> Result<f32> {
        let contact_points = physics_client.get_contact_points(
            self.id,
            None,
            Some(self.link_map.get(fsr_link).map(|f| f.joint_index)),
            None,
        )?;

        let mut total_force = 0f32;
        for contact in contact_points {
            total_force += contact.normal_force.unwrap_or(0.0) as f32;
        }

        Ok(total_force)
    }
}
#[derive(Debug)]
pub struct BulletJoint(JointInfo);

fn build_link_id_map(
    physics_client: &mut PhysicsClient,
    body_id: BodyId,
    link_map: &mut HashMap<String, JointInfo>,
    joint_map: &mut HashMap<String, BulletJoint>,
) {
    for i in 0..physics_client.get_num_joints(body_id) {
        let joint_info = physics_client.get_joint_info(body_id, i);
        match joint_info.joint_type {
            rubullet::JointType::Revolute | rubullet::JointType::Prismatic => {
                joint_map.insert(
                    joint_info.joint_name.clone(),
                    BulletJoint(physics_client.get_joint_info(body_id, i)),
                );
                println!("{}", joint_info.joint_name);
            }
            _ => { // not a joint in nao
            }
        }
        link_map.insert(
            physics_client.get_joint_info(body_id, i).link_name,
            physics_client.get_joint_info(body_id, i),
        );
    }
}
