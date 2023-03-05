use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayRequest;

impl Codecable<'_> for PlayRequest { }

impl Into<ClientRequestKind> for PlayRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Play(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayReply;

impl Into<ServerReplyKind> for PlayReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Play(self)
    }
}