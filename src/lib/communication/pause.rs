//! Pause a running lap.

use serde::{Serialize, Deserialize};

use super::{
    reply::ReplyKind,
    request::RequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Request to pause a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseRequest;

impl Into<RequestKind> for PauseRequest {
    fn into(self) -> RequestKind {
        RequestKind::Pause(self)
    }
}

/// Reply from `swd` after pausing a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseReply;

impl Into<ReplyKind> for PauseReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Pause(self)
    }
}