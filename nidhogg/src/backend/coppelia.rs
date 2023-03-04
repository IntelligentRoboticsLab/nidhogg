use crate::NaoRobot;
use zmq_remote_api::{RemoteApiClient, RemoteApiClientParams};

// todo: need to add references to all joints most likely, could maybe use `JointArray<Joint>` for this
// todo: and then write values to each joint when writing update
pub struct CopelliaCommunicator {
    client: RemoteApiClient,
}

impl NaoRobot for CopelliaCommunicator {
    type Backend = CopelliaCommunicator;

    fn connect() -> crate::Result<Self::Backend> {
        
        let client = RemoteApiClient::new(RemoteApiClientParams {
            host: "localhost".to_string(),
            ..RemoteApiClientParams::default()
        })
        // XXX: probably want to fork this project so we can get rid of this atrocity.
        .map_err(|e| crate::error::Error::CoppelliaConnectError(e.show()))?; 

        Ok(CopelliaCommunicator { client: client })
    }

    fn write_update(&mut self, update: crate::Update) -> crate::Result<()> {
        todo!("implement writing to coppelia")
    }

    fn read_state(&mut self) -> crate::Result<crate::State> {
        todo!("implement reading from coppelia")
    }
}