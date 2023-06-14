use crate::{types::JointArray, Error, NaoBackend, NaoControlMessage, NaoState, Result};
use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams};
use crate::{types::{JointArray, JointArrayBuilder, Vector2, Vector3}, Error, NaoBackend, NaoControlMessage, NaoState, Result};
use std::{rc::Rc, sync::Arc};
use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams, RemoteAPIError};

#[allow(missing_debug_implementations)]
pub struct CoppeliaBackend {
    sim: Arc<Sim>,
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
        let sim =  Arc::new(Sim::new(client.clone()));
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
        let state: NaoState = NaoState {
            position: get_joint_positions(&self.sim)?,
            stiffness: get_joint_stiffnesses(&self.sim)?,
            accelerometer: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            gyroscope: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            angles: Vector2 { x: 0.0, y: 0.0 },
            sonar: Default::default(),
            force_sensitive_resistors: Default::default(),
            touch: Default::default(),
            battery: Default::default(),
            temperature: Default::default(),
            current: Default::default(),
            status: Default::default()
        };
        Ok(state)
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

// TODO: zou beter kunnen door joint handles te gebruiken want die hebben we al
// wanneer we deze functie nodig hebben
fn get_position(sim: &Sim, path: impl Into<String>) -> Result<f64> {
    let path: String = path.into();
    let object = get_object(sim, path)?;
    let position =  sim
        .get_joint_position(object)
        .map_err(|e| Error::CoppeliaGetPositionError(e.show()));

    return position;
}

// Deze zou ook met joint handles kunnen ipv path denk ik
fn get_stiffness(sim: &Sim, path: impl Into<String>) -> Result<f64> {
    let path: String = path.into();
    let object = get_object(sim, path)?;
    let position =  sim
        .get_joint_force(object)
        .map_err(|e| Error::CoppeliaGetPositionError(e.show()));

    return position;
}

// Deze zou ook met joint handles kunnen ipv path denk ik
// fn get_stiffness(sim: &Sim, path: impl Into<String>) -> Result<f64> {
//     let path: String = path.into();
//     let object = get_object(sim, path)?;
//     let position =  sim
//         .get_velocity(object)
//         .map_err(|e| Error::CoppeliaGetPositionError(e.show()));

//     return position;
// }

fn get_joint_positions(sim: &Sim)  -> Result<JointArray<f32>>{
    let joint_positions: JointArray<f32> = JointArray {
        head_yaw: get_position(&sim, "/NAO/HeadYaw")? as f32,
        head_pitch: get_position(&sim, "/NAO/HeadPitch")? as f32,

        left_shoulder_pitch: get_position(&sim, "/NAO/LShoulderPitch")? as f32,
        left_shoulder_roll: get_position(&sim, "/NAO/LShoulderRoll")? as f32,
        left_elbow_yaw: get_position(&sim, "/NAO/LElbowYaw")? as f32,
        left_elbow_roll: get_position(&sim, "/NAO/LElbowRoll")? as f32,
        left_wrist_yaw: get_position(&sim, "/NAO/LWristYaw")? as f32,
        left_hip_yaw_pitch: get_position(&sim, "/NAO/LHipYawPitch")? as f32,
        left_hip_roll: get_position(&sim, "/NAO/LHipRoll")? as f32,
        left_hip_pitch: get_position(&sim, "/NAO/LHipPitch")? as f32,
        left_knee_pitch: get_position(&sim, "/NAO/LKneePitch")? as f32,
        left_ankle_pitch: get_position(&sim, "/NAO/LAnklePitch")? as f32,
        left_ankle_roll: get_position(&sim, "/NAO/LAnkleRoll")? as f32,

        right_shoulder_pitch: get_position(&sim, "/NAO/RShoulderPitch")? as f32,
        right_shoulder_roll: get_position(&sim, "/NAO/RShoulderRoll")? as f32,
        right_elbow_yaw: get_position(&sim, "/NAO/RElbowYaw")? as f32,
        right_elbow_roll: get_position(&sim, "/NAO/RElbowRoll")? as f32,
        right_wrist_yaw: get_position(&sim, "/NAO/RWristYaw")? as f32,
        // right_hip_yaw_pitch: self.get_object("/NAO/RHipYawPitch")?; // heeft LoLa niet,
        right_hip_roll: get_position(&sim, "/NAO/RHipRoll")? as f32,
        right_hip_pitch: get_position(&sim, "/NAO/RHipPitch")? as f32,
        right_knee_pitch: get_position(&sim, "/NAO/RKneePitch")? as f32,
        right_ankle_pitch: get_position(&sim, "/NAO/RAnklePitch")? as f32,
        right_ankle_roll: get_position(&sim, "/NAO/RAnkleRoll")? as f32,

        // handen heeft coppelia niet dus nog een keer wrist als placeholder
        left_hand: get_position(&sim, "/NAO/LWristYaw")? as f32,
        right_hand: get_position(&sim, "/NAO/RWristYaw")? as f32,
    };
    Ok(joint_positions)
}

fn get_joint_stiffnesses(sim: &Sim)  -> Result<JointArray<f32>>{
    let joint_positions: JointArray<f32> = JointArray {
        head_yaw: get_stiffness(&sim, "/NAO/HeadYaw")? as f32,
        head_pitch: get_stiffness(&sim, "/NAO/HeadPitch")? as f32,

        left_shoulder_pitch: get_stiffness(&sim, "/NAO/LShoulderPitch")? as f32,
        left_shoulder_roll: get_stiffness(&sim, "/NAO/LShoulderRoll")? as f32,
        left_elbow_yaw: get_stiffness(&sim, "/NAO/LElbowYaw")? as f32,
        left_elbow_roll: get_stiffness(&sim, "/NAO/LElbowRoll")? as f32,
        left_wrist_yaw: get_stiffness(&sim, "/NAO/LWristYaw")? as f32,
        left_hip_yaw_pitch: get_stiffness(&sim, "/NAO/LHipYawPitch")? as f32,
        left_hip_roll: get_stiffness(&sim, "/NAO/LHipRoll")? as f32,
        left_hip_pitch: get_stiffness(&sim, "/NAO/LHipPitch")? as f32,
        left_knee_pitch: get_stiffness(&sim, "/NAO/LKneePitch")? as f32,
        left_ankle_pitch: get_stiffness(&sim, "/NAO/LAnklePitch")? as f32,
        left_ankle_roll: get_stiffness(&sim, "/NAO/LAnkleRoll")? as f32,

        right_shoulder_pitch: get_stiffness(&sim, "/NAO/RShoulderPitch")? as f32,
        right_shoulder_roll: get_stiffness(&sim, "/NAO/RShoulderRoll")? as f32,
        right_elbow_yaw: get_stiffness(&sim, "/NAO/RElbowYaw")? as f32,
        right_elbow_roll: get_stiffness(&sim, "/NAO/RElbowRoll")? as f32,
        right_wrist_yaw: get_stiffness(&sim, "/NAO/RWristYaw")? as f32,
        // right_hip_yaw_pitch: self.get_object("/NAO/RHipYawPitch")?; // heeft LoLa niet,
        right_hip_roll: get_stiffness(&sim, "/NAO/RHipRoll")? as f32,
        right_hip_pitch: get_stiffness(&sim, "/NAO/RHipPitch")? as f32,
        right_knee_pitch: get_stiffness(&sim, "/NAO/RKneePitch")? as f32,
        right_ankle_pitch: get_stiffness(&sim, "/NAO/RAnklePitch")? as f32,
        right_ankle_roll: get_stiffness(&sim, "/NAO/RAnkleRoll")? as f32,

        // handen heeft coppelia niet dus nog een keer wrist als placeholder
        left_hand: get_stiffness(&sim, "/NAO/LWristYaw")? as f32,
        right_hand: get_stiffness(&sim, "/NAO/RWristYaw")? as f32,
    };
    Ok(joint_positions)
}
