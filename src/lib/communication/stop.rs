//! Permanently terminate a stopwatch.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Reply from `swd` after stopping a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopReply;

impl Into<ReplyKind> for StopReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Stop(self)
    }
}