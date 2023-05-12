//! Continue running the timer of a lap.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Reply from `swd` after playing a [`Stopwatch`]'s lap.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayReply;

impl Into<ReplyKind> for PlayReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Play(self)
    }
}