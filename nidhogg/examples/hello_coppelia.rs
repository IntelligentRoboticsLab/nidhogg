use nidhogg::{
    backends::CoppeliaBackend,
    types::{JointArray},
    NaoBackend, NaoControlMessage,
};

use miette::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut nao = CoppeliaBackend::connect()?;

    let update = NaoControlMessage::builder()
        .position(
            JointArray::<f32>::builder().head_yaw(std::f32::consts::FRAC_PI_4).build()
        )
        .build();

    loop {
        nao.send_control_msg(update.clone())?;
    }

    Ok(())
}
