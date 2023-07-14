use miette::bail;
use rubullet::nalgebra::{Isometry3, Quaternion, Vector3};
use rubullet::{BodyId, JointInfo, LoadModelFlags, PhysicsClient, UrdfOptions};

use crate::types::JointArray;
use crate::Result;

#[derive(Debug)]
pub struct BulletNao {
    pub id: BodyId,
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

        for i in 0..physics_client.get_num_joints(id) {
            let joint_info = physics_client.get_joint_info(id, i);
            println!("{}", joint_info.link_name);
        }

        physics_client.create_constraint(
            id,
            None,
            None,
            None,
            rubullet::JointType::Fixed,
            [0.0; 3],
            Isometry3::identity(),
            start_position,
        )?;

        Ok(BulletNao { id: id })
    }
}

fn build_joint_link_array(
    physics_client: &mut PhysicsClient,
    body_id: BodyId,
) -> Result<JointArray<WrappedJointInfo>> {
    let mut builder = JointArray::<WrappedJointInfo>::builder();
    for i in 0..physics_client.get_num_joints(body_id) {
        let joint_info = physics_client.get_joint_info(body_id, i);
        match joint_info.joint_name.as_str() {
            "HeadPitch" => {
                builder = builder.head_pitch(WrappedJointInfo(joint_info));
            },
            "HeadYaw" => {
                builder = builder.head_yaw(WrappedJointInfo(joint_info));
            },
            _ => {}
        }
    }

    Ok(builder.build())
}

struct WrappedJointInfo(JointInfo);

impl Default for WrappedJointInfo {
    fn default() -> Self {
        Self(JointInfo {
            joint_index: Default::default(),
            joint_name: Default::default(),
            joint_type: rubullet::JointType::Fixed,
            q_index: Default::default(),
            u_index: Default::default(),
            flags: Default::default(),
            joint_damping: Default::default(),
            joint_friction: Default::default(),
            joint_lower_limit: Default::default(),
            joint_upper_limit: Default::default(),
            joint_max_force: Default::default(),
            joint_max_velocity: Default::default(),
            link_name: Default::default(),
            joint_axis: Default::default(),
            parent_frame_pose: Isometry3::identity(),
            parent_index: Default::default(),
        })
    }
}
