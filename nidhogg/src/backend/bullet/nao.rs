use std::collections::HashMap;

use miette::bail;
use rubullet::nalgebra::{Isometry3, Quaternion, Vector3};
use rubullet::{BodyId, JointInfo, LoadModelFlags, PhysicsClient, UrdfOptions};

use crate::types::JointArray;
use crate::Result;

#[derive(Debug)]
pub struct BulletNao {
    pub id: BodyId,
    pub link_map: HashMap<String, JointInfo>,
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

        let mut link_ids = HashMap::new();
        build_joint_link_array(physics_client, id, &mut link_ids);

        // TODO: needs more idiomatic rust!
        physics_client.set_collision_filter_pair(
            id,
            id,
            link_ids.get("torso").unwrap().joint_index,
            link_ids.get("Head").unwrap().joint_index,
            false,
        );

        do_for_both_sides(physics_client, id, "Thigh".to_string(), "Hip".to_string(), &link_ids);
        do_for_both_sides(physics_client, id, "Bicep".to_string(), "ForeArm".to_string(), &link_ids);
        do_for_both_sides(physics_client, id, "Pelvis".to_string(), "Thigh".to_string(), &link_ids);
        do_for_both_sides(physics_client, id, "Pelvis".to_string(), "Thigh".to_string(), &link_ids);
        
        Ok(BulletNao {
            id: id,
            link_map: link_ids,
        })
    }
}

fn do_for_both_sides(
    physics_client: &mut PhysicsClient,
    id: BodyId,
    link1: String,
    link2: String,
    link_ids: &HashMap<String, JointInfo>,
) {
    physics_client.set_collision_filter_pair(
        id,
        id,
        link_ids.get(&format!("R{}", link1)).unwrap().joint_index,
        link_ids.get(&format!("R{}", link2)).unwrap().joint_index,
        false,
    );
    physics_client.set_collision_filter_pair(
        id,
        id,
        link_ids.get(&format!("L{}", link1)).unwrap().joint_index,
        link_ids.get(&format!("L{}", link2)).unwrap().joint_index,
        false,
    );
}

fn build_joint_link_array(
    physics_client: &mut PhysicsClient,
    body_id: BodyId,
    map: &mut HashMap<String, JointInfo>,
) {
    for i in 0..physics_client.get_num_joints(body_id) {
        map.insert(
            String::from(physics_client.get_joint_info(body_id, i).link_name),
            physics_client.get_joint_info(body_id, i),
        );
    }
}
