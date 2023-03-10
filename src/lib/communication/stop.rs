use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopRequest;

impl Codecable<'_> for StopRequest { }

impl Into<ClientRequestKind> for StopRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Stop(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopReply;

impl Into<ServerReplyKind> for StopReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Stop(self)
    }
}