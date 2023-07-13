#![allow(missing_debug_implementations)]
use crate::NaoBackend;
use environment::NaoBulletEnvironment;
use rubullet::{Mode, PhysicsClient};

mod environment;
mod nao;

pub struct BulletBackend {
    pub physics_client: PhysicsClient,
    pub environment: NaoBulletEnvironment,
}

impl NaoBackend for BulletBackend {
    fn connect() -> crate::Result<Self> {
        let mut physics_client = PhysicsClient::connect(Mode::Gui)?;
        let environment = NaoBulletEnvironment::create(&mut physics_client)?;

        Ok(BulletBackend {
            physics_client: physics_client,
            environment: environment,
        })
    }

    fn send_control_msg(&mut self, update: crate::NaoControlMessage) -> crate::Result<()> {
        todo!()
    }

    fn read_nao_state(&mut self) -> crate::Result<crate::NaoState> {
        todo!()
    }
}
