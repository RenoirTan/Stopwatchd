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
pub struct DeleteRequest;

impl Codecable<'_> for DeleteRequest { }

impl Into<ClientRequestKind> for DeleteRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Delete(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteReply {
    pub success: HashMap<Identifier, DeleteSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl DeleteReply {
    pub fn new() -> Self {
        DeleteReply { success: HashMap::new(), errored: HashMap::new() }
    }

    pub fn add_success(&mut self, stop: DeleteSuccess) {
        let identifier = Identifier::from_uuid_name(&stop.details.get_uuid_name());
        self.success.insert(identifier, stop);
    }

    pub fn add_error(&mut self, fse: FindStopwatchError) {
        let identifier = fse.identifier.clone();
        self.errored.insert(identifier, fse);
    }
}

impl FromSuccessfuls for DeleteReply {
    type Successful = DeleteSuccess;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>
    {
        let success = map_identifier_to_values(iter, DeleteSuccess::get_identifier);
        Self { success, errored: HashMap::new() }
    }
}

impl FromErrors for DeleteReply {
    type Error = FindStopwatchError;

    fn from_errors<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Error>
    {
        let errored = map_identifier_to_values(iter, |e| e.identifier.clone());
        Self { success: HashMap::new(), errored }
    }
}

impl Into<ServerReply> for DeleteReply {
    fn into(self) -> ServerReply {
        ServerReply::Delete(self)
    }
}

impl Into<ServerMessage> for DeleteReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for DeleteReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteSuccess {
    pub details: StopwatchDetails
}

impl DeleteSuccess {
    pub fn to_reply(self) -> DeleteReply {
        let mut sr = DeleteReply::new();
        sr.add_success(self);
        sr
    }

    pub fn get_identifier(&self) -> Identifier {
        self.details.get_identifier()
    }
}

impl Codecable<'_> for DeleteSuccess { }

impl FromStopwatch for DeleteSuccess {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }
}

impl From<StopwatchDetails> for DeleteSuccess {
    fn from(details: StopwatchDetails) -> Self {
        Self { details }
    }
}

impl Into<DeleteReply> for DeleteSuccess {
    fn into(self) -> DeleteReply {
        self.to_reply()
    }
}

impl Into<ServerReply> for DeleteSuccess {
    fn into(self) -> ServerReply {
        ServerReply::Delete(self.into())
    }
}

impl Into<ServerMessage> for DeleteSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}