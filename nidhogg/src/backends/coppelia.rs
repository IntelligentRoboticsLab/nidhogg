use crate::{NaoBackend, NaoControlMessage, NaoState, Result};
use zmq_remote_api::{RemoteApiClient, RemoteApiClientParams};

#[allow(missing_debug_implementations)]
pub struct CopelliaBackend {
    #[allow(dead_code)]
    client: RemoteApiClient,
}

impl NaoBackend for CopelliaBackend {
    fn connect() -> Result<Self> {
        let client = RemoteApiClient::new(RemoteApiClientParams {
            host: "localhost".to_string(),
            ..RemoteApiClientParams::default()
        })
        .map_err(|e| crate::error::Error::CoppelliaConnectError(e.show()))?;

        Ok(CopelliaBackend { client })
    }

    fn send_control_msg(
        &mut self,
        #[allow(unused_variables)] update: NaoControlMessage,
    ) -> Result<()> {
        todo!("implement writing to coppelia")
    }

    fn read_nao_state(&mut self) -> Result<NaoState> {
        todo!("implement reading from coppelia")
    }
}
