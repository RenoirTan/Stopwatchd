use serde::{Serialize, Deserialize};

use crate::{
    traits::Codecable,
    models::stopwatch::{Stopwatch, FindStopwatchError}
};

use super::{
    client_message::{ClientRequest, ClientMessage},
    server_message::{ServerReply, ServerMessage}, details::StopwatchDetails
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequest {
    pub identifier: Option<String>,
    pub verbose: bool
}

impl Codecable<'_> for InfoRequest { }

impl Into<ClientRequest> for InfoRequest {
    fn into(self) -> ClientRequest {
        ClientRequest::Info(self)
    }
}

impl Into<ClientMessage> for InfoRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoReply {
    pub result: Result<InfoSuccess, FindStopwatchError>
}

impl InfoReply {
    pub fn found(&self) -> bool {
        self.result.is_ok()
    }
}

impl Into<ServerReply> for InfoReply {
    fn into(self) -> ServerReply {
        ServerReply::Info(self)
    }
}

impl Into<ServerMessage> for InfoReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for InfoReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoSuccess {
    pub details: Vec<StopwatchDetails>
}

impl InfoSuccess {
    pub fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = vec![StopwatchDetails::from_stopwatch(stopwatch, verbose)];
        Self { details }
    }

    pub fn from_iter<'s, I>(iter: I, verbose: bool) -> Self
    where
        I: Iterator<Item = &'s Stopwatch>
    {
        let details = StopwatchDetails::from_iter(iter, verbose);
        Self { details }
    }
}

impl Codecable<'_> for InfoSuccess { }

impl Into<InfoReply> for InfoSuccess {
    fn into(self) -> InfoReply {
        InfoReply { result: Ok(self) }
    }
}

impl Into<ServerReply> for InfoSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Info(self.into())
    }
}

impl Into<ServerMessage> for InfoSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}