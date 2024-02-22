#![allow(missing_debug_implementations)]
use crate::{
    types::{Vector2, Vector3},
    backend::ReadHardwareInfo,
    HardwareInfo, NaoBackend, NaoControlMessage, NaoState, Result,
};
use environment::NaoBulletEnvironment;
use rubullet::{nalgebra::Isometry3, Mode, PhysicsClient, SetPhysicsEngineParameterOptions};

use self::nao::BulletNao;

mod environment;
#[macro_use]
mod nao;

pub struct BulletBackend {
    pub physics_client: PhysicsClient,
    pub environment: NaoBulletEnvironment,
    pub nao: BulletNao,
}

unsafe impl Send for BulletBackend {}
unsafe impl Sync for BulletBackend {}

impl NaoBackend for BulletBackend {
    fn connect() -> Result<Self> {
        let mut physics_client = PhysicsClient::connect(Mode::GuiMainThread)?;
        let environment = NaoBulletEnvironment::create(&mut physics_client)?;
        let start_position = Isometry3::translation(0.0, 0.0, 0.30);
        let nao = BulletNao::create(&mut physics_client, start_position)?;

        println!("{:?}", physics_client.get_physics_engine_parameters()?);

        physics_client.set_physics_engine_parameter(SetPhysicsEngineParameterOptions {
            num_solver_iterations: Some(1000),
            enable_cone_friction: Some(true),
            ..Default::default()
        });

        Ok(BulletBackend {
            physics_client,
            environment,
            nao,
        })
    }

    fn send_control_msg(&mut self, update: NaoControlMessage) -> Result<()> {
        self.nao
            .set_angles(&mut self.physics_client, update.position, update.stiffness);
        Ok(())
    }

    fn read_nao_state(&mut self) -> Result<NaoState> {
        let vel = self.physics_client.get_base_velocity(self.nao.id)?;
        self.physics_client.step_simulation()?;
        Ok(NaoState {
            position: Default::default(),
            stiffness: Default::default(),
            accelerometer: Vector3 {
                x: vel.get_linear_velocity().x as f32,
                y: vel.get_linear_velocity().y as f32,
                z: vel.get_linear_velocity().z as f32,
            },

            gyroscope: Vector3 {
                x: vel.get_angular_velocity().x as f32,
                y: vel.get_angular_velocity().y as f32,
                z: vel.get_angular_velocity().z as f32,
            },
            angles: Vector2 { x: 0.0, y: 0.0 },
            sonar: Default::default(),
            force_sensitive_resistors: self.nao.get_fsr(&mut self.physics_client, &self.environment.plane_id)?,
            touch: self.nao.get_touch(&mut self.physics_client)?,
            battery: Default::default(),
            temperature: Default::default(),
            current: Default::default(),
            status: Default::default(),
        })
    }
}

impl ReadHardwareInfo for BulletBackend {
    fn read_hardware_info(&mut self) -> Result<HardwareInfo> {
        let hardware_info = "bullet".to_string();
        Ok(HardwareInfo {
            body_id: hardware_info.clone(),
            body_version: hardware_info.clone(),
            head_id: hardware_info.clone(),
            head_version: hardware_info.clone(),
        })
    }
}