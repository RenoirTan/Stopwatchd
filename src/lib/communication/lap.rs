use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapRequest;

impl Codecable<'_> for LapRequest { }

impl Into<ClientRequestKind> for LapRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Lap(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapReply;

impl Into<ServerReplyKind> for LapReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Lap(self)
    }
}