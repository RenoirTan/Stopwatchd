//! Messages passed from clients to `swd` server.

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

/// Possible actions requested by the client.
/// 
/// See the respective .*Request structs for more information.
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

/// A request from a client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRequest {
    /// List of stopwatches that the requested action specified by `specific_args` is meant to
    /// apply to.
    pub identifiers: Vec<Identifier>,
    /// Return verbose output.
    pub verbose: bool,
    /// Type of request.
    pub specific_args: ClientRequestKind
}

impl ClientRequest {
    /// Create a new [`ClientRequest`].
    /// 
    /// # Arguments
    /// * identifiers - List of [`Identifier`]s (or an iterator that generates identifiers).
    /// * verbose - Whether to return verbose output, may have adverse effects on the performance
    ///     of swd as it has to process and send more data.
    /// * specific_args - Type of request.
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

/// Message from a client.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientMessage {
    /// PID of client.
    pub pid: u32,
    /// Requested action.
    pub request: ClientRequest
}

impl ClientMessage {
    /// Create a [`ClientMessage`] from [`ClientRequest`].
    /// The PID will be filled in automatically.
    pub fn create(request: ClientRequest) -> Self {
        let pid = process::id();
        Self { pid, request }
    }
}

impl TryFrom<&[u8]> for ClientMessage {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Codecable::from_bytes(&value)
    }
}

impl TryInto<Vec<u8>> for ClientMessage {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Codecable::to_bytes(&self)
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