use rubullet::nalgebra::{Isometry3, Quaternion, Vector3};
use rubullet::{BodyId, LoadModelFlags, PhysicsClient, UrdfOptions, JointInfo};

use crate::Result;
use crate::types::JointArray;

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
            println!("{}: {:?}", joint_info.link_name, joint_info);
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

fn build_joint_link_array(physics_client: &mut PhysicsClient, body_id: BodyId) -> JointArray<JointInfo> {
    for i in 0..physics_client.get_num_joints(body_id) {
        let joint_info = physics_client.get_joint_info(body_id, i);

        
    }
}