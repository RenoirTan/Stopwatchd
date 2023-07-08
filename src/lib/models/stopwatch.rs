//! Stopwatch.

use std::{
    time::{Duration, SystemTime},
    fmt
};

use serde::{Serialize, Deserialize};

use crate::identifiers::{Identifier, UniqueId, Name};

use super::lap::{CurrentLap, FinishedLap};

/// Minimum default capacity for lists storing laps.
pub const MIN_LAPS_CAPACITY: usize = 4;

/// What the [`Stopwatch`] is doing.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Playing,
    Paused,
    Ended
}

impl State {
    pub fn playing(&self) -> bool {
        matches!(self, Self::Playing)
    }

    pub fn paused(&self) -> bool {
        matches!(self, Self::Paused)
    }

    pub fn ended(&self) -> bool {
        matches!(self, Self::Ended)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use State::*;
        write!(f, "{}", match self {
            Playing => "playing",
            Paused => "paused",
            Ended => "ended"
        })
    }
}

/// Represents a stopwatch, with laps and an API to pause and play.
#[derive(Debug)]
pub struct Stopwatch {
    pub identifier: Identifier,
    finished_laps: Vec<FinishedLap>,
    current_lap: Option<CurrentLap> // If some, not yet ended
}

impl Stopwatch {
    /// New stopwatch with an optional name. The stopwatch is paused by default.
    pub fn new<N: Into<Name>>(name: N) -> Self {
        let id = UniqueId::generate();
        let identifier = Identifier::new(id, name.into());
        let finished_laps = Vec::new();
        let current_lap = Some(CurrentLap::new(id));
        Self { identifier, finished_laps, current_lap }
    }

    /// New stopwatch but start immediately.
    pub fn start<N: Into<Name>>(name: N) -> Self {
        let mut sw = Self::new(name);
        sw.play();
        sw
    }

    /// Starts the stopwatch.
    pub fn play(&mut self) -> State {
        if let Some(ref mut lap) = self.current_lap {
            let state = lap.state();
            lap.play();
            state
        } else {
            State::Ended
        }
    }

    /// Pauses the stopwatch.
    pub fn pause(&mut self) -> State {
        if let Some(ref mut lap) = self.current_lap {
            let state = lap.state();
            lap.pause();
            state
        } else {
            State::Ended
        }
    }

    /// Stop the current lap and create a new lap.
    pub fn new_lap(&mut self, start_immediately: bool) -> State {
        match self.current_lap.take() {
            Some(prev_lap) => {
                if start_immediately {
                    self.current_lap = Some(CurrentLap::start(self.identifier.id));
                } else {
                    self.current_lap = Some(CurrentLap::new(self.identifier.id));
                }
                self.finished_laps.push(prev_lap.end());
                if start_immediately {
                    State::Playing
                } else {
                    State::Paused
                }
            },
            None => State::Ended
        }
    }

    /// Count the number of laps, including the current one.
    pub fn laps(&self) -> usize {
        self.finished_laps.len() + if self.current_lap.is_some() { 1 } else { 0 }
    }

    /// Get a copy first lap.
    pub fn first_lap(&self) -> Option<FinishedLap> {
        match self.finished_laps.first() {
            Some(lap) => Some(lap.clone()),
            None => self.current_lap.as_ref().map(|l| l.normalize())
        }
    }

    /// Get a copy of the last lap.
    pub fn last_lap(&self) -> Option<FinishedLap> {
        match self.current_lap.as_ref() {
            Some(lap) => Some(lap.normalize()),
            None => self.finished_laps.last().map(|l| l.clone())
        }
    }

    /// List of already finished lap. This excludes the current lap if it hasn't
    /// ended yet.
    pub fn finished_laps(&self) -> &[FinishedLap] {
        &self.finished_laps
    }

    /// Get the current lap if it's running.
    pub fn current_lap(&self) -> Option<&CurrentLap> {
        self.current_lap.as_ref()
    }

    /// Get a copy of all laps.
    pub fn all_laps(&self) -> Vec<FinishedLap> {
        let mut laps = self.finished_laps.clone();
        if let Some(cur) = &self.current_lap {
            laps.push(cur.normalize());
        }
        laps
    }

    /// Terminate this stopwatch such that it cannot be played again.
    pub fn end(&mut self) -> State {
        if let Some(prev_lap) = self.current_lap.take() {
            let state = prev_lap.state();
            self.finished_laps.push(prev_lap.end());
            state
        } else {
            State::Ended
        }
    }

    /// Get the current [`State`] of the stopwatch.
    pub fn state(&self) -> State {
        match &self.current_lap {
            Some(lap) => lap.state(),
            None => State::Ended
        }
    }

    /// When the stopwatch was created.
    pub fn start_time(&self) -> Option<SystemTime> {
        self.first_lap().map(|l| l.start)
    }

    /// How much time the stopwatch was playing.
    pub fn total_time(&self) -> Duration {
        let total = match &self.current_lap {
            Some(lap) => lap.total_time(),
            None => Duration::new(0, 0)
        };
        self.finished_laps.iter()
            .fold(total, |total, lap| total + lap.duration)
    }

    /// Details about this stopwatch.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Stopwatch ID: {}\n", self.identifier.id));
        report.push_str(&format!("    State: {:?}\n", self.state()));
        report.push_str(&format!("    Laps: {}\n", self.laps()));
        report.push_str(&format!("    Duration: {} ms", self.total_time().as_millis()));
        report
    }
}

/*
pub fn _simulate_stopwatch(duration: Duration) {
    debug!("_simulating stopwatch");
    let mut stopwatch = Stopwatch::new(None);
    println!("{}", stopwatch.report());
    stopwatch.play();
    std::thread::sleep(duration);
    stopwatch.pause();
    println!("{}", stopwatch.report());
    stopwatch.play();
    std::thread::sleep(duration);
    println!("{}", stopwatch.report());
    stopwatch.new_lap(true);
    println!("{}", stopwatch.report());
    std::thread::sleep(duration);
    stopwatch.pause();
    println!("{}", stopwatch.report());
    stopwatch.play();
    std::thread::sleep(duration);
    stopwatch.end();
    println!("{}", stopwatch.report());
    println!("Stopwatch {} done!", stopwatch.id);
    debug!("stopwatch _simulation done");
}
*/