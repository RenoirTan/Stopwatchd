//! Messages from the `swd` server to the client.

use std::{fmt, collections::HashMap, hash::Hash};

use serde::{Serialize, Deserialize};

use crate::error::{FindStopwatchError, InvalidState, BadNameError};

use super::{details::StopwatchDetails, reply_specifics::SpecificAnswer};

/// Convert a mapping of [`StopwatchDetails`] into a [`HashMap`] of `V`.
/// 
/// # Arguments
/// * iter - Mapping of [`StopwatchDetails`]. The keys of `iter` get preserved in the [`HashMap`]
///     that gets returned. [`StopwatchDetails`] get converted to `V`.
pub fn details_map_into<I, K, V>(iter: I) -> HashMap<K, V>
where
    I: IntoIterator<Item = (K, StopwatchDetails)>,
    K: Hash + Eq,
    V: From<StopwatchDetails>
{
    iter.into_iter().map(|(k, v)| (k, From::from(v))).collect()
}

/// Possible errors emitted by `swd`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerError {
    FindStopwatchError(FindStopwatchError),
    InvalidState(InvalidState),
    BadName(BadNameError),
    Other(String)
}

impl ServerError {
    /// Raw stopwatch identifier that caused the error.
    pub fn get_raw_id(&self) -> Option<&str> {
        use ServerError::*;
        match self {
            FindStopwatchError(fse) => Some(&fse.raw_identifier),
            InvalidState(is) => Some(&is.raw_identifier),
            BadName(_) => None, // TODO: Should have identifier
            Other(_) => None
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ServerError::*;
        match self {
            FindStopwatchError(fse) => write!(f, "{}", fse.diagnose()),
            InvalidState(is) => write!(f, "{}", is),
            BadName(bne) => bne.fmt(f),
            Other(s) => write!(f, "{}", s)
        }
    }
}

impl std::error::Error for ServerError { }

impl From<FindStopwatchError> for ServerError {
    fn from(fse: FindStopwatchError) -> Self {
        Self::FindStopwatchError(fse)
    }
}

impl From<InvalidState> for ServerError {
    fn from(is: InvalidState) -> Self {
        Self::InvalidState(is)
    }
}

impl From<String> for ServerError {
    fn from(error: String) -> Self {
        Self::Other(error)
    }
}

/// Reply from the `swd` server.
/// Contains details on what happened to each `Stopwatch` after an action is carried
/// out by the server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reply {
    /// Successful messages. The [`Identifier`] of the relevant `Stopwatch` is included here.
    pub successful: HashMap<String, StopwatchDetails>,
    /// Error messages. The offending [`Identifier`]s may be in one of the keys.
    /// Errors that were not caused by a particular [`Identifier`] will be associated with the
    /// [`None`] key.
    pub errors: HashMap<Option<String>, Vec<ServerError>>,
    /// Type of action `swd` tried to take.
    pub specific_answer: SpecificAnswer
}

impl Reply {
    /// Create a new [`ServerReply`].
    pub fn new(specific_answer: SpecificAnswer) -> Self {
        let successful = HashMap::new();
        let errors = HashMap::new();
        Self { successful, errors, specific_answer }
    }

    /// Add success messages (i.e. [`StopwatchDetails`]). [`Identifier`]s will be gleaned from
    /// [`StopwatchDetails`].
    pub fn add_successful<I>(&mut self, successful: I)
    where
        I: IntoIterator<Item = StopwatchDetails>
    {
        self.extend_successful(successful.into_iter().map(|d| (d.get_raw_id(), d)))
    }

    /// Add a collection of [`StopwatchDetails`] mapped to their respective [`Identifier`]s.
    pub fn extend_successful<I, S>(&mut self, successful: I)
    where
        I: IntoIterator<Item = (S, StopwatchDetails)>,
        S: Into<String>
    {
        self.successful.extend(successful.into_iter().map(|(i, d)| (i.into(), d)));
    }

    /// Add error messages. [`Identifier`]s can be elicited from [`ServerError`].
    pub fn add_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = ServerError>
    {
        self.extend_uncollected_errors(
            errors.into_iter().map(|e| (e.get_raw_id().map(str::to_string), e))
        );
    }

    /// Add error messages with a specified [`Identifier`].
    pub fn extend_uncollected_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = (Option<String>, ServerError)>
    {
        for (identifier, error) in errors {
            match self.errors.get_mut(&identifier) {
                Some(current_errors) => {
                    current_errors.push(error);
                },
                None => {
                    self.errors.insert(identifier, vec![error]);
                }
            }
        }
    }

    /// Add a mapping of [`Identifier`]s and the errors they've produced.
    pub fn extend_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = (Option<String>, Vec<ServerError>)>
    {
        for (identifier, my_errors) in errors {
            match self.errors.get_mut(&identifier) {
                Some(current_errors) => {
                    current_errors.extend(my_errors);
                },
                None => {
                    self.errors.insert(identifier, my_errors);
                }
            }
        }
    }
}