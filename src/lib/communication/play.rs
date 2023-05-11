//! Continue running the timer of a lap.

use serde::{Serialize, Deserialize};

use super::{
    reply::ReplyKind,
    request::RequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Request to play a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayRequest;

impl Into<RequestKind> for PlayRequest {
    fn into(self) -> RequestKind {
        RequestKind::Play(self)
    }
}

/// Reply from `swd` after playing a [`Stopwatch`]'s lap.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayReply;

impl Into<ReplyKind> for PlayReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Play(self)
    }
}