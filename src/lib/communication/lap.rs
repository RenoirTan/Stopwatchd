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
pub struct LapRequest {
    pub identifiers: Vec<String>,
    pub verbose: bool
}

impl Codecable<'_> for LapRequest { }

impl Into<ClientRequest> for LapRequest {
    fn into(self) -> ClientRequest {
        ClientRequest::Lap(self)
    }
}

impl Into<ClientMessage> for LapRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapReply {
    pub success: HashMap<Identifier, LapSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl LapReply {
    pub fn new() -> Self {
        LapReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, stop: LapSuccess) {
        let identifier = Identifier::from_uuid_name(&stop.details.get_uuid_name());
        self.success.insert(identifier, stop);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for LapReply {
    type Successful = LapSuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let success = map_identifier_to_values(iter, LapSuccess::get_identifier);
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for LapReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let errored = map_identifier_to_values(iter, |e| e.identifier.clone());
        Self { success: HashMap::new(), errored }
    }
}

impl Into<ServerReply> for LapReply {
    fn into(self) -> ServerReply {
        ServerReply::Lap(self)
    }
}

impl Into<ServerMessage> for LapReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for LapReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapSuccess {
    pub details: StopwatchDetails
}

impl LapSuccess {
    pub fn to_reply(self) -> LapReply {
        let mut sr = LapReply::new();
        sr.add_success(self);
        sr
    }

    pub fn get_identifier(&self) -> Identifier {
        self.details.get_identifier()
    }
}

impl Codecable<'_> for LapSuccess { }

impl FromStopwatch for LapSuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for LapSuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<LapReply> for LapSuccess {
    fn into(self) -> LapReply {
        self.to_reply()
    }
}

impl Into<ServerReply> for LapSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Lap(self.into())
    }
}

impl Into<ServerMessage> for LapSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}