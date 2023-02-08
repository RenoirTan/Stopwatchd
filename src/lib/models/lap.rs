use std::time::{SystemTime, Instant, Duration};

use uuid::Uuid;

#[derive(Debug)]
pub struct Lap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    pub timer: Instant,
    pub duration: Duration,
    pub playing: bool,
    pub ended: bool
}

impl Lap {
    pub fn new_standby(sw_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let start = SystemTime::now();
        let timer = Instant::now();
        let duration = Duration::new(0, 0);
        let playing = false;
        let ended = false;
        Self { id, sw_id, start, timer, duration, playing, ended }
    }

    pub fn start_immediately(sw_id: Uuid) -> Self {
        let mut lap = Self::new_standby(sw_id);
        lap.play();
        lap
    }

    pub fn play(&mut self) -> bool {
        if self.playing || self.ended {
            true
        } else {
            self.timer = Instant::now();
            self.playing = true;
            false
        }
    }

    pub fn pause(&mut self) -> bool {
        if !self.playing || self.ended {
            false
        } else {
            self.duration += self.timer.elapsed();
            self.playing = false;
            true
        }
    }

    pub fn end(&mut self) -> bool {
        if self.ended {
            true
        } else {
            self.pause();
            self.ended = true;
            false
        }
    }

    pub fn total_time(&self) -> Duration {
        if self.playing {
            self.duration + self.timer.elapsed()
        } else {
            self.duration
        }
    }
}