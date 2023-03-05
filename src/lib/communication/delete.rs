use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRequest;

impl Codecable<'_> for DeleteRequest { }

impl Into<ClientRequestKind> for DeleteRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Delete(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteReply;

impl Into<ServerReplyKind> for DeleteReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Delete(self)
    }
}