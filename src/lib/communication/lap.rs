//! Create a new lap for the specified [`Stopwatch`]es.

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Request to create a new lap.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapRequest;

impl Codecable<'_> for LapRequest { }

impl Into<ClientRequestKind> for LapRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Lap(self)
    }
}

/// Reply from `swd` creating new laps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapReply;

impl Into<ServerReplyKind> for LapReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Lap(self)
    }
}