//! Start a new stopwatch.

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Get `swd` to create a new [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRequest;

impl Codecable<'_> for StartRequest { }

impl Into<ClientRequestKind> for StartRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Start(self)
    }
}

/// Reply from `swd` after creating new stopwatches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartReply;

impl Into<ServerReplyKind> for StartReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Start(self)
    }
}