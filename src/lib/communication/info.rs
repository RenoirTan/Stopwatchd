//! Grab information about a [`Stopwatch`] or a number of stopwatches.

use serde::{Serialize, Deserialize};

use crate::{traits::Codecable, identifiers::Identifier};

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[allow(unused)]
use crate::{
    communication::details::StopwatchDetails,
    models::stopwatch::Stopwatch
};

/// Request for information about stopwatches managed by `swd`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequest;

impl Codecable<'_> for InfoRequest { }

impl Into<ClientRequestKind> for InfoRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Info(self)
    }
}

/// Kind of information coming from `swd`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfoReply {
    /// [`StopwatchDetails`] are returned in the order requested by the client.
    #[default] Basic,
    /// No stopwatch in particular was requested.
    All(InfoAll)
}

impl Into<ServerReplyKind> for InfoReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Info(self)
    }
}

/// Stores details on how information should be presented when no particular
/// [`Stopwatch`] or stopwatches were requested.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoAll {
    /// Order in which stopwatches were last accessed.
    /// Provides a sequence that the client can show details in.
    pub access_order: Vec<Identifier>
}

impl Into<InfoReply> for InfoAll {
    fn into(self) -> InfoReply {
        InfoReply::All(self)
    }
}