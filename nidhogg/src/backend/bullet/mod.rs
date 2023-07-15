#![allow(missing_debug_implementations)]
use crate::{NaoBackend, NaoControlMessage, NaoState, Result, types::{Vector3, Vector2}};
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
    fn connect() -> Result<Self> {
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

    fn send_control_msg(&mut self, update: NaoControlMessage) -> Result<()> {
        self.nao
            .set_angles(&mut self.physics_client, update.position, update.stiffness);
        self.physics_client.step_simulation()?;
        Ok(())
    }

    fn read_nao_state(&mut self) -> Result<NaoState> {
        Ok(NaoState {
            position: Default::default(),
            stiffness: Default::default(),
            accelerometer: Vector3 { x: 0.0, y: 0.0, z: 0.0},
            gyroscope: Vector3 { x: 0.0, y: 0.0, z: 0.0},
            angles: Vector2 { x: 0.0, y: 0.0},
            sonar: Default::default(),
            force_sensitive_resistors: Default::default(),
            touch: self.nao.get_touch(&mut self.physics_client)?,
            battery: Default::default(),
            temperature: Default::default(),
            current: Default::default(),
            status: Default::default(),
        })
    }
}
