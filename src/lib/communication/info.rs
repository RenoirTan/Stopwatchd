//! Grab information about a [`Stopwatch`] or a number of stopwatches.

use serde::{Serialize, Deserialize};

use crate::identifiers::Identifier;

use super::server::ReplyKind;

#[allow(unused)]
use crate::{
    communication::details::StopwatchDetails,
    models::stopwatch::Stopwatch
};

/// Kind of information coming from `swd`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfoReply {
    /// [`StopwatchDetails`] are returned in the order requested by the client.
    #[default] Basic,
    /// No stopwatch in particular was requested.
    All(InfoAll)
}

impl Into<ReplyKind> for InfoReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Info(self)
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