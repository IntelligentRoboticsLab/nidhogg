use crate::types::*;
use serde::{Deserialize, Serialize};

impl From<Ear<Left>> for [f32; 10] {
    fn from(value: Ear<Left>) -> Self {
        [
            value.intensity_0_deg,
            value.intensity_36_deg,
            value.intensity_72_deg,
            value.intensity_108_deg,
            value.intensity_144_deg,
            value.intensity_180_deg,
            value.intensity_216_deg,
            value.intensity_252_deg,
            value.intensity_288_deg,
            value.intensity_324_deg,
        ]
    }
}

impl From<Ear<Right>> for [f32; 10] {
    fn from(value: Ear<Right>) -> Self {
        [
            value.intensity_324_deg,
            value.intensity_288_deg,
            value.intensity_252_deg,
            value.intensity_216_deg,
            value.intensity_180_deg,
            value.intensity_144_deg,
            value.intensity_108_deg,
            value.intensity_72_deg,
            value.intensity_36_deg,
            value.intensity_0_deg,
        ]
    }
}

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        [value.red, value.green, value.blue]
    }
}

impl From<Eye<Left>> for [f32; 24] {
    fn from(value: Eye<Left>) -> Self {
        [
            value.color_45_deg.red,
            value.color_0_deg.red,
            value.color_315_deg.red,
            value.color_270_deg.red,
            value.color_225_deg.red,
            value.color_180_deg.red,
            value.color_135_deg.red,
            value.color_90_deg.red,
            // bad rustfmt
            value.color_45_deg.green,
            value.color_0_deg.green,
            value.color_315_deg.green,
            value.color_270_deg.green,
            value.color_225_deg.green,
            value.color_180_deg.green,
            value.color_135_deg.green,
            value.color_90_deg.green,
            // bad rustfmt
            value.color_45_deg.blue,
            value.color_0_deg.blue,
            value.color_315_deg.blue,
            value.color_270_deg.blue,
            value.color_225_deg.blue,
            value.color_180_deg.blue,
            value.color_135_deg.blue,
            value.color_90_deg.blue,
        ]
    }
}

impl From<Eye<Right>> for [f32; 24] {
    fn from(value: Eye<Right>) -> Self {
        [
            value.color_0_deg.red,
            value.color_45_deg.red,
            value.color_90_deg.red,
            value.color_135_deg.red,
            value.color_180_deg.red,
            value.color_225_deg.red,
            value.color_270_deg.red,
            value.color_315_deg.red,
            // bad rustfmt
            value.color_0_deg.green,
            value.color_45_deg.green,
            value.color_90_deg.green,
            value.color_135_deg.green,
            value.color_180_deg.green,
            value.color_225_deg.green,
            value.color_270_deg.green,
            value.color_315_deg.green,
            // bad rustfmt
            value.color_0_deg.blue,
            value.color_45_deg.blue,
            value.color_90_deg.blue,
            value.color_135_deg.blue,
            value.color_180_deg.blue,
            value.color_225_deg.blue,
            value.color_270_deg.blue,
            value.color_315_deg.blue,
        ]
    }
}

impl From<Skull> for [f32; 12] {
    fn from(value: Skull) -> Self {
        [
            value.left_front_0,
            value.left_front_1,
            value.left_middle_0,
            value.left_rear_0,
            value.left_rear_1,
            value.left_rear_2,
            value.right_rear_2,
            value.right_rear_1,
            value.right_rear_0,
            value.right_middle_0,
            value.right_front_0,
            value.right_front_1,
        ]
    }
}

impl<T: Copy> From<[T; 25]> for JointArray<T> {
    fn from(value: [T; 25]) -> Self {
        Self {
            head_yaw: value[0],
            head_pitch: value[1],

            left_shoulder_pitch: value[2],
            left_shoulder_roll: value[3],
            left_elbow_yaw: value[4],
            left_elbow_roll: value[5],
            left_wrist_yaw: value[6],
            left_hip_yaw_pitch: value[7],
            left_hip_roll: value[8],
            left_hip_pitch: value[9],
            left_knee_pitch: value[10],
            left_ankle_pitch: value[11],
            left_ankle_roll: value[12],

            right_hip_roll: value[13],
            right_hip_pitch: value[14],
            right_knee_pitch: value[15],
            right_ankle_pitch: value[16],
            right_ankle_roll: value[17],
            right_shoulder_pitch: value[18],
            right_shoulder_roll: value[19],
            right_elbow_yaw: value[20],
            right_elbow_roll: value[21],
            right_wrist_yaw: value[22],

            left_hand: value[23],
            right_hand: value[24],
        }
    }
}

impl<T> From<JointArray<T>> for [T; 25] {
    fn from(value: JointArray<T>) -> Self {
        [
            value.head_yaw,
            value.head_pitch,
            value.left_shoulder_pitch,
            value.left_shoulder_roll,
            value.left_elbow_yaw,
            value.left_elbow_roll,
            value.left_wrist_yaw,
            value.left_hip_yaw_pitch,
            value.left_hip_roll,
            value.left_hip_pitch,
            value.left_knee_pitch,
            value.left_ankle_pitch,
            value.left_ankle_roll,
            value.right_hip_roll,
            value.right_hip_pitch,
            value.right_knee_pitch,
            value.right_ankle_pitch,
            value.right_ankle_roll,
            value.right_shoulder_pitch,
            value.right_shoulder_roll,
            value.right_elbow_yaw,
            value.right_elbow_roll,
            value.right_wrist_yaw,
            value.left_hand,
            value.right_hand,
        ]
    }
}

impl From<[f32; 4]> for Battery {
    fn from(value: [f32; 4]) -> Self {
        Battery {
            charge: value[0],
            current: value[1],
            status: value[2],
            temperature: value[3],
        }
    }
}

impl From<[f32; 8]> for ForceSensitiveResistors {
    fn from(value: [f32; 8]) -> Self {
        let left: [f32; 4] = value[..4].try_into().unwrap();
        let right: [f32; 4] = value[4..].try_into().unwrap();

        Self {
            left_foot: left.into(),
            right_foot: right.into(),
        }
    }
}

impl From<[f32; 4]> for ForceSensitiveResistorFoot {
    fn from(value: [f32; 4]) -> Self {
        Self {
            front_left: value[0],
            front_right: value[1],
            rear_left: value[2],
            rear_right: value[3],
        }
    }
}

impl<T: Copy> From<[T; 2]> for Sonar<T> {
    fn from(value: [T; 2]) -> Self {
        Sonar {
            left: value[0],
            right: value[1],
        }
    }
}

impl<T> From<Sonar<T>> for [T; 2] {
    fn from(value: Sonar<T>) -> Self {
        [value.left, value.right]
    }
}

impl From<[f32; 14]> for Touch {
    fn from(value: [f32; 14]) -> Self {
        Self {
            chest_board: value[0],
            head_front: value[1],
            head_middle: value[2],
            head_rear: value[3],
            left_foot_left: value[4],
            left_foot_right: value[5],
            left_hand_back: value[6],
            left_hand_left: value[7],
            left_hand_right: value[8],
            right_foot_left: value[9],
            right_foot_right: value[10],
            right_hand_back: value[11],
            right_hand_left: value[12],
            right_hand_right: value[13],
        }
    }
}

impl From<[f32; 2]> for Vector2<f32> {
    fn from(value: [f32; 2]) -> Self {
        Vector2 {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<[f32; 3]> for Vector3<f32> {
    fn from(value: [f32; 3]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawState<'a> {
    pub stiffness: [f32; 25],
    pub position: [f32; 25],
    pub temperature: [f32; 25],
    pub current: [f32; 25],
    pub battery: [f32; 4],
    pub accelerometer: [f32; 3],
    pub gyroscope: [f32; 3],
    pub angles: [f32; 2],
    pub sonar: [f32; 2],
    pub f_s_r: [f32; 8],
    pub touch: [f32; 14],
    pub status: [i32; 25],
    #[serde(borrow)]
    pub robot_config: [&'a str; 4],
}

impl From<RawState<'_>> for State {
    fn from(value: RawState) -> Self {
        Self {
            stiffness: value.stiffness.into(),
            position: value.position.into(),
            temperature: value.temperature.into(),
            current: value.current.into(),
            battery: value.battery.into(),
            accelerometer: value.accelerometer.into(),
            gyroscope: value.gyroscope.into(),
            angles: value.angles.into(),
            sonar: value.sonar.into(),
            force_sensitive_resistors: value.f_s_r.into(),
            touch: value.touch.into(),
            status: value.status.into(),
        }
    }
}

impl<'a> From<RawState<'a>> for HardwareInfo {
    fn from(value: RawState<'a>) -> Self {
        Self {
            body_id: value.robot_config[0].to_string(),
            body_version: value.robot_config[1].to_string(),
            head_id: value.robot_config[2].to_string(),
            head_version: value.robot_config[3].to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawUpdate {
    pub position: [f32; 25],
    pub stiffness: [f32; 25],
    pub r_ear: [f32; 10],
    pub l_ear: [f32; 10],
    pub chest: [f32; 3],
    pub l_eye: [f32; 24],
    pub r_eye: [f32; 24],
    pub l_foot: [f32; 3],
    pub r_foot: [f32; 3],
    pub skull: [f32; 12],
    pub sonar: [bool; 2],
}

impl From<Update> for RawUpdate {
    fn from(value: Update) -> Self {
        Self {
            position: value.position.into(),
            stiffness: value.stiffness.into(),
            r_ear: value.right_ear.into(),
            l_ear: value.left_ear.into(),
            chest: value.chest.into(),
            l_eye: value.left_eye.into(),
            r_eye: value.right_eye.into(),
            l_foot: value.left_foot.into(),
            r_foot: value.right_foot.into(),
            skull: value.skull.into(),
            sonar: value.sonar.into(),
        }
    }
}
