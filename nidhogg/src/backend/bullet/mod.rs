#![allow(missing_debug_implementations)]
use crate::NaoBackend;
use environment::NaoBulletEnvironment;
use rubullet::{nalgebra::Isometry3, Mode, PhysicsClient};

use self::nao::BulletNao;

mod environment;
#[macro_use]
mod nao;

pub struct BulletBackend {
    pub physics_client: PhysicsClient,
    pub environment: NaoBulletEnvironment,
    pub nao: BulletNao,
}

impl NaoBackend for BulletBackend {
    fn connect() -> crate::Result<Self> {
        let mut physics_client = PhysicsClient::connect(Mode::Gui)?;
        let environment = NaoBulletEnvironment::create(&mut physics_client)?;
        let start_position = Isometry3::translation(0.0, 0.0, 0.29);
        let nao = BulletNao::create(&mut physics_client, start_position)?;

        Ok(BulletBackend {
            physics_client,
            environment,
            nao,
        })
    }

    fn send_control_msg(&mut self, update: crate::NaoControlMessage) -> crate::Result<()> {
        self.nao
            .set_angles(&mut self.physics_client, update.position, update.stiffness);
        self.physics_client.step_simulation()?;
        Ok(())
    }

    fn read_nao_state(&mut self) -> crate::Result<crate::NaoState> {
        todo!()
    }
}
