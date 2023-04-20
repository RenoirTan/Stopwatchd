//! Pause a running lap.

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

/// Request to pause a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseRequest;

impl Codecable<'_> for PauseRequest { }

impl Into<ClientRequestKind> for PauseRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Pause(self)
    }
}

/// Reply from `swd` after pausing a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseReply;

impl Into<ServerReplyKind> for PauseReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Pause(self)
    }
}