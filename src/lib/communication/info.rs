use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    traits::{Codecable, FromStopwatch, FromSuccessfuls, FromErrors},
    models::stopwatch::Stopwatch,
    error::FindStopwatchError, identifiers::Identifier
};

use super::{
    client_message::{ClientRequest, ClientMessage},
    server_message::{ServerReply, ServerMessage}, details::StopwatchDetails
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequest {
    pub identifiers: Vec<Identifier>,
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
    pub success: HashMap<Identifier, InfoSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl InfoReply {
    pub fn new() -> Self {
        InfoReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, info: InfoSuccess) {
        let identifier = Identifier::from_uuid_name(&info.details.get_uuid_name());
        self.success.insert(identifier, info);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for InfoReply {
    type Successful = InfoSuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let mut success = HashMap::new();
        for info in iter {
            let identifier = Identifier::from_uuid_name(&info.details.get_uuid_name());
            success.insert(identifier, info);
        }
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for InfoReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let mut errored = HashMap::new();
        for error in iter {
            let identifier = error.identifier.clone();
            errored.insert(identifier, error);
        }
        Self { success: HashMap::new(), errored }
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
    pub details: StopwatchDetails
}

impl InfoSuccess {
    pub fn to_reply(self) -> InfoReply {
        let mut success = HashMap::new();
        let identifier = Identifier::from_uuid_name(&self.details.get_uuid_name());
        success.insert(identifier, self);
        InfoReply { success, errored: HashMap::new() }
    }
}

impl Codecable<'_> for InfoSuccess { }

impl FromStopwatch for InfoSuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for InfoSuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<InfoReply> for InfoSuccess {
    fn into(self) -> InfoReply {
        self.to_reply()
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