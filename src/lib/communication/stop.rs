//! Permanently terminate a stopwatch.

use serde::{Serialize, Deserialize};

use super::{
    server::ReplyKind,
    client::RequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Stop a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopRequest;

impl Into<RequestKind> for StopRequest {
    fn into(self) -> RequestKind {
        RequestKind::Stop(self)
    }
}

/// Reply from `swd` after stopping a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopReply;

impl Into<ReplyKind> for StopReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Stop(self)
    }
}