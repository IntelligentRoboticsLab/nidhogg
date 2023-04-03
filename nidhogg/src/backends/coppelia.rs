use crate::{types::JointArray, Error, NaoBackend, NaoControlMessage, NaoState, Result};
use std::{sync::Arc};
use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams};

#[allow(missing_debug_implementations)]
pub struct CoppeliaBackend {
    #[allow(dead_code)]
    // client: Arc<RemoteApiClient>,
    sim: Sim,
    joint_handles: JointArray<i64>, //Option?
}

impl NaoBackend for CoppeliaBackend {
    fn connect() -> Result<Self> {
        let client = Arc::new(
            RemoteApiClient::new(RemoteApiClientParams {
                host: "localhost".to_string(),
                ..RemoteApiClientParams::default()
            })
            .map_err(|e| Error::CoppeliaConnectError(e.show()))?,
        );
        let sim = Sim::new(client.clone());
        let joint_handles = get_joint_handles(&sim)?;
        client.to_owned().set_stepping(false).unwrap();
        sim.start_simulation().unwrap();
        // let joint_handles = get_joint_handles(&sim);
        Ok(CoppeliaBackend { sim, joint_handles })
    }

    fn send_control_msg(&mut self, update: NaoControlMessage) -> Result<()> {
        self.set_joint_position(self.joint_handles.head_pitch, update.position.head_pitch)?;
        self.set_joint_position(self.joint_handles.head_yaw, update.position.head_yaw)?;

        self.set_joint_position(
            self.joint_handles.left_shoulder_pitch,
            update.position.left_shoulder_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.left_shoulder_roll,
            update.position.left_shoulder_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.left_elbow_yaw,
            update.position.left_elbow_yaw,
        )?;
        self.set_joint_position(
            self.joint_handles.left_elbow_roll,
            update.position.left_elbow_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.left_wrist_yaw,
            update.position.left_wrist_yaw,
        )?;
        self.set_joint_position(
            self.joint_handles.left_hip_yaw_pitch,
            update.position.left_hip_yaw_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.left_hip_roll,
            update.position.left_hip_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.left_hip_pitch,
            update.position.left_hip_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.left_knee_pitch,
            update.position.left_knee_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.left_ankle_pitch,
            update.position.left_ankle_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.left_ankle_roll,
            update.position.left_ankle_roll,
        )?;

        self.set_joint_position(
            self.joint_handles.right_shoulder_pitch,
            update.position.right_shoulder_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.right_shoulder_roll,
            update.position.right_shoulder_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.right_elbow_yaw,
            update.position.right_elbow_yaw,
        )?;
        self.set_joint_position(
            self.joint_handles.right_elbow_roll,
            update.position.right_elbow_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.right_wrist_yaw,
            update.position.right_wrist_yaw,
        )?;
        // self.set_joint_position(self.joint_handles.right_hip_yaw_pitch, update.position.left_hip_yaw_pitch)?;
        self.set_joint_position(
            self.joint_handles.right_hip_roll,
            update.position.right_hip_roll,
        )?;
        self.set_joint_position(
            self.joint_handles.right_hip_pitch,
            update.position.right_hip_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.right_knee_pitch,
            update.position.right_knee_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.right_ankle_pitch,
            update.position.right_ankle_pitch,
        )?;
        self.set_joint_position(
            self.joint_handles.right_ankle_roll,
            update.position.right_ankle_roll,
        )?;

        self.set_joint_stiffness(self.joint_handles.head_pitch, update.stiffness.head_pitch)?;
        self.set_joint_stiffness(self.joint_handles.head_yaw, update.stiffness.head_yaw)?;

        self.set_joint_stiffness(
            self.joint_handles.left_shoulder_pitch,
            update.stiffness.left_shoulder_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_shoulder_roll,
            update.stiffness.left_shoulder_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_elbow_yaw,
            update.stiffness.left_elbow_yaw,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_elbow_roll,
            update.stiffness.left_elbow_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_wrist_yaw,
            update.stiffness.left_wrist_yaw,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_hip_yaw_pitch,
            update.stiffness.left_hip_yaw_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_hip_roll,
            update.stiffness.left_hip_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_hip_pitch,
            update.stiffness.left_hip_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_knee_pitch,
            update.stiffness.left_knee_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_ankle_pitch,
            update.stiffness.left_ankle_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.left_ankle_roll,
            update.stiffness.left_ankle_roll,
        )?;

        self.set_joint_stiffness(
            self.joint_handles.right_shoulder_pitch,
            update.stiffness.right_shoulder_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_shoulder_roll,
            update.stiffness.right_shoulder_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_elbow_yaw,
            update.stiffness.right_elbow_yaw,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_elbow_roll,
            update.stiffness.right_elbow_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_wrist_yaw,
            update.stiffness.right_wrist_yaw,
        )?;
        // self.set_joint_stiffness(self.joint_handles.right_hip_yaw_pitch, update.stiffness.left_hip_yaw_pitch)?;
        self.set_joint_stiffness(
            self.joint_handles.right_hip_roll,
            update.stiffness.right_hip_roll,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_hip_pitch,
            update.stiffness.right_hip_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_knee_pitch,
            update.stiffness.right_knee_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_ankle_pitch,
            update.stiffness.right_ankle_pitch,
        )?;
        self.set_joint_stiffness(
            self.joint_handles.right_ankle_roll,
            update.stiffness.right_ankle_roll,
        )?;
        Ok(())
    }

    fn read_nao_state(&mut self) -> Result<NaoState> {
        todo!("implement reading from coppelia")
    }
}

impl CoppeliaBackend {
    fn set_joint_position(&self, joint_handle: i64, target_position: f32) -> Result<()> {
        self.sim
            .set_joint_target_position(joint_handle, target_position as f64, None)
            .map_err(|e| Error::CoppeliaSetValueError(e.show()))
    }

    fn set_joint_stiffness(&self, joint_handle: i64, target_stiffness: f32) -> Result<()> {
        self.sim
            .set_joint_target_force(joint_handle, target_stiffness as f64, None)
            .map_err(|e| Error::CoppeliaSetValueError(e.show()))
    }
}

fn get_object(sim: &Sim, path: impl Into<String>) -> Result<i64> {
    let path: String = path.into();
    let object = sim
        .get_object(path, None)
        .map_err(|e| Error::CoppeliaGetObjectError(e.show()));

    return object;
}

fn get_joint_handles(sim: &Sim) -> Result<JointArray<i64>> {
    let joint_handles: JointArray<i64> = JointArray {
        head_yaw: get_object(&sim, "/NAO/HeadYaw")?,
        head_pitch: get_object(&sim, "/NAO/HeadPitch")?,

        left_shoulder_pitch: get_object(&sim, "/NAO/LShoulderPitch")?,
        left_shoulder_roll: get_object(&sim, "/NAO/LShoulderRoll")?,
        left_elbow_yaw: get_object(&sim, "/NAO/LElbowYaw")?,
        left_elbow_roll: get_object(&sim, "/NAO/LElbowRoll")?,
        left_wrist_yaw: get_object(&sim, "/NAO/LWristYaw")?,
        left_hip_yaw_pitch: get_object(&sim, "/NAO/LHipYawPitch")?,
        left_hip_roll: get_object(&sim, "/NAO/LHipRoll")?,
        left_hip_pitch: get_object(&sim, "/NAO/LHipPitch")?,
        left_knee_pitch: get_object(&sim, "/NAO/LKneePitch")?,
        left_ankle_pitch: get_object(&sim, "/NAO/LAnklePitch")?,
        left_ankle_roll: get_object(&sim, "/NAO/LAnkleRoll")?,

        right_shoulder_pitch: get_object(&sim, "/NAO/RShoulderPitch")?,
        right_shoulder_roll: get_object(&sim, "/NAO/RShoulderRoll")?,
        right_elbow_yaw: get_object(&sim, "/NAO/RElbowYaw")?,
        right_elbow_roll: get_object(&sim, "/NAO/RElbowRoll")?,
        right_wrist_yaw: get_object(&sim, "/NAO/RWristYaw")?,
        // right_hip_yaw_pitch: self.get_object("/NAO/RHipYawPitch")?; // heeft LoLa niet,
        right_hip_roll: get_object(&sim, "/NAO/RHipRoll")?,
        right_hip_pitch: get_object(&sim, "/NAO/RHipPitch")?,
        right_knee_pitch: get_object(&sim, "/NAO/RKneePitch")?,
        right_ankle_pitch: get_object(&sim, "/NAO/RAnklePitch")?,
        right_ankle_roll: get_object(&sim, "/NAO/RAnkleRoll")?,

        // handen heeft coppelia niet dus nog een keer wrist als placeholder
        left_hand: get_object(&sim, "/NAO/LWristYaw")?,
        right_hand: get_object(&sim, "/NAO/RWristYaw")?,
    };
    Ok(joint_handles)
}
