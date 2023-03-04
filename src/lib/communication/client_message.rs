use std::{process, io};

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

use super::{
    start::StartRequest,
    info::InfoRequest,
    stop::StopRequest,
    lap::LapRequest,
    pause::PauseRequest,
    play::PlayRequest,
    delete::DeleteRequest
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientRequest {
    Start(StartRequest),
    Info(InfoRequest),
    Stop(StopRequest),
    Lap(LapRequest),
    Pause(PauseRequest),
    Play(PlayRequest),
    Delete(DeleteRequest),
    #[default] Default
}

impl Into<ClientMessage> for ClientRequest {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientMessage {
    pub pid: u32,
    pub request: ClientRequest
}

impl ClientMessage {
    pub fn create(request: ClientRequest) -> Self {
        let pid = process::id();
        Self { pid, request }
    }
}

impl Codecable<'_> for ClientMessage { }

impl TryFrom<&[u8]> for ClientMessage {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl TryInto<Vec<u8>> for ClientMessage {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        communication::{
            start::StartRequest,
            client_message::ClientRequest
        },
        traits::Codecable, models::stopwatch::Name
    };

    use super::ClientMessage;

    #[test]
    fn test_cycle_0() {
        let request = ClientRequest::Start(StartRequest {
            name: Name::default(),
            verbose: false
        });
        let cm = ClientMessage {
            pid: 100,
            request
        };

        let encoded = cm.to_bytes().unwrap();
        let decoded = ClientMessage::from_bytes(&encoded).unwrap();

        assert_eq!(cm, decoded);
    }

    #[test]
    fn test_cycle_1() {
        let request = ClientRequest::Start(StartRequest {
            name: Name::new("random"),
            verbose: true
        });
        let cm = ClientMessage {
            pid: 0x87654321,
            request
        };

        let encoded = cm.to_bytes().unwrap();
        let decoded = ClientMessage::from_bytes(&encoded).unwrap();

        assert_eq!(cm, decoded);
    }
}