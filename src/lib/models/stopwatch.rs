use std::time::{Duration, SystemTime};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::lap::{CurrentLap, FinishedLap};

pub const NAME_LEN: usize = 6;
pub type Name = [u8; NAME_LEN];

pub fn truncated_name_from_bytes(name: &[u8]) -> Name {
    let name_len = name.len();
    let mut output = [0, 0, 0, 0, 0, 0];
    for i in 0..NAME_LEN {
        if i < name_len {
            output[i] = name[i];
        }
    }
    output
}

pub fn truncated_name_from_str<S: AsRef<str>>(name: S) -> Name {
    truncated_name_from_bytes(name.as_ref().as_bytes())
}

pub fn name_from_str<S: AsRef<str>>(name: S) -> Result<Name, usize> {
    let name = name.as_ref().as_bytes();
    let name_len = name.len();
    if name_len > NAME_LEN {
        return Err(name_len);
    }

    Ok(truncated_name_from_bytes(name))
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

#[derive(Debug)]
pub struct Stopwatch {
    pub id: Uuid,
    pub name: Option<Name>,
    finished_laps: Vec<FinishedLap>,
    current_lap: Option<CurrentLap> // If some, not yet ended
}

impl Stopwatch {
    pub fn new(name: Option<Name>) -> Self {
        let id = Uuid::new_v4();
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