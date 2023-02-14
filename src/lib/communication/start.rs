use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::stopwatch::{Name, State, Stopwatch}
};

use super::{
    client_message::{ClientMessage, ClientRequest},
    server_message::{ServerReply, ServerMessage}
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientStartStopwatch {
    pub name: Option<Name>,
    pub verbose: bool
}

impl Codecable<'_> for ClientStartStopwatch { }

impl Into<ClientRequest> for ClientStartStopwatch {
    fn into(self) -> ClientRequest {
        ClientRequest::Start(self)
    }
}

impl Into<ClientMessage> for ClientStartStopwatch {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStartStopwatch {
    pub sw_id: Uuid,
    pub name: Option<Name>,
    pub state: State,
    pub start_time: Option<SystemTime>
}

impl Codecable<'_> for ServerStartStopwatch { }

impl From<&Stopwatch> for ServerStartStopwatch {
    fn from(stopwatch: &Stopwatch) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name;
        let state = stopwatch.state();
        let start_time = stopwatch.start_time();
        Self { sw_id, name, state, start_time }
    }
}

impl Into<ServerReply> for ServerStartStopwatch {
    fn into(self) -> ServerReply {
        ServerReply::Start(self)
    }
}

impl Into<ServerMessage> for ServerStartStopwatch {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}