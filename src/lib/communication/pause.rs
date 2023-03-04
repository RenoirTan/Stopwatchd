use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    traits::{Codecable, FromStopwatch, FromSuccessfuls, FromErrors},
    models::stopwatch::Stopwatch,
    error::FindStopwatchError,
    identifiers::Identifier, util::map_identifier_to_values
};

use super::{
    server_message::{ServerReply, ServerMessage},
    details::StopwatchDetails,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseRequest;

impl Codecable<'_> for PauseRequest { }

impl Into<ClientRequestKind> for PauseRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Pause(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseReply {
    pub success: HashMap<Identifier, PauseSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl PauseReply {
    pub fn new() -> Self {
        PauseReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, stop: PauseSuccess) {
        let identifier = Identifier::from_uuid_name(&stop.details.get_uuid_name());
        self.success.insert(identifier, stop);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for PauseReply {
    type Successful = PauseSuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let success = map_identifier_to_values(iter, PauseSuccess::get_identifier);
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for PauseReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let errored = map_identifier_to_values(iter, |e| e.identifier.clone());
        Self { success: HashMap::new(), errored }
    }
}

impl Into<ServerReply> for PauseReply {
    fn into(self) -> ServerReply {
        ServerReply::Pause(self)
    }
}

impl Into<ServerMessage> for PauseReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for PauseReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseSuccess {
    pub details: StopwatchDetails
}

impl PauseSuccess {
    pub fn to_reply(self) -> PauseReply {
        let mut sr = PauseReply::new();
        sr.add_success(self);
        sr
    }

    pub fn get_identifier(&self) -> Identifier {
        self.details.get_identifier()
    }
}

impl Codecable<'_> for PauseSuccess { }

impl FromStopwatch for PauseSuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for PauseSuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<PauseReply> for PauseSuccess {
    fn into(self) -> PauseReply {
        self.to_reply()
    }
}

impl Into<ServerReply> for PauseSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Pause(self.into())
    }
}

impl Into<ServerMessage> for PauseSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}