//! Delete a stopwatch from `swd`.

use serde::{Serialize, Deserialize};

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

/// Delete action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRequest;

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