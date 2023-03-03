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
pub struct PlayRequest {
    pub identifiers: Vec<Identifier>,
    pub verbose: bool
}

impl Codecable<'_> for PlayRequest { }

impl Into<ClientRequest> for PlayRequest {
    fn into(self) -> ClientRequest {
        ClientRequest::Play(self)
    }
}

impl Into<ClientMessage> for PlayRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayReply {
    pub success: HashMap<Identifier, PlaySuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl PlayReply {
    pub fn new() -> Self {
        PlayReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, stop: PlaySuccess) {
        let identifier = Identifier::from_uuid_name(&stop.details.get_uuid_name());
        self.success.insert(identifier, stop);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for PlayReply {
    type Successful = PlaySuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let success = map_identifier_to_values(iter, PlaySuccess::get_identifier);
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for PlayReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let errored = map_identifier_to_values(iter, |e| e.identifier.clone());
        Self { success: HashMap::new(), errored }
    }
}

impl Into<ServerReply> for PlayReply {
    fn into(self) -> ServerReply {
        ServerReply::Play(self)
    }
}

impl Into<ServerMessage> for PlayReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for PlayReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaySuccess {
    pub details: StopwatchDetails
}

impl PlaySuccess {
    pub fn to_reply(self) -> PlayReply {
        let mut sr = PlayReply::new();
        sr.add_success(self);
        sr
    }

    pub fn get_identifier(&self) -> Identifier {
        self.details.get_identifier()
    }
}

impl Codecable<'_> for PlaySuccess { }

impl FromStopwatch for PlaySuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for PlaySuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<PlayReply> for PlaySuccess {
    fn into(self) -> PlayReply {
        self.to_reply()
    }
}

impl Into<ServerReply> for PlaySuccess {
    fn into(self) -> ServerReply {
        ServerReply::Play(self.into())
    }
}

impl Into<ServerMessage> for PlaySuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}