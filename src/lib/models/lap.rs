use std::time::{SystemTime, Instant, Duration};

use uuid::Uuid;

#[derive(Debug)]
pub struct CurrentLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    pub timer: Instant,
    pub duration: Duration,
    pub playing: bool
}

impl CurrentLap {
    pub fn new_standby(sw_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let start = SystemTime::now();
        let timer = Instant::now();
        let duration = Duration::new(0, 0);
        let playing = false;
        Self { id, sw_id, start, timer, duration, playing }
    }

    pub fn start_immediately(sw_id: Uuid) -> Self {
        let mut lap = Self::new_standby(sw_id);
        lap.play();
        lap
    }

    pub fn play(&mut self) -> bool {
        if self.playing {
            true
        } else {
            self.timer = Instant::now();
            self.playing = true;
            false
        }
    }

    pub fn pause(&mut self) -> bool {
        if !self.playing {
            false
        } else {
            self.duration += self.timer.elapsed();
            self.playing = false;
            true
        }
    }

    pub fn end(self) -> FinishedLap {
        self.into()
    }

    pub fn total_time(&self) -> Duration {
        if self.playing {
            self.duration + self.timer.elapsed()
        } else {
            self.duration
        }
    }
}

impl Into<FinishedLap> for CurrentLap {
    fn into(self) -> FinishedLap {
        let duration = self.total_time();
        FinishedLap { id: self.id, sw_id: self.sw_id, start: self.start, duration }
    }
}

#[derive(Debug)]
pub struct FinishedLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    pub duration: Duration
}
