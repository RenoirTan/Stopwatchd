use std::time::{SystemTime, Duration, Instant};

use uuid::Uuid;

pub const NAME_LEN: usize = 6;
pub type Name = [u8; NAME_LEN];

#[derive(Debug)]
pub struct Stopwatch {
    pub id: Uuid,
    pub name: Option<Name>,
    pub start: SystemTime,
    pub timer: Instant,
    pub duration: Duration,
    pub playing: bool
}

impl Stopwatch {
    pub fn new_standby(name: Option<Name>) -> Self {
        let id = Uuid::new_v4();
        let start = SystemTime::now();
        let timer = Instant::now();
        let duration = Duration::new(0, 0);
        let playing = false;
        Self { id, name, start, timer, duration, playing }
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
        if !self.playing {
            self.timer = Instant::now();
            self.playing = true;
            false
        } else {
            true
        }
    }

    /// Pauses the stopwatch.
    /// 
    /// If stopwatch was playing, true is returned.
    /// False is returned if the stopwatch wasn't playing (as a warning).
    pub fn pause(&mut self) -> bool {
        if self.playing {
            self.duration += self.timer.elapsed();
            self.playing = false;
            true
        } else {
            false
        }
    }
}

pub fn _simulate_stopwatch(duration: Duration) {
    debug!("_simulating stopwatch");
    let mut stopwatch = Stopwatch::new_standby(None);
    println!("{:?}", stopwatch);
    stopwatch.play();
    std::thread::sleep(duration);
    stopwatch.pause();
    println!("{:?}", stopwatch);
    debug!("stopwatch _simulation done");
}