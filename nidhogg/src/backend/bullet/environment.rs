use std::sync::{Mutex, Arc};

use crate::Result;
use rubullet::nalgebra::Vector3;
use rubullet::{BodyId, PhysicsClient};

#[derive(Debug)]
pub struct NaoBulletEnvironment {
    pub plane_id: BodyId,
}

impl NaoBulletEnvironment {
    pub fn create(physics_client: Arc<Mutex<PhysicsClient>>) -> Result<Self> {
        let mut pc = physics_client.lock().unwrap();
        pc.set_additional_search_path("../../bullet_data")?;
        pc.set_gravity(Vector3::new(0.0, 0.0, -9.81));

        let plane_id = pc.load_urdf("plane.urdf", None)?;

        Ok(NaoBulletEnvironment { plane_id })
    }
}
