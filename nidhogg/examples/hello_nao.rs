use std::time::Duration;

use nidhogg::types::{Color, LeftEye, RightEye};
use nidhogg::{Nao, Update};

use color_eyre::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let mut nao = Nao::connect_retry(10, Duration::from_millis(500))?;

    let state = nao.read_state()?;
    let hw_info = nao.read_hardware_info()?;

    println!("{hw_info:?}");
    println!("{state:?}");

    // TODO: Make nice builder structs :)
    let mut update = Update::default();
    let green = Color::new(0.0, 1.0, 0.0);

    update.left_eye = LeftEye {
        color_0_deg: green,
        color_45_deg: green,
        color_90_deg: green,
        color_135_deg: green,
        color_180_deg: green,
        color_225_deg: green,
        color_270_deg: green,
        color_315_deg: green,
        ..Default::default()
    };

    update.right_eye = RightEye {
        color_0_deg: green,
        color_45_deg: green,
        color_90_deg: green,
        color_135_deg: green,
        color_180_deg: green,
        color_225_deg: green,
        color_270_deg: green,
        color_315_deg: green,
        ..Default::default()
    };
    nao.write_update(update)?;

    Ok(())
}
