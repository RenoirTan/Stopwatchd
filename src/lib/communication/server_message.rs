use std::{process, io};

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{start::ServerStartStopwatch, info::ServerInfoStopwatch};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerReply {
    Start(ServerStartStopwatch),
    Info(ServerInfoStopwatch),
    #[default] Default
}

impl Into<ServerMessage> for ServerReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self)
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