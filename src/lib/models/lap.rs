use std::time::{SystemTime, Instant, Duration};

use uuid::Uuid;

#[derive(Debug)]
pub struct CurrentLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    timer: Option<Instant>,
    pub duration: Duration
}

impl CurrentLap {
    pub fn new(sw_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let start = SystemTime::now();
        let timer = None;
        let duration = Duration::new(0, 0);
        Self { id, sw_id, start, timer, duration }
    }

    pub fn start(sw_id: Uuid) -> Self {
        let mut lap = Self::new(sw_id);
        lap.play();
        lap
    }

    pub fn play(&mut self) {
        if let None = self.timer {
            self.timer = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if let Some(timer) = self.timer.take() {
            self.duration += timer.elapsed();
        }
    }

    pub fn playing(&self) -> bool {
        self.timer.is_some()
    }

    pub fn end(self) -> FinishedLap {
        self.into()
    }

    pub fn normalize(&self) -> FinishedLap {
        FinishedLap {
            id: self.id,
            sw_id: self.sw_id,
            start: self.start,
            duration: self.total_time()
        }
    }

    pub fn total_time(&self) -> Duration {
        if let Some(ref timer) = self.timer {
            self.duration + timer.elapsed()
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

#[derive(Clone, Debug)]
pub struct FinishedLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    pub duration: Duration
}
