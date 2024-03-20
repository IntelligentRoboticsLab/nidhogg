use std::time::Duration;

use nidhogg::{
    backend::{ConnectWithRetry, LolaBackend, ReadHardwareInfo},
    types::{LeftEye, RgbF32},
    NaoBackend, NaoControlMessage,
};

use miette::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut nao = LolaBackend::connect_with_retry(10, Duration::from_millis(500))?;

    let state = nao.read_nao_state()?;
    let hw_info = nao.read_hardware_info()?;

    println!("{hw_info:?}");
    println!("{state:?}");

    let update = NaoControlMessage::builder()
        .left_eye(
            LeftEye::builder()
                .l0(RgbF32::builder().red(1.0).green(1.0).build())
                .build(),
        )
        .build();

    nao.send_control_msg(update)?;

    Ok(())
}
