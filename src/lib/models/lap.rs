//! Laps, you know what they are.

use std::time::{SystemTime, Instant, Duration};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[allow(unused)]
use super::stopwatch::Stopwatch;

/// The active lap that is ticking.
#[derive(Debug)]
pub struct CurrentLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    timer: Option<Instant>,
    pub duration: Duration
}

impl CurrentLap {
    /// Create a new lap.
    /// 
    /// # Arguments
    /// * `sw_id` - [`Uuid`] of the [`Stopwatch`] that will own this lap.
    pub fn new(sw_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let start = SystemTime::now();
        let timer = None;
        let duration = Duration::new(0, 0);
        Self { id, sw_id, start, timer, duration }
    }

    /// Start the timer on this lap.
    pub fn start(sw_id: Uuid) -> Self {
        let mut lap = Self::new(sw_id);
        lap.play();
        lap
    }

    /// Continue playing the lap after pausing it.
    pub fn play(&mut self) {
        if let None = self.timer {
            self.timer = Some(Instant::now());
        }
    }

    /// Temporarily stop the timer.
    pub fn pause(&mut self) {
        if let Some(timer) = self.timer.take() {
            self.duration += timer.elapsed();
        }
    }

    /// Check if the current lap is playing.
    pub fn playing(&self) -> bool {
        self.timer.is_some()
    }

    /// Permannently end the current lap, transforming it into a
    /// [`FinishedLap`].
    pub fn end(self) -> FinishedLap {
        self.into()
    }

    /// Create a copy of a [`FinishedLap`].
    pub fn normalize(&self) -> FinishedLap {
        FinishedLap {
            id: self.id,
            sw_id: self.sw_id,
            start: self.start,
            duration: self.total_time()
        }
    }

    /// The total time this lap has been running for.
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

/// A [`FinishedLap`] cannot be played or paused.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinishedLap {
    pub id: Uuid,
    pub sw_id: Uuid,
    pub start: SystemTime,
    pub duration: Duration
}
