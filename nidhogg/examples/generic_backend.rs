use std::time::Duration;

use nidhogg::{
    backend::{ConnectWithRetryExt, LolaBackend},
    types::{Color, LeftEye},
    NaoBackend, NaoControlMessage, NaoState,
};

use miette::Result;

struct App<B: NaoBackend> {
    backend: B,
    #[allow(dead_code)]
    state: NaoState,
}

// These methods will work with all backends.
impl<B: NaoBackend> App<B> {
    #[allow(dead_code)]
    pub fn init() -> Result<Self> {
        let mut backend = B::connect()?;
        let state = backend.read_nao_state()?;
        Ok(Self { backend, state })
    }

    pub fn send_control_msg(&mut self, msg: NaoControlMessage) -> Result<()> {
        self.backend.send_control_msg(msg)?;
        Ok(())
    }
}

// These methods will only work with the LoLA backend
impl App<LolaBackend> {
    pub fn init_with_retry(retry_count: u32, retry_interval: Duration) -> Result<Self> {
        let mut backend = LolaBackend::connect_with_retry(retry_count, retry_interval)?;
        let state = backend.read_nao_state()?;
        Ok(Self { backend, state })
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let mut app: App<CoppeliaBackend> = App::init()?;
    let mut app: App<LolaBackend> = App::init_with_retry(10, Duration::from_millis(500))?;

    let update = NaoControlMessage::builder()
        .left_eye(
            LeftEye::builder()
                .color_0_deg(Color::builder().red(1.0).green(1.0).build())
                .build(),
        )
        .build();

    app.send_control_msg(update)?;

    Ok(())
}
