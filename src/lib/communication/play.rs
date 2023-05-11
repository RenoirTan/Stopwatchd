//! Continue running the timer of a lap.

use serde::{Serialize, Deserialize};

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Request to play a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayRequest;

impl Into<ClientRequestKind> for PlayRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Play(self)
    }
}

/// Reply from `swd` after playing a [`Stopwatch`]'s lap.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayReply;

impl Into<ServerReplyKind> for PlayReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Play(self)
    }
}