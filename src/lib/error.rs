//! Custom errors for Stopwatchd.

use std::fmt;

use serde::{Serialize, Deserialize};

use crate::{identifiers::{Identifier, UuidName}, models::stopwatch::State};
#[allow(unused)]
use crate::models::stopwatch::Stopwatch;

/// No matches or multiple conflicting matches for an [`Identifier`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindStopwatchError {
    pub identifier: Identifier,
    pub duplicates: Vec<UuidName>
}

impl FindStopwatchError {
    /// Short error message.
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

    /// More detailed error message.
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

impl std::error::Error for FindStopwatchError { }

/// Action does not apply for a [`Stopwatch`] because it's in the wrong
/// [`State`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidState {
    pub identifier: Identifier,
    pub state: State
}

impl fmt::Display for InvalidState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is currently {}", self.identifier, self.state)
    }
}

impl std::error::Error for InvalidState { }