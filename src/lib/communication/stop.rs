use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    traits::{Codecable, FromStopwatch, FromSuccessfuls, FromErrors},
    models::stopwatch::Stopwatch,
    error::FindStopwatchError,
    identifiers::Identifier, util::map_identifier_to_values
};

use super::{
    client_message::{ClientRequest, ClientMessage},
    server_message::{ServerReply, ServerMessage},
    details::StopwatchDetails
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
    pub success: HashMap<Identifier, StopSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl StopReply {
    pub fn new() -> Self {
        StopReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, stop: StopSuccess) {
        let identifier = Identifier::from_uuid_name(&stop.details.get_uuid_name());
        self.success.insert(identifier, stop);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for StopReply {
    type Successful = StopSuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let success = map_identifier_to_values(iter, StopSuccess::get_identifier);
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for StopReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let errored = map_identifier_to_values(iter, |e| e.identifier.clone());
        Self { success: HashMap::new(), errored }
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
    pub details: StopwatchDetails
}

impl StopSuccess {
    pub fn to_reply(self) -> StopReply {
        let mut sr = StopReply::new();
        sr.add_success(self);
        sr
    }

    pub fn get_identifier(&self) -> Identifier {
        self.details.get_identifier()
    }
}

impl Codecable<'_> for StopSuccess { }

impl FromStopwatch for StopSuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for StopSuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<StopReply> for StopSuccess {
    fn into(self) -> StopReply {
        self.to_reply()
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