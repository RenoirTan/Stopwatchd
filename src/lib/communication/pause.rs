//! Pause a running lap.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Reply from `swd` after pausing a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseReply;

impl Into<ReplyKind> for PauseReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Pause(self)
    }
}