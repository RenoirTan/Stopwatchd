//! Delete a stopwatch from `swd`.

use serde::{Serialize, Deserialize};

use super::{
    reply::ReplyKind,
    request::RequestKind
};

/// Delete action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRequest;

impl Into<RequestKind> for DeleteRequest {
    fn into(self) -> RequestKind {
        RequestKind::Delete(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteReply;

impl Into<ReplyKind> for DeleteReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Delete(self)
    }
}