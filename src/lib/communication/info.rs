use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    traits::Codecable,
    models::stopwatch::Stopwatch,
    error::FindStopwatchError, identifiers::Identifier
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
    pub success: HashMap<Identifier, InfoSuccess>,
    pub errored: HashMap<Identifier, FindStopwatchError>
}

impl InfoReply {
    pub fn from_stopwatch_iter<'s, I>(iter: I, verbose: bool) -> Self
    where
        I: Iterator<Item = &'s Stopwatch>
    {
        let mut success = HashMap::new();
        for stopwatch in iter {
            let identifier = Identifier::from_uuid_name(&stopwatch.get_uuid_name());
            let details = InfoSuccess::from_stopwatch(stopwatch, verbose);
            success.insert(identifier, details);
        }
        Self {
            success,
            errored: HashMap::new()
        }
    }

    pub fn from_success_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = InfoSuccess>
    {
        let mut success = HashMap::new();
        for info in iter {
            let identifier = Identifier::from_uuid_name(&info.details.get_uuid_name());
            success.insert(identifier, info);
        }
        Self { success, errored: HashMap::new() }
    }

    pub fn from_details_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = StopwatchDetails>
    {
        Self::from_success_iter(iter.map(|d| InfoSuccess { details: d }))
    }

    pub fn from_err_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = FindStopwatchError>
    {
        let mut errored = HashMap::new();
        for fse in iter {
            errored.insert(fse.identifier.clone(), fse);
        }
        Self {
            success: HashMap::new(),
            errored
        }
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
    pub fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let details = StopwatchDetails::from_stopwatch(stopwatch, verbose);
        Self { details }
    }

    pub fn to_reply(self) -> InfoReply {
        let mut success = HashMap::new();
        let identifier = Identifier::from_uuid_name(&self.details.get_uuid_name());
        success.insert(identifier, self);
        InfoReply { success, errored: HashMap::new() }
    }
}

impl Codecable<'_> for InfoSuccess { }

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