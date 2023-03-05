use serde::{Serialize, Deserialize};

use crate::{traits::Codecable, identifiers::Identifier};

use super::{
    server_message::ServerReplyKind,
    client_message::ClientRequestKind
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequest;

impl Codecable<'_> for InfoRequest { }

impl Into<ClientRequestKind> for InfoRequest {
    fn into(self) -> ClientRequestKind {
        ClientRequestKind::Info(self)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfoReply {
    #[default] Basic,
    All(InfoAll)
}

impl Into<ServerReplyKind> for InfoReply {
    fn into(self) -> ServerReplyKind {
        ServerReplyKind::Info(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoAll {
    pub access_order: Vec<Identifier>
}

impl Into<InfoReply> for InfoAll {
    fn into(self) -> InfoReply {
        InfoReply::All(self)
    }
}