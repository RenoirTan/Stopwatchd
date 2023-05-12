//! Messages passed from clients to `swd` server.

use serde::{Serialize, Deserialize};

use crate::{identifiers::Identifier, util::iter_into_vec};

use super::{
    start::StartRequest,
    info::InfoRequest,
    stop::StopRequest,
    lap::LapRequest,
    pause::PauseRequest,
    play::PlayRequest,
    delete::DeleteRequest
};

/// Common arguments for requests.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonArgs {
    /// List of stopwatches the specified action should be applied to.
    pub identifiers: Vec<Identifier>,
    /// Whether to return verbose/more detailed information.
    pub verbose: bool
}

impl CommonArgs {
    /// Create a new [`CommonArgs`] object.
    pub fn new(identifiers: Vec<Identifier>, verbose: bool) -> Self {
        Self { identifiers, verbose }
    }

    pub fn from_iter<I, T>(identifiers: I, verbose: bool) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Identifier>
    {
        Self::new(iter_into_vec(identifiers), verbose)
    }
}

impl Default for CommonArgs {
    fn default() -> Self {
        Self { identifiers: vec![], verbose: false }
    }
}

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
    /// Common arguments (not specific to one or a few [`RequestKind`]).
    pub common_args: CommonArgs,
    /// Type of request.
    pub specific_args: RequestKind
}

impl Request {
    /// Create a new [`Request`].
    pub fn new(common_args: CommonArgs, specific_args: RequestKind) -> Self {
        Self { common_args, specific_args }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        communication::{
            start::StartRequest,
            client::{RequestKind, Request, CommonArgs}
        },
        traits::Codecable,
        models::stopwatch::Name
    };

    #[test]
    fn test_cycle_0() {
        let specific = RequestKind::Start(StartRequest);
        let common = CommonArgs::from_iter([Name::default()], false);
        let request = Request::new(common, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }

    #[test]
    fn test_cycle_1() {
        let specific = RequestKind::Start(StartRequest);
        let common = CommonArgs::from_iter(["random"], false);
        let request = Request::new(common, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }
}