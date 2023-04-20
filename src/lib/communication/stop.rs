use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Stop a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopRequest;

impl Codecable<'_> for StopRequest { }

impl Into<ClientRequestKind> for StopRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Stop(self)
    }
}

/// Reply from `swd` after stopping a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopReply;

impl Into<ServerReplyKind> for StopReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Stop(self)
    }
}