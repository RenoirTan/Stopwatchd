use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::stopwatch::{Name, State, Stopwatch},
    error::FindStopwatchError
};

use super::{
    server_message::{ServerReply, ServerMessage},
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRequest;

impl Codecable<'_> for StartRequest { }

impl Into<ClientRequestKind> for StartRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Start(self)
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