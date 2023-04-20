//! Info on a [`Stopwatch`] passed to the client.

use std::time::{SystemTime, Duration};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    models::{
        stopwatch::{Name, State, Stopwatch},
        lap::FinishedLap
    },
    traits::Codecable,
    identifiers::{UuidName, Identifier}
};

/// Details about a [`Stopwatch`]. See the methods and fields to see what
/// details exist.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopwatchDetails {
    pub sw_id: Uuid,
    pub name: Name,
    pub state: State,
    pub start_time: Option<SystemTime>,
    pub total_time: Duration,
    laps_count: usize,
    current_lap_time: Duration,
    pub verbose_info: Option<VerboseDetails>
}

impl StopwatchDetails {
    /// Extract details from a [`Stopwatch`]. Optionally include `verbose` info.
    pub fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name.clone();
        let state = stopwatch.state();
        let start_time = stopwatch.start_time();
        let total_time = stopwatch.total_time();
        let laps_count = stopwatch.laps();
        let current_lap_time = stopwatch.last_lap().unwrap().duration;
        let verbose_info = if verbose {
            Some(VerboseDetails::from_stopwatch(stopwatch))
        } else {
            None
        };
        Self {
            sw_id,
            name,
            state,
            start_time,
            total_time,
            laps_count,
            current_lap_time,
            verbose_info
        }
    }

    /// Create a collection of [`StopwatchDetails`] from an [`Iterator`] of
    /// [`Stopwatch`]s.
    pub fn from_iter<'s, I, D>(iter: I, verbose: bool) -> D
    where
        I: Iterator<Item = &'s Stopwatch>,
        D: FromIterator<Self>
    {
        iter.map(|s| StopwatchDetails::from_stopwatch(&s, verbose))
            .collect()
    }

    /// Number of laps in the stopwatch.
    pub fn laps_count(&self) -> usize {
        match &self.verbose_info {
            Some(vi) => vi.laps.len(),
            None => self.laps_count
        }
    }

    /// Time elapsed for the current lap.
    pub fn current_lap_time(&self) -> Duration {
        match &self.verbose_info {
            Some(vi) => vi.laps.last().unwrap().duration,
            None => self.current_lap_time
        }
    }

    /// Obtain [`UuidName`].
    pub fn get_uuid_name(&self) -> UuidName {
        UuidName {
            id: self.sw_id,
            name: self.name.clone()
        }
    }

    /// Get a string that this stopwatch can be identified by.
    pub fn get_identifier(&self) -> Identifier {
        self.get_uuid_name().as_identifier()
    }
}

impl Codecable<'_> for StopwatchDetails { }

/// Extra information, supplements [`StopwatchDetails`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerboseDetails {
    pub laps: Vec<FinishedLap>
}

impl VerboseDetails {
    /// Create [`VerboseDetails`] from a [`Stopwatch`].
    pub fn from_stopwatch(stopwatch: &Stopwatch) -> Self {
        let laps = stopwatch.all_laps();
        Self { laps }
    }
}

#[cfg(test)]
mod test {
    use crate::models::stopwatch::{Stopwatch, Name};

    use super::StopwatchDetails;

    fn make_stopwatch() -> Stopwatch {
        let mut stopwatch = Stopwatch::start(Some(Name::new("aaa")));
        stopwatch.new_lap(true);
        stopwatch.pause();
        stopwatch
    }

    fn basic_asserts(stopwatch: &Stopwatch, info: &StopwatchDetails) {
        assert_eq!(stopwatch.id, info.sw_id);
        assert_eq!(stopwatch.name, info.name);
        assert_eq!(stopwatch.state(), info.state);
        assert_eq!(stopwatch.start_time(), info.start_time);
        assert_eq!(stopwatch.total_time(), info.total_time);
        assert_eq!(stopwatch.laps(), info.laps_count());
    }

    #[test]
    fn test_from_stopwatch() {
        let stopwatch = make_stopwatch();
        let info = StopwatchDetails::from_stopwatch(&stopwatch, false);
        basic_asserts(&stopwatch, &info);
        assert_eq!(info.verbose_info, None);
    }

    #[test]
    fn test_from_stopwatch_verbose() {
        let stopwatch = make_stopwatch();
        let info = StopwatchDetails::from_stopwatch(&stopwatch, true);
        basic_asserts(&stopwatch, &info);
        assert!(matches!(info.verbose_info, Some(_)));
    }
}