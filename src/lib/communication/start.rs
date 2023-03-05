use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRequest;

impl Codecable<'_> for StartRequest { }

impl Into<ClientRequestKind> for StartRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Start(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartReply;

impl Into<ServerReplyKind> for StartReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Start(self)
    }
}