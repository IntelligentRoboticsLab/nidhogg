use crate::{NaoBackend, NaoControlMsg, NaoState, Result};
use zmq_remote_api::{RemoteApiClient, RemoteApiClientParams};

// todo: need to add references to all joints most likely, could maybe use `JointArray<Joint>` for this
// todo: and then write values to each joint when writing update
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
        // XXX: probably want to fork this project so we can get rid of this atrocity.
        .map_err(|e| crate::error::Error::CoppelliaConnectError(e.show()))?;

        Ok(CopelliaBackend { client: client })
    }

    fn send_control_msg(&mut self, #[allow(unused_variables)] update: NaoControlMsg) -> Result<()> {
        todo!("implement writing to coppelia")
    }

    fn read_nao_state(&mut self) -> Result<NaoState> {
        todo!("implement reading from coppelia")
    }
}
