use rubullet::nalgebra::Isometry3;
use rubullet::{BodyId, ItemId, JointInfo, LoadModelFlags, PhysicsClient, UrdfOptions};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::types::{JointArray, Touch};
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
        physics_client: Arc<Mutex<PhysicsClient>>,
        start_position: Isometry3<f64>,
    ) -> Result<Self> {
        let mut pc = physics_client.lock().unwrap();
        let id = pc.load_urdf(
            "nao.urdf",
            UrdfOptions {
                base_transform: start_position,
                // use self collision and provided nao texture
                flags: LoadModelFlags::URDF_USE_SELF_COLLISION
                    | LoadModelFlags::URDF_USE_MATERIAL_COLORS_FROM_MTL,
                ..Default::default()
            },
        )?;

        let balance_constraint = pc.create_constraint(
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
        drop(pc);
        build_link_id_map(physics_client.clone(), id, &mut link_ids, &mut joint_ids);

        let mut pc = physics_client.lock().unwrap();
        // TODO: needs more idiomatic rust!
        pc.set_collision_filter_pair(
            id,
            id,
            link_ids.get("torso").unwrap().joint_index,
            link_ids.get("Head").unwrap().joint_index,
            false,
        );

        for side in ["L", "R"] {
            pc.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Thigh")).unwrap().joint_index,
                link_ids.get(&format!("{side}Hip")).unwrap().joint_index,
                false,
            );
            pc.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Bicep")).unwrap().joint_index,
                link_ids.get(&format!("{side}ForeArm")).unwrap().joint_index,
                false,
            );
            pc.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Pelvis")).unwrap().joint_index,
                link_ids.get(&format!("{side}Thigh")).unwrap().joint_index,
                false,
            );
            pc.set_collision_filter_pair(
                id,
                id,
                link_ids.get(&format!("{side}Tibia")).unwrap().joint_index,
                link_ids
                    .get(&format!("{}_ankle", side.to_lowercase()))
                    .unwrap()
                    .joint_index,
                false,
            );
            pc.set_collision_filter_pair(
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
            pc.set_collision_filter_pair(
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
            pc.set_collision_filter_pair(
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
                    pc.set_collision_filter_pair(
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
            pc.reset_joint_state(id, joint.0.joint_index, 0.0, None)?;
        }

        pc.remove_constraint(balance_constraint);

        Ok(BulletNao {
            id,
            link_map: link_ids,
            joint_map: joint_ids,
            touch_input: Touch {
                chest_board: pc.add_user_debug_parameter(
                    "chest_board",
                    0.0,
                    1.0,
                    0.0,
                )?,
                head_front: pc.add_user_debug_parameter("head_front", 0.0, 1.0, 0.0)?,
                head_middle: pc.add_user_debug_parameter(
                    "head_middle",
                    0.0,
                    1.0,
                    0.0,
                )?,
                head_rear: pc.add_user_debug_parameter("head_rear", 0.0, 1.0, 0.0)?,
                left_foot_left: pc.add_user_debug_parameter(
                    "left_foot_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_foot_right: pc.add_user_debug_parameter(
                    "left_foot_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_back: pc.add_user_debug_parameter(
                    "left_hand_back",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_left: pc.add_user_debug_parameter(
                    "left_hand_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                left_hand_right: pc.add_user_debug_parameter(
                    "left_hand_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_foot_left: pc.add_user_debug_parameter(
                    "right_foot_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_foot_right: pc.add_user_debug_parameter(
                    "right_foot_right",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_back: pc.add_user_debug_parameter(
                    "right_hand_back",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_left: pc.add_user_debug_parameter(
                    "right_hand_left",
                    0.0,
                    1.0,
                    0.0,
                )?,
                right_hand_right: pc.add_user_debug_parameter(
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
        physics_client: Arc<Mutex<PhysicsClient>>,
        positions: JointArray<f32>,
        stiffness: JointArray<f32>,
    ) {
        let mut pc = physics_client.lock().unwrap();
        control_command!(pc, self, "HeadYaw", positions, stiffness);
        control_command!(pc, self, "HeadPitch", positions, stiffness);
        control_command!(pc, self, "LHipYawPitch", positions, stiffness);
        control_command!(pc, self, "LHipRoll", positions, stiffness);
        control_command!(pc, self, "LHipPitch", positions, stiffness);
        control_command!(pc, self, "LKneePitch", positions, stiffness);
        control_command!(pc, self, "LAnklePitch", positions, stiffness);
        control_command!(pc, self, "LAnkleRoll", positions, stiffness);
        control_command!(pc, self, "RHipYawPitch", positions, stiffness);
        control_command!(pc, self, "RHipRoll", positions, stiffness);
        control_command!(pc, self, "RHipPitch", positions, stiffness);
        control_command!(pc, self, "RKneePitch", positions, stiffness);
        control_command!(pc, self, "RAnklePitch", positions, stiffness);
        control_command!(pc, self, "RAnkleRoll", positions, stiffness);
        control_command!(pc, self, "LShoulderPitch", positions, stiffness);
        control_command!(pc, self, "LShoulderRoll", positions, stiffness);
        control_command!(pc, self, "LElbowYaw", positions, stiffness);
        control_command!(pc, self, "LElbowRoll", positions, stiffness);
        control_command!(pc, self, "LWristYaw", positions, stiffness);
        control_command!(pc, self, "LHand", positions, stiffness);
        control_command!(pc, self, "RShoulderPitch", positions, stiffness);
        control_command!(pc, self, "RShoulderRoll", positions, stiffness);
        control_command!(pc, self, "RElbowYaw", positions, stiffness);
        control_command!(pc, self, "RElbowRoll", positions, stiffness);
        control_command!(pc, self, "RWristYaw", positions, stiffness);
        control_command!(pc, self, "RHand", positions, stiffness);
    }

    pub fn get_touch(&self, physics_client: Arc<Mutex<PhysicsClient>>) -> Result<Touch<f32>> {
        let mut pc = physics_client.lock().unwrap();
        Ok(Touch {
            chest_board: pc.read_user_debug_parameter(self.touch_input.chest_board)? as f32,
            head_front: pc.read_user_debug_parameter(self.touch_input.head_front)? as f32,
            head_middle: pc.read_user_debug_parameter(self.touch_input.head_middle)? as f32,
            head_rear: pc.read_user_debug_parameter(self.touch_input.head_rear)? as f32,
            left_foot_left: pc.read_user_debug_parameter(self.touch_input.left_foot_left)? as f32,
            left_foot_right: pc.read_user_debug_parameter(self.touch_input.left_foot_right)? as f32,
            left_hand_back: pc.read_user_debug_parameter(self.touch_input.left_hand_back)? as f32,
            left_hand_left: pc.read_user_debug_parameter(self.touch_input.left_hand_left)? as f32,
            left_hand_right: pc.read_user_debug_parameter(self.touch_input.left_hand_right)? as f32,
            right_foot_left: pc.read_user_debug_parameter(self.touch_input.right_foot_left)? as f32,
            right_foot_right: pc.read_user_debug_parameter(self.touch_input.right_foot_right)? as f32,
            right_hand_back: pc.read_user_debug_parameter(self.touch_input.right_hand_back)? as f32,
            right_hand_left: pc.read_user_debug_parameter(self.touch_input.right_hand_left)? as f32,
            right_hand_right: pc.read_user_debug_parameter(self.touch_input.right_hand_right)? as f32,
        })
    }
}
#[derive(Debug)]
pub struct BulletJoint(JointInfo);

fn build_link_id_map(
    physics_client: Arc<Mutex<PhysicsClient>>,
    body_id: BodyId,
    link_map: &mut HashMap<String, JointInfo>,
    joint_map: &mut HashMap<String, BulletJoint>,
) {
    let mut pc = physics_client.lock().unwrap();
    for i in 0..pc.get_num_joints(body_id) {
        let joint_info = pc.get_joint_info(body_id, i);
        match joint_info.joint_type {
            rubullet::JointType::Revolute | rubullet::JointType::Prismatic => {
                joint_map.insert(
                    joint_info.joint_name.clone(),
                    BulletJoint(pc.get_joint_info(body_id, i)),
                );
                println!("{}", joint_info.joint_name);
            }
            _ => { // not a joint in nao
            }
        }
        link_map.insert(
            pc.get_joint_info(body_id, i).link_name,
            pc.get_joint_info(body_id, i),
        );
    }
}
