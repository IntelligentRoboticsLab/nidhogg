use crate::Result;
use rubullet::nalgebra::Vector3;
use rubullet::{BodyId, PhysicsClient};


#[derive(Debug)]
pub struct NaoBulletEnvironment {
    pub plane_id: BodyId,
}

impl NaoBulletEnvironment {
    pub fn create(physics_client: &mut PhysicsClient) -> Result<Self> {
        physics_client.set_additional_search_path("../../bullet_data")?;
        physics_client.set_gravity(Vector3::new(0.0, 0.0, -9.81));

        let plane_id = physics_client.load_urdf("plane.urdf", None)?;

        Ok(NaoBulletEnvironment { plane_id })
    }
}
