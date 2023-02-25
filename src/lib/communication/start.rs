use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::stopwatch::{Name, State, Stopwatch, FindStopwatchError}
};

use super::{
    client_message::{ClientMessage, ClientRequest},
    server_message::{ServerReply, ServerMessage}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientStartStopwatch {
    pub name: Name,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStartStopwatch {
    pub start: Result<ServerStartStopwatchInner, FindStopwatchError>
}

impl ServerStartStopwatch {
    pub fn started(&self) -> bool {
        self.start.is_ok()
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

impl Codecable<'_> for ServerStartStopwatch { }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStartStopwatchInner {
    pub sw_id: Uuid,
    pub name: Name,
    pub state: State,
    pub start_time: Option<SystemTime>
}

impl Codecable<'_> for ServerStartStopwatchInner { }

impl From<&Stopwatch> for ServerStartStopwatchInner {
    fn from(stopwatch: &Stopwatch) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name.clone();
        let state = stopwatch.state();
        let start_time = stopwatch.start_time();
        Self { sw_id, name, state, start_time }
    }
}

impl Into<ServerStartStopwatch> for ServerStartStopwatchInner {
    fn into(self) -> ServerStartStopwatch {
        ServerStartStopwatch { start: Ok(self) }
    }
}

impl Into<ServerReply> for ServerStartStopwatchInner {
    fn into(self) -> ServerReply {
        ServerReply::Start(self.into())
    }
}

impl Into<ServerMessage> for ServerStartStopwatchInner {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}