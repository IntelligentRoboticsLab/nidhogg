use miette::Result;

use nidhogg::{types::*, NaoControlMsg};

fn main() -> Result<()> {
    let cool_float = 1337.0;
    let cool_bool = false;
    let cool_color = Color::new(0.42, 0.42, 0.42);

    let _msg = NaoControlMsg::builder()
        .position(
            JointArray::builder()
                .head_joints(
                    HeadJoints::builder()
                        .pitch(cool_float)
                        .yaw(cool_float)
                        .build(),
                )
                .left_arm_joints(
                    LeftArmJoints::builder()
                        .shoulder_pitch(cool_float)
                        .shoulder_roll(cool_float)
                        .elbow_yaw(cool_float)
                        .elbow_roll(cool_float)
                        .wrist_yaw(cool_float)
                        .hand(cool_float)
                        .build(),
                )
                .right_arm_joints(
                    RightArmJoints::builder()
                        .shoulder_pitch(cool_float)
                        .shoulder_roll(cool_float)
                        .elbow_yaw(cool_float)
                        .elbow_roll(cool_float)
                        .wrist_yaw(cool_float)
                        .hand(cool_float)
                        .build(),
                )
                .left_leg_joints(
                    LeftLegJoints::builder()
                        .hip_yaw_pitch(cool_float)
                        .hip_roll(cool_float)
                        .hip_pitch(cool_float)
                        .knee_pitch(cool_float)
                        .ankle_pitch(cool_float)
                        .ankle_roll(cool_float)
                        .build(),
                )
                .right_leg_joints(
                    RightLegJoints::builder()
                        .hip_roll(cool_float)
                        .hip_pitch(cool_float)
                        .knee_pitch(cool_float)
                        .ankle_pitch(cool_float)
                        .ankle_roll(cool_float)
                        .build(),
                )
                .build(),
        )
        .stiffness(
            JointArray::builder()
                .head_yaw(cool_float)
                .head_pitch(cool_float)
                .left_shoulder_pitch(cool_float)
                .left_shoulder_roll(cool_float)
                .left_elbow_yaw(cool_float)
                .left_elbow_roll(cool_float)
                .left_wrist_yaw(cool_float)
                .left_hip_yaw_pitch(cool_float)
                .left_hip_roll(cool_float)
                .left_hip_pitch(cool_float)
                .left_knee_pitch(cool_float)
                .left_ankle_pitch(cool_float)
                .left_ankle_roll(cool_float)
                .right_hip_roll(cool_float)
                .right_hip_pitch(cool_float)
                .right_knee_pitch(cool_float)
                .right_ankle_pitch(cool_float)
                .right_ankle_roll(cool_float)
                .right_shoulder_pitch(cool_float)
                .right_shoulder_roll(cool_float)
                .right_elbow_yaw(cool_float)
                .right_elbow_roll(cool_float)
                .right_wrist_yaw(cool_float)
                .left_hand(cool_float)
                .right_hand(cool_float)
                .build(),
        )
        .sonar(
            SonarEnabled::builder()
                .left(cool_bool)
                .right(cool_bool)
                .build(),
        )
        .left_ear(
            LeftEar::builder()
                .intensity_0_deg(cool_float)
                .intensity_36_deg(cool_float)
                .intensity_72_deg(cool_float)
                .intensity_108_deg(cool_float)
                .intensity_144_deg(cool_float)
                .intensity_180_deg(cool_float)
                .intensity_216_deg(cool_float)
                .intensity_252_deg(cool_float)
                .intensity_288_deg(cool_float)
                .intensity_324_deg(cool_float)
                .build(),
        )
        .right_ear(
            RightEar::builder()
                .intensity_0_deg(cool_float)
                .intensity_36_deg(cool_float)
                .intensity_72_deg(cool_float)
                .intensity_108_deg(cool_float)
                .intensity_144_deg(cool_float)
                .intensity_180_deg(cool_float)
                .intensity_216_deg(cool_float)
                .intensity_252_deg(cool_float)
                .intensity_288_deg(cool_float)
                .intensity_324_deg(cool_float)
                .build(),
        )
        .chest(cool_color)
        .left_eye(
            LeftEye::builder()
                .color_0_deg(cool_color)
                .color_45_deg(cool_color)
                .color_90_deg(cool_color)
                .color_135_deg(cool_color)
                .color_180_deg(cool_color)
                .color_225_deg(cool_color)
                .color_270_deg(cool_color)
                .color_315_deg(cool_color)
                .build(),
        )
        .right_eye(
            RightEye::builder()
                .color_0_deg(cool_color)
                .color_45_deg(cool_color)
                .color_90_deg(cool_color)
                .color_135_deg(cool_color)
                .color_180_deg(cool_color)
                .color_225_deg(cool_color)
                .color_270_deg(cool_color)
                .color_315_deg(cool_color)
                .build(),
        )
        .left_foot(cool_color)
        .right_foot(cool_color)
        .skull(
            Skull::builder()
                .left_front_0(cool_float)
                .left_front_1(cool_float)
                .left_middle_0(cool_float)
                .left_rear_0(cool_float)
                .left_rear_1(cool_float)
                .left_rear_2(cool_float)
                .right_front_0(cool_float)
                .right_front_1(cool_float)
                .right_middle_0(cool_float)
                .right_rear_0(cool_float)
                .right_rear_1(cool_float)
                .right_rear_2(cool_float)
                .build(),
        )
        .build();

    Ok(())
}
