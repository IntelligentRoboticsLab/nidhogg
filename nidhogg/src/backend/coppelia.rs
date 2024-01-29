use crate::{Error, NaoBackend, NaoControlMessage, NaoState, Result};
use coppeliasim_zmq_remote_api::{RemoteApiClient, RemoteApiClientParams};

use super::ConnectWithRetry;

#[allow(missing_debug_implementations)]
pub struct CoppeliaBackend {
    #[allow(dead_code)]
    client: RemoteApiClient,
}

impl NaoBackend for CoppeliaBackend {
    fn connect() -> Result<Self> {
        let client = RemoteApiClient::new(RemoteApiClientParams {
            host: "localhost".to_string(),
            ..RemoteApiClientParams::default()
        })
        .map_err(|e| Error::CoppeliaConnectError(e.show()))?;

        Ok(CoppeliaBackend { client })
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

impl ConnectWithRetry for CoppeliaBackend {}
