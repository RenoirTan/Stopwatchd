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
pub struct StartRequest {
    pub name: Name,
    pub verbose: bool
}

impl Codecable<'_> for StartRequest { }

impl Into<ClientRequest> for StartRequest {
    fn into(self) -> ClientRequest {
        ClientRequest::Start(self)
    }
}

impl Into<ClientMessage> for StartRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartReply {
    pub start: Result<StartSuccess, FindStopwatchError>
}

impl StartReply {
    pub fn started(&self) -> bool {
        self.start.is_ok()
    }
}

impl Into<ServerReply> for StartReply {
    fn into(self) -> ServerReply {
        ServerReply::Start(self)
    }
}

impl Into<ServerMessage> for StartReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for StartReply { }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartSuccess {
    pub sw_id: Uuid,
    pub name: Name,
    pub state: State,
    pub start_time: Option<SystemTime>
}

impl Codecable<'_> for StartSuccess { }

impl From<&Stopwatch> for StartSuccess {
    fn from(stopwatch: &Stopwatch) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name.clone();
        let state = stopwatch.state();
        let start_time = stopwatch.start_time();
        Self { sw_id, name, state, start_time }
    }
}

impl Into<StartReply> for StartSuccess {
    fn into(self) -> StartReply {
        StartReply { start: Ok(self) }
    }
}

impl Into<ServerReply> for StartSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Start(self.into())
    }
}

impl Into<ServerMessage> for StartSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}