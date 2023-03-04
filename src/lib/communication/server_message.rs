use std::{process, io, fmt};

use serde::{Serialize, Deserialize};

use crate::{traits::Codecable, error::FindStopwatchError};

use super::{start::StartReply, info::InfoReply, stop::StopReply, lap::LapReply, pause::PauseReply, play::PlayReply, delete::DeleteReply};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerReply {
    Start(StartReply),
    Info(InfoReply),
    Stop(StopReply),
    Lap(LapReply),
    Pause(PauseReply),
    Play(PlayReply),
    Delete(DeleteReply),
    #[default] Default
}

impl Into<ServerMessage> for ServerReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerError {
    FindStopwatchError(FindStopwatchError),
    Other(String)
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ServerError::*;
        match self {
            FindStopwatchError(fse) => write!(f, "{}", fse.diagnose()),
            Other(s) => write!(f, "{}", s)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerMessage {
    pub pid: u32,
    pub reply: ServerReply
}

impl ServerMessage {
    pub fn create(reply: ServerReply) -> Self {
        let pid = process::id();
        Self { pid, reply }
    }
}

impl Codecable<'_> for ServerMessage { }

impl Default for ServerMessage {
    fn default() -> Self {
        let pid = process::id();
        let reply = ServerReply::default();
        Self { pid, reply }
    }
}

impl TryFrom<&[u8]> for ServerMessage {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl TryInto<Vec<u8>> for ServerMessage {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}