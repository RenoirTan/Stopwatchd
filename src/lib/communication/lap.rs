//! Create a new lap for the specified [`Stopwatch`]es.

use serde::{Serialize, Deserialize};

use super::server::ReplyKind;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// Reply from `swd` creating new laps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapReply;

impl Into<ReplyKind> for LapReply {
    fn into(self) -> ReplyKind {
        ReplyKind::Lap(self)
    }
}