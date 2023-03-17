use crate::{Error, NaoBackend, NaoControlMessage, NaoState, Result, types::JointArray};
use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams};
use std::rc::Rc;

#[allow(missing_debug_implementations)]
pub struct CoppeliaBackend {
    #[allow(dead_code)]
    client: Rc<RemoteApiClient>,
    sim: Sim,
    joint_handles: JointArray<i64>,
}

impl NaoBackend for CoppeliaBackend {
    fn connect() -> Result<Self> {
        let client = Rc::new(
            RemoteApiClient::new(RemoteApiClientParams {
                host: "localhost".to_string(),
                ..RemoteApiClientParams::default()
            }).map_err(|e| Error::CoppeliaConnectError(e.show()))?
        );

        let sim = Sim::new(client.clone());
        // Get the joints
        // TODO: make function that wraps sim.get_object() and handles the error properly
        //  .map_err(|e| Error::CoppeliaConnectError(e.show()))?);
        let joint_handles: JointArray<i64> = JointArray {
            head_yaw: sim.get_object("/NAO/HeadYaw".to_string(), None).unwrap(),
            head_pitch: sim.get_object("/NAO/HeadPitch".to_string(), None).unwrap(),

            left_shoulder_pitch: sim.get_object("/NAO/LShoulderPitch".to_string(), None).unwrap(),
            left_shoulder_roll: sim.get_object("/NAO/LShoulderRoll".to_string(), None).unwrap(),
            left_elbow_yaw: sim.get_object("/NAO/LElbowYaw".to_string(), None).unwrap(),
            left_elbow_roll: sim.get_object("/NAO/LElbowRoll".to_string(), None).unwrap(),
            left_wrist_yaw: sim.get_object("/NAO/LWristYaw".to_string(), None).unwrap(),
            left_hip_yaw_pitch: sim.get_object("/NAO/LHipYawPitch".to_string(), None).unwrap(),
            left_hip_roll: sim.get_object("/NAO/LHipRoll".to_string(), None).unwrap(),
            left_hip_pitch: sim.get_object("/NAO/LHipPitch".to_string(), None).unwrap(),
            left_knee_pitch: sim.get_object("/NAO/LKneePitch".to_string(), None).unwrap(),
            left_ankle_pitch: sim.get_object("/NAO/LAnklePitch".to_string(), None).unwrap(),
            left_ankle_roll: sim.get_object("/NAO/LAnkleRoll".to_string(), None).unwrap(),

            right_shoulder_pitch: sim.get_object("/NAO/RShoulderPitch".to_string(), None).unwrap(),
            right_shoulder_roll: sim.get_object("/NAO/RShoulderRoll".to_string(), None).unwrap(),
            right_elbow_yaw: sim.get_object("/NAO/RElbowYaw".to_string(), None).unwrap(),
            right_elbow_roll: sim.get_object("/NAO/RElbowRoll".to_string(), None).unwrap(),
            right_wrist_yaw: sim.get_object("/NAO/RWristYaw".to_string(), None).unwrap(),
            // right_hip_yaw_pitch: sim.get_object("/NAO/RHipYawPitch".to_string(), None)?; // heeft LoLa niet,
            right_hip_roll: sim.get_object("/NAO/RHipRoll".to_string(), None).unwrap(),
            right_hip_pitch: sim.get_object("/NAO/RHipPitch".to_string(), None).unwrap(),
            right_knee_pitch: sim.get_object("/NAO/RKneePitch".to_string(), None).unwrap(),
            right_ankle_pitch: sim.get_object("/NAO/RAnklePitch".to_string(), None).unwrap(),
            right_ankle_roll: sim.get_object("/NAO/RAnkleRoll".to_string(), None).unwrap(),

            // handen heeft coppelia niet dus nog een keer wrist als placeholder
            left_hand: sim.get_object("/NAO/LWristYaw".to_string(), None).unwrap(),
            right_hand: sim.get_object("/NAO/RWristYaw".to_string(), None).unwrap(),
        };

        client.to_owned().set_stepping(true).unwrap();
        sim.start_simulation().unwrap();
        Ok(CoppeliaBackend { client, sim, joint_handles })
    }

    fn send_control_msg(
        &mut self,
        update: NaoControlMessage,
    ) -> Result<()> {
        self.set_joint_position(self.joint_handles.head_pitch, update.position.head_pitch)?;
        self.set_joint_position(self.joint_handles.head_yaw, update.position.head_yaw)?;

        self.set_joint_position(self.joint_handles.left_shoulder_pitch, update.position.left_shoulder_pitch)?;
        self.set_joint_position(self.joint_handles.left_shoulder_roll, update.position.left_shoulder_roll)?;
        self.set_joint_position(self.joint_handles.left_elbow_yaw, update.position.left_elbow_yaw)?;
        self.set_joint_position(self.joint_handles.left_elbow_roll, update.position.left_elbow_roll)?;
        self.set_joint_position(self.joint_handles.left_wrist_yaw, update.position.left_wrist_yaw)?;
        self.set_joint_position(self.joint_handles.left_hip_yaw_pitch, update.position.left_hip_yaw_pitch)?;
        self.set_joint_position(self.joint_handles.left_hip_roll, update.position.left_hip_roll)?;
        self.set_joint_position(self.joint_handles.left_hip_pitch, update.position.left_hip_pitch)?;
        self.set_joint_position(self.joint_handles.left_knee_pitch, update.position.left_knee_pitch)?;
        self.set_joint_position(self.joint_handles.left_ankle_pitch, update.position.left_ankle_pitch)?;
        self.set_joint_position(self.joint_handles.left_ankle_roll, update.position.left_ankle_roll)?;

        self.set_joint_position(self.joint_handles.right_shoulder_pitch, update.position.right_shoulder_pitch)?;
        self.set_joint_position(self.joint_handles.right_shoulder_roll, update.position.right_shoulder_roll)?;
        self.set_joint_position(self.joint_handles.right_elbow_yaw, update.position.right_elbow_yaw)?;
        self.set_joint_position(self.joint_handles.right_elbow_roll, update.position.right_elbow_roll)?;
        self.set_joint_position(self.joint_handles.right_wrist_yaw, update.position.right_wrist_yaw)?;
        // self.set_joint_position(self.joint_handles.right_hip_yaw_pitch, update.position.left_hip_yaw_pitch)?;
        self.set_joint_position(self.joint_handles.right_hip_roll, update.position.right_hip_roll)?;
        self.set_joint_position(self.joint_handles.right_hip_pitch, update.position.right_hip_pitch)?;
        self.set_joint_position(self.joint_handles.right_knee_pitch, update.position.right_knee_pitch)?;
        self.set_joint_position(self.joint_handles.right_ankle_pitch, update.position.right_ankle_pitch)?;
        self.set_joint_position(self.joint_handles.right_ankle_roll, update.position.right_ankle_roll)?;

        self.set_joint_stiffness(self.joint_handles.head_pitch, update.stiffness.head_pitch)?;
        self.set_joint_stiffness(self.joint_handles.head_yaw, update.stiffness.head_yaw)?;

        self.set_joint_stiffness(self.joint_handles.left_shoulder_pitch, update.stiffness.left_shoulder_pitch)?;
        self.set_joint_stiffness(self.joint_handles.left_shoulder_roll, update.stiffness.left_shoulder_roll)?;
        self.set_joint_stiffness(self.joint_handles.left_elbow_yaw, update.stiffness.left_elbow_yaw)?;
        self.set_joint_stiffness(self.joint_handles.left_elbow_roll, update.stiffness.left_elbow_roll)?;
        self.set_joint_stiffness(self.joint_handles.left_wrist_yaw, update.stiffness.left_wrist_yaw)?;
        self.set_joint_stiffness(self.joint_handles.left_hip_yaw_pitch, update.stiffness.left_hip_yaw_pitch)?;
        self.set_joint_stiffness(self.joint_handles.left_hip_roll, update.stiffness.left_hip_roll)?;
        self.set_joint_stiffness(self.joint_handles.left_hip_pitch, update.stiffness.left_hip_pitch)?;
        self.set_joint_stiffness(self.joint_handles.left_knee_pitch, update.stiffness.left_knee_pitch)?;
        self.set_joint_stiffness(self.joint_handles.left_ankle_pitch, update.stiffness.left_ankle_pitch)?;
        self.set_joint_stiffness(self.joint_handles.left_ankle_roll, update.stiffness.left_ankle_roll)?;

        self.set_joint_stiffness(self.joint_handles.right_shoulder_pitch, update.stiffness.right_shoulder_pitch)?;
        self.set_joint_stiffness(self.joint_handles.right_shoulder_roll, update.stiffness.right_shoulder_roll)?;
        self.set_joint_stiffness(self.joint_handles.right_elbow_yaw, update.stiffness.right_elbow_yaw)?;
        self.set_joint_stiffness(self.joint_handles.right_elbow_roll, update.stiffness.right_elbow_roll)?;
        self.set_joint_stiffness(self.joint_handles.right_wrist_yaw, update.stiffness.right_wrist_yaw)?;
        // self.set_joint_stiffness(self.joint_handles.right_hip_yaw_pitch, update.stiffness.left_hip_yaw_pitch)?;
        self.set_joint_stiffness(self.joint_handles.right_hip_roll, update.stiffness.right_hip_roll)?;
        self.set_joint_stiffness(self.joint_handles.right_hip_pitch, update.stiffness.right_hip_pitch)?;
        self.set_joint_stiffness(self.joint_handles.right_knee_pitch, update.stiffness.right_knee_pitch)?;
        self.set_joint_stiffness(self.joint_handles.right_ankle_pitch, update.stiffness.right_ankle_pitch)?;
        self.set_joint_stiffness(self.joint_handles.right_ankle_roll, update.stiffness.right_ankle_roll)?;

        self.client.to_owned().step(true).unwrap();
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
            .map_err(| e | Error::CoppeliaSetValueError(e.show()))
    }

    // TODO: make a function for getting the object
    fn get_object(&self, path: impl Into<String>) -> Result<()> {
        let path: String = path.into();
        // TODO: also needs the error mapping (you can use Error::CoppeliaGetObjectError)


        Ok(())
    }
}