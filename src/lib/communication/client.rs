//! Messages passed from clients to `swd` server.

use serde::{Serialize, Deserialize};

use crate::identifiers::Identifier;

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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestKind {
    Start(StartRequest),
    Info(InfoRequest),
    Stop(StopRequest),
    Lap(LapRequest),
    Pause(PauseRequest),
    Play(PlayRequest),
    Delete(DeleteRequest)
}

/// A request from a client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// List of stopwatches that the requested action specified by `specific_args` is meant to
    /// apply to.
    pub identifiers: Vec<Identifier>,
    /// Return verbose output.
    pub verbose: bool,
    /// Type of request.
    pub specific_args: RequestKind
}

impl Request {
    /// Create a new [`ClientRequest`].
    /// 
    /// # Arguments
    /// * identifiers - List of [`Identifier`]s (or an iterator that generates identifiers).
    /// * verbose - Whether to return verbose output, may have adverse effects on the performance
    ///     of swd as it has to process and send more data.
    /// * specific_args - Type of request.
    pub fn new<I, T>(identifiers: I, verbose: bool, specific_args: RequestKind) -> Self
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

#[cfg(test)]
mod test {
    use crate::{
        communication::{
            start::StartRequest,
            client::{RequestKind, Request}
        },
        traits::Codecable,
        models::stopwatch::Name
    };

    #[test]
    fn test_cycle_0() {
        let specific = RequestKind::Start(StartRequest);
        let request = Request::new([Name::default()], false, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }

    #[test]
    fn test_cycle_1() {
        let specific = RequestKind::Start(StartRequest);
        let request = Request::new(["random"], true, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }
}