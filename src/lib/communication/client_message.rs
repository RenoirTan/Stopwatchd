use std::{process, io};

use serde::{Serialize, Deserialize};

use crate::{traits::Codecable, identifiers::Identifier};

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
pub enum ClientRequestKind {
    Start(StartRequest),
    Info(InfoRequest),
    Stop(StopRequest),
    Lap(LapRequest),
    Pause(PauseRequest),
    Play(PlayRequest),
    Delete(DeleteRequest),
    #[default] Default
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRequest {
    pub identifiers: Vec<Identifier>,
    pub verbose: bool,
    pub specific_args: ClientRequestKind
}

impl ClientRequest {
    pub fn new<I, T>(identifiers: I, verbose: bool, specific_args: ClientRequestKind) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Identifier>
    {
        Self {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
            verbose,
            specific_args
        }
    }
}

impl Codecable<'_> for ClientRequest { }

impl Default for ClientRequest {
    fn default() -> Self {
        Self {
            identifiers: vec![],
            verbose: false,
            specific_args: Default::default()
        }
    }
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
            client_message::{ClientRequestKind, ClientRequest}
        },
        traits::Codecable, models::stopwatch::Name
    };

    use super::ClientMessage;

    #[test]
    fn test_cycle_0() {
        let specific = ClientRequestKind::Start(StartRequest);
        let request = ClientRequest::new([Name::default()], false, specific);
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
        let specific = ClientRequestKind::Start(StartRequest);
        let request = ClientRequest::new(["random"], true, specific);
        let cm = ClientMessage {
            pid: 0x87654321,
            request
        };

        let encoded = cm.to_bytes().unwrap();
        let decoded = ClientMessage::from_bytes(&encoded).unwrap();

        assert_eq!(cm, decoded);
    }
}