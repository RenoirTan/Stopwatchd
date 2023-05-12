//! Start a new stopwatch.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Reply from `swd` after creating new stopwatches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartReply;

impl Into<ReplyKind> for StartReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Start(self)
    }
}