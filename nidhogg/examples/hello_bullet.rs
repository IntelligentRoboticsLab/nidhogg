//! An introduction to the usage of RuBullet.
use nidhogg::backend::BulletBackend;
use nidhogg::types::JointArray;
use nidhogg::{NaoBackend, NaoControlMessage, Result};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let mut bullet = BulletBackend::connect()?;

    let stiffness = 1.0;
    for t in 0..10000 {
        bullet.send_control_msg(
            NaoControlMessage::builder()
                .position(
                    JointArray::<f32>::builder()
                        .left_ankle_pitch(-std::f32::consts::FRAC_PI_8)
                        .right_ankle_pitch(-std::f32::consts::FRAC_PI_8)
                        .left_knee_pitch(std::f32::consts::FRAC_PI_4)
                        .right_knee_pitch(std::f32::consts::FRAC_PI_4)
                        .left_hip_pitch(-std::f32::consts::FRAC_PI_6)
                        .right_hip_pitch(-std::f32::consts::FRAC_PI_6)
                        .head_yaw(1.5333 * f32::sin(0.01 * (t as f32)))
                        .build(),
                )
                .stiffness(
                    JointArray::<f32>::builder()
                        .left_hip_pitch(stiffness)
                        .right_hip_pitch(stiffness)
                        .left_hip_roll(stiffness)
                        .right_hip_roll(stiffness)
                        .left_ankle_pitch(stiffness)
                        .right_ankle_pitch(stiffness)
                        .left_knee_pitch(stiffness)
                        .right_knee_pitch(stiffness)
                        .left_hip_yaw_pitch(stiffness)
                        .head_yaw(stiffness)
                        .build(),
                )
                .build(),
        )?;
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
