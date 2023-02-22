use std::time::{SystemTime, Duration};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::{
        stopwatch::{Name, State, Stopwatch},
        lap::FinishedLap
    }
};

use super::{
    client_message::{ClientRequest, ClientMessage},
    server_message::{ServerReply, ServerMessage}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientInfoStopwatch {
    pub identifier: String,
    pub verbose: bool
}

impl Codecable<'_> for ClientInfoStopwatch { }

impl Into<ClientRequest> for ClientInfoStopwatch {
    fn into(self) -> ClientRequest {
        ClientRequest::Info(self)
    }
}

impl Into<ClientMessage> for ClientInfoStopwatch {
    fn into(self) -> ClientMessage {
        ClientMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfoStopwatch {
    pub sw_id: Uuid,
    pub name: Name,
    pub state: State,
    pub start_time: Option<SystemTime>,
    pub total_time: Duration,
    laps_count: usize,
    pub verbose_info: Option<ServerInfoStopwatchVerbose>
}

impl ServerInfoStopwatch {
    pub fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name.clone();
        let state = stopwatch.state();
        let start_time = stopwatch.start_time();
        let total_time = stopwatch.total_time();
        let laps_count = stopwatch.laps();
        let verbose_info = if verbose {
            Some(ServerInfoStopwatchVerbose::from_stopwatch(stopwatch))
        } else {
            None
        };
        Self {
            sw_id,
            name,
            state,
            start_time,
            total_time,
            laps_count,
            verbose_info
        }
    }

    pub fn laps_count(&self) -> usize {
        match &self.verbose_info {
            Some(vi) => vi.laps.len(),
            None => self.laps_count
        }
    }
}

impl Codecable<'_> for ServerInfoStopwatch { }

impl Into<ServerReply> for ServerInfoStopwatch {
    fn into(self) -> ServerReply {
        ServerReply::Info(self)
    }
}

impl Into<ServerMessage> for ServerInfoStopwatch {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfoStopwatchVerbose {
    pub laps: Vec<FinishedLap>
}

impl ServerInfoStopwatchVerbose {
    pub fn from_stopwatch(stopwatch: &Stopwatch) -> Self {
        let laps = stopwatch.all_laps();
        Self { laps }
    }
}

#[cfg(test)]
mod test {
    use crate::models::stopwatch::{Stopwatch, Name};

    use super::ServerInfoStopwatch;

    fn make_stopwatch() -> Stopwatch {
        let mut stopwatch = Stopwatch::start(Some(Name::new("aaa")));
        stopwatch.new_lap(true);
        stopwatch.pause();
        stopwatch
    }

    fn basic_asserts(stopwatch: &Stopwatch, info: &ServerInfoStopwatch) {
        assert_eq!(stopwatch.id, info.sw_id);
        assert_eq!(stopwatch.name, info.name);
        assert_eq!(stopwatch.state(), info.state);
        assert_eq!(stopwatch.start_time(), info.start_time);
        assert_eq!(stopwatch.total_time(), info.total_time);
        assert_eq!(stopwatch.laps(), info.laps_count());
    }

    #[test]
    fn test_from_stopwatch() {
        let stopwatch = make_stopwatch();
        let info = ServerInfoStopwatch::from_stopwatch(&stopwatch, false);
        basic_asserts(&stopwatch, &info);
        assert_eq!(info.verbose_info, None);
    }

    #[test]
    fn test_from_stopwatch_verbose() {
        let stopwatch = make_stopwatch();
        let info = ServerInfoStopwatch::from_stopwatch(&stopwatch, true);
        basic_asserts(&stopwatch, &info);
        assert!(matches!(info.verbose_info, Some(_)));
    }
}