use std::time::Duration;

use uuid::Uuid;

use crate::models::lap::CurrentLap;

use super::lap::FinishedLap;

pub const NAME_LEN: usize = 6;
pub type Name = [u8; NAME_LEN];

pub const MIN_LAPS_CAPACITY: usize = 4;

#[derive(Debug)]
pub struct Stopwatch {
    pub id: Uuid,
    pub name: Option<Name>,
    finished_laps: Vec<FinishedLap>,
    current_lap: Option<CurrentLap> // If some, not yet ended
}

impl Stopwatch {
    pub fn new_standby(name: Option<Name>) -> Self {
        let id = Uuid::new_v4();
        let finished_laps = Vec::new();
        let current_lap = Some(CurrentLap::new_standby(id));
        Self { id, name, finished_laps, current_lap }
    }

    pub fn start_immediately(name: Option<Name>) -> Self {
        let mut sw = Self::new_standby(name);
        sw.play();
        sw
    }

    /// Starts the stopwatch.
    /// 
    /// If stopwatch wasn't playing before, false is returned.
    /// True is returned if the stopwatch was playing (just as a warning).
    pub fn play(&mut self) -> bool {
        match &mut self.current_lap {
            Some(lap) => {
                lap.play()
            },
            None => false
        }
    }

    /// Pauses the stopwatch.
    /// 
    /// If stopwatch was playing, true is returned.
    /// False is returned if the stopwatch wasn't playing (as a warning).
    pub fn pause(&mut self) -> bool {
        match &mut self.current_lap {
            Some(lap) => {
                lap.pause()
            },
            None => false
        }
    }

    pub fn new_lap(&mut self) -> bool {
        match self.current_lap.take() {
            Some(prev_lap) => {
                self.current_lap = Some(CurrentLap::start_immediately(self.id));
                self.finished_laps.push(prev_lap.end());
                true
            },
            None => false
        }
    }

    pub fn laps(&self) -> usize {
        self.finished_laps.len() + if self.current_lap.is_some() { 1 } else { 0 }
    }

    pub fn end(&mut self) -> bool {
        match self.current_lap.take() {
            Some(prev_lap) => {
                self.finished_laps.push(prev_lap.end());
                false
            },
            None => true
        }
    }

    pub fn has_ended(&self) -> bool {
        self.current_lap.is_none()
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
        report.push_str(&format!("    Laps: {}\n", self.laps()));
        report.push_str(&format!("    Duration: {} ms", self.total_time().as_millis()));
        report
    }
}

pub fn _simulate_stopwatch(duration: Duration) {
    debug!("_simulating stopwatch");
    let mut stopwatch = Stopwatch::new_standby(None);
    println!("{}", stopwatch.report());
    stopwatch.play();
    std::thread::sleep(duration);
    stopwatch.pause();
    println!("{}", stopwatch.report());
    stopwatch.play();
    std::thread::sleep(duration);
    println!("{}", stopwatch.report());
    stopwatch.new_lap();
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