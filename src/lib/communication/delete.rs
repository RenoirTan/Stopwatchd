//! Delete a stopwatch from `swd`.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteReply;

impl Into<ReplyKind> for DeleteReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Delete(self)
    }
}