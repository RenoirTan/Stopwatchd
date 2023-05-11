//! Start a new stopwatch.

use serde::{Serialize, Deserialize};

use super::{
    server::ReplyKind,
    client::RequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Get `swd` to create a new [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRequest;

impl Into<RequestKind> for StartRequest {
    fn into(self) -> RequestKind {
        RequestKind::Start(self)
    }
}

/// Reply from `swd` after creating new stopwatches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartReply;

impl Into<ReplyKind> for StartReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Start(self)
    }
}