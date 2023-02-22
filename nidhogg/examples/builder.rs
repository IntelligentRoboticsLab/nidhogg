use color_eyre::Result;
use nidhogg::{
    types::{Color, JointArray, LeftEye, LeftLegJoints},
    Update,
};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let crazy_update = Update::builder()
        .stiffness(
            JointArray::builder()
                .head_pitch(0.25)
                .left_leg_joints(
                    LeftLegJoints::builder()
                        .hip_pitch(0.55)
                        .hip_roll(0.56)
                        .build(),
                )
                .left_hand(35.2)
                .build(),
        )
        .left_eye(
            LeftEye::builder()
                .color_180_deg(Color::new(0.24, 1.0, 0.45))
                .build(),
        )
        .build();

    assert_eq!(crazy_update.stiffness.head_pitch, 0.25);
    assert_eq!(crazy_update.stiffness.left_hip_pitch, 0.55);
    assert_eq!(crazy_update.stiffness.left_hip_roll, 0.56);
    assert_eq!(crazy_update.stiffness.left_hand, 35.2);
    assert_eq!(crazy_update.left_eye.color_180_deg.red, 0.24);
    assert_eq!(crazy_update.left_eye.color_180_deg.green, 1.0);
    assert_eq!(crazy_update.left_eye.color_180_deg.blue, 0.45);

    Ok(())
}
