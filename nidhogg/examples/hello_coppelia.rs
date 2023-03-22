use std::time::Duration;

use nidhogg::{backends::CoppeliaBackend, types::JointArray, NaoBackend, NaoControlMessage};

use miette::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut nao = CoppeliaBackend::connect()?;

    let update = NaoControlMessage::builder()
        .stiffness(JointArray {
            head_yaw: 1.0,
            head_pitch: 1.0,
            left_shoulder_pitch: 1.0,
            left_shoulder_roll: 1.0,
            left_elbow_yaw: 1.0,
            left_elbow_roll: 1.0,
            left_wrist_yaw: 1.0,
            left_hip_yaw_pitch: 1.0,
            left_hip_roll: 1.0,
            left_hip_pitch: 1.0,
            left_knee_pitch: 1.0,
            left_ankle_pitch: 1.0,
            left_ankle_roll: 1.0,
            right_hip_roll: 1.0,
            right_hip_pitch: 1.0,
            right_knee_pitch: 1.0,
            right_ankle_pitch: 1.0,
            right_ankle_roll: 1.0,
            right_shoulder_pitch: 1.0,
            right_shoulder_roll: 1.0,
            right_elbow_yaw: 1.0,
            right_elbow_roll: 1.0,
            right_wrist_yaw: 1.0,
            left_hand: 1.0,
            right_hand: 1.0,
        })
        .position(
            JointArray::<f32>::builder()
                .head_yaw(std::f32::consts::FRAC_PI_4)
                .build(),
        )
        .build();

    loop {
        nao.send_control_msg(update.clone())?;
        std::thread::sleep(Duration::from_millis(300));
    }
}
