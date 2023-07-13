use crate::Result;
use rubullet::nalgebra::{Isometry3, Vector3};
use rubullet::{BodyId, PhysicsClient};

use super::nao::BulletNao;

#[derive(Debug)]
pub struct NaoBulletEnvironment {
    pub plane_id: BodyId,
    pub nao: BulletNao,
}

impl NaoBulletEnvironment {
    pub fn create(physics_client: &mut PhysicsClient) -> Result<Self> {
        physics_client.set_additional_search_path("../../bullet_data")?;
        physics_client.set_gravity(Vector3::new(0.0, 0.0, -10.0));

        let plane_id = physics_client.load_urdf("plane.urdf", None)?;

        let start_position = Isometry3::translation(0.0, 0.0, 1.0);
        let nao = BulletNao::create(physics_client, start_position)?;

        Ok(NaoBulletEnvironment { plane_id, nao })
    }
}
