use std::{
    time::{Duration, SystemTime},
    ops::Deref, error, fmt
};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::identifiers::{UNMatchKind, UuidName, Identifier};

use super::lap::{CurrentLap, FinishedLap};


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    #[inline]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }

    #[inline]
    pub fn empty() -> Self {
        Self("".to_string())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl Default for Name {
    #[inline]
    fn default() -> Self {
        Self("".to_string())
    }
}

impl<S: Into<String>> From<Option<S>> for Name {
    #[inline]
    fn from(name: Option<S>) -> Self {
        Self(match name {
            Some(n) => n.into(),
            None => String::new()
        })
    }
}

impl Deref for Name {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

pub const MIN_LAPS_CAPACITY: usize = 4;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindStopwatchError {
    pub identifier: Identifier,
    pub duplicates: Vec<UuidName>
}

impl FindStopwatchError {
    pub fn summarize(&self) -> String {
        let duplicates_len = self.duplicates.len();
        if duplicates_len == 0 {
            format!("no stopwatch was found with identifier: {}", self.identifier)
        } else {
            format!(
                "{} stopwatches were found with identifier: {}",
                duplicates_len,
                self.identifier
            )
        }
    }

    pub fn diagnose(&self) -> String {
        let mut diagnosis = self.summarize();
        if self.duplicates.len() == 0 {
            diagnosis
        } else {
            diagnosis += "\n";
            for uuid_name in &self.duplicates {
                let uuid = uuid_name.id;
                let name = &uuid_name.name;
                diagnosis += &format!("    Uuid: {:?} Name: {:?}", uuid, name);
            }
            diagnosis
        }
    }
}

impl fmt::Display for FindStopwatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

impl error::Error for FindStopwatchError { }

#[derive(Debug)]
pub struct Stopwatch {
    pub id: Uuid,
    pub name: Name,
    finished_laps: Vec<FinishedLap>,
    current_lap: Option<CurrentLap> // If some, not yet ended
}

impl Stopwatch {
    pub fn new(name: Option<Name>) -> Self {
        let id = Uuid::new_v4();
        let name = name.unwrap_or_default();
        let finished_laps = Vec::new();
        let current_lap = Some(CurrentLap::new(id));
        Self { id, name, finished_laps, current_lap }
    }

    pub fn start(name: Option<Name>) -> Self {
        let mut sw = Self::new(name);
        sw.play();
        sw
    }

    /// Starts the stopwatch.
    pub fn play(&mut self) {
        if let Some(ref mut lap) = self.current_lap {
            lap.play();
        }
    }

    /// Pauses the stopwatch.
    pub fn pause(&mut self) {
        if let Some(ref mut lap) = self.current_lap {
            lap.pause();
        }
    }

    pub fn new_lap(&mut self, start_immediately: bool) -> State {
        match self.current_lap.take() {
            Some(prev_lap) => {
                if start_immediately {
                    self.current_lap = Some(CurrentLap::start(self.id));
                } else {
                    self.current_lap = Some(CurrentLap::new(self.id));
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

    pub fn laps(&self) -> usize {
        self.finished_laps.len() + if self.current_lap.is_some() { 1 } else { 0 }
    }

    pub fn first_lap(&self) -> Option<FinishedLap> {
        match self.finished_laps.first() {
            Some(lap) => Some(lap.clone()),
            None => self.current_lap.as_ref().map(|l| l.normalize())
        }
    }

    pub fn matches_identifier(&self, identifier: &Identifier) -> Option<UNMatchKind> {
        self.get_uuid_name().matches(identifier)
    }

    pub fn get_uuid_name(&self) -> UuidName {
        UuidName { id: self.id, name: self.name.clone() }
    }

    pub fn last_lap(&self) -> Option<FinishedLap> {
        match self.current_lap.as_ref() {
            Some(lap) => Some(lap.normalize()),
            None => self.finished_laps.last().map(|l| l.clone())
        }
    }

    pub fn finished_laps(&self) -> &[FinishedLap] {
        &self.finished_laps
    }

    pub fn current_lap(&self) -> Option<&CurrentLap> {
        self.current_lap.as_ref()
    }

    pub fn all_laps(&self) -> Vec<FinishedLap> {
        let mut laps = self.finished_laps.clone();
        if let Some(cur) = &self.current_lap {
            laps.push(cur.normalize());
        }
        laps
    }

    pub fn end(&mut self) {
        if let Some(prev_lap) = self.current_lap.take() {
            self.finished_laps.push(prev_lap.end());
        }
    }

    pub fn state(&self) -> State {
        match &self.current_lap {
            Some(lap) => if lap.playing() {
                State::Playing
            } else {
                State::Paused
            },
            None => State::Ended
        }
    }

    pub fn start_time(&self) -> Option<SystemTime> {
        self.first_lap().map(|l| l.start)
    }

    pub fn total_time(&self) -> Duration {
        let total = match &self.current_lap {
            Some(lap) => lap.total_time(),
            None => Duration::new(0, 0)
        };
        self.finished_laps.iter()
            .fold(total, |total, lap| total + lap.duration)
    }

    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Stopwatch ID: {}\n", self.id));
        report.push_str(&format!("    State: {:?}\n", self.state()));
        report.push_str(&format!("    Laps: {}\n", self.laps()));
        report.push_str(&format!("    Duration: {} ms", self.total_time().as_millis()));
        report
    }
}

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