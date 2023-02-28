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
pub struct StopRequest {
    pub identifiers: Vec<String>,
    pub verbose: bool
}

impl Codecable<'_> for StopRequest { }

impl Into<ClientRequest> for StopRequest {
    fn into(self) -> ClientRequest {
        ClientRequest::Stop(self)
    }
}

impl Into<ClientMessage> for StopRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopReply {
    pub result: Result<StopSuccess, FindStopwatchError>
}

impl StopReply {
    pub fn found(&self) -> bool {
        self.result.is_ok()
    }
}

impl Into<ServerReply> for StopReply {
    fn into(self) -> ServerReply {
        ServerReply::Stop(self)
    }
}

impl Into<ServerMessage> for StopReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for StopReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopSuccess {
    pub details: Vec<StopwatchDetails>
}

impl StopSuccess {
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

impl Codecable<'_> for StopSuccess { }

impl Into<StopReply> for StopSuccess {
    fn into(self) -> StopReply {
        StopReply { result: Ok(self) }
    }
}

impl Into<ServerReply> for StopSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Stop(self.into())
    }
}

impl Into<ServerMessage> for StopSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}