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

    // TODO: Make nice builder structs :);
    let green = Color::builder().green(1.0).blue(0.0).build();

    let update = Update::builder()
        .left_eye(
            LeftEye::builder()
                .color_0_deg(Color::builder().red(1.0).green(1.0).build())
                .build(),
        )
        .build();
    nao.write_update(update)?;

    Ok(())
}
