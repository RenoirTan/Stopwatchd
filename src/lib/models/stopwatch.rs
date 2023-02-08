use std::time::Duration;

use uuid::Uuid;

use crate::models::lap::Lap;

pub const NAME_LEN: usize = 6;
pub type Name = [u8; NAME_LEN];

pub const MIN_LAPS_CAPACITY: usize = 4;

#[derive(Debug)]
pub struct Stopwatch {
    pub id: Uuid,
    pub name: Option<Name>,
    laps: Vec<Lap>,
    pub ended: bool
}

impl Stopwatch {
    pub fn new_standby(name: Option<Name>) -> Self {
        let id = Uuid::new_v4();
        let lap = Lap::new_standby(id);
        let laps = vec![lap];
        let ended = false;
        Self { id, name, laps, ended }
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
        if self.ended {
            return false;
        }
        let last_lap = self.laps.last_mut().unwrap(); // assert at least one lap
        last_lap.play()
    }

    /// Pauses the stopwatch.
    /// 
    /// If stopwatch was playing, true is returned.
    /// False is returned if the stopwatch wasn't playing (as a warning).
    pub fn pause(&mut self) -> bool {
        if self.ended {
            return false;
        }
        let last_lap = self.laps.last_mut().unwrap(); // assert at least one lap
        last_lap.pause()
    }

    pub fn lap(&mut self) -> bool {
        self.laps.last_mut().unwrap().end();
        self.laps.push(Lap::start_immediately(self.id));
        true
    }

    pub fn end(&mut self) -> bool {
        if self.ended {
            true
        } else {
            let last_lap = self.laps.last_mut().unwrap();
            last_lap.end()
        }
    }

    pub fn total_time(&self) -> Duration {
        self.laps.iter()
            .fold(Duration::new(0, 0), |total, lap| total + lap.total_time())
    }

    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Stopwatch ID: {}\n", self.id));
        report.push_str(&format!("    Laps: {}\n", self.laps.len()));
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
    stopwatch.lap();
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