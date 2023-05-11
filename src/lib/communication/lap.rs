//! Create a new lap for the specified [`Stopwatch`]es.

use serde::{Serialize, Deserialize};

use super::{
    reply::ReplyKind,
    request::RequestKind
};

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Request to create a new lap.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapRequest;

impl Into<RequestKind> for LapRequest {
    fn into(self) -> RequestKind {
        RequestKind::Lap(self)
    }
}

/// Reply from `swd` creating new laps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapReply;

impl Into<ReplyKind> for LapReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Lap(self)
    }
}