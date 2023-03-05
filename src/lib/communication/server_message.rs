use std::{process, io, fmt, collections::HashMap, hash::Hash};

use serde::{Serialize, Deserialize};

use crate::{traits::Codecable, error::FindStopwatchError, identifiers::Identifier};

use super::{
    start::StartReply,
    info::InfoReply,
    stop::StopReply,
    lap::LapReply,
    pause::PauseReply,
    play::PlayReply,
    delete::DeleteReply,
    details::StopwatchDetails
};

pub fn details_map_into<I, K, V>(iter: I) -> HashMap<K, V>
where
    I: IntoIterator<Item = (K, StopwatchDetails)>,
    K: Hash + Eq,
    V: From<StopwatchDetails>
{
    iter.into_iter().map(|(k, v)| (k, From::from(v))).collect()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerReplyKind {
    Start(StartReply),
    Info(InfoReply),
    Stop(StopReply),
    Lap(LapReply),
    Pause(PauseReply),
    Play(PlayReply),
    Delete(DeleteReply),
    #[default] Default
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerError {
    FindStopwatchError(FindStopwatchError),
    Other(String)
}

impl ServerError {
    pub fn get_identifier(&self) -> Option<&Identifier> {
        use ServerError::*;
        match self {
            FindStopwatchError(fse) => Some(&fse.identifier),
            Other(_) => None
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ServerError::*;
        match self {
            FindStopwatchError(fse) => write!(f, "{}", fse.diagnose()),
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

impl From<String> for ServerError {
    fn from(error: String) -> Self {
        Self::Other(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerReply {
    pub successful: HashMap<Identifier, StopwatchDetails>,
    pub errors: HashMap<Option<Identifier>, Vec<ServerError>>,
    pub specific_answer: ServerReplyKind
}

impl ServerReply {
    pub fn new(specific_answer: ServerReplyKind) -> Self {
        let successful = HashMap::new();
        let errors = HashMap::new();
        Self { successful, errors, specific_answer }
    }

    pub fn add_successful<I>(&mut self, successful: I)
    where
        I: IntoIterator<Item = StopwatchDetails>
    {
        self.extend_successful(successful.into_iter().map(|d| (d.get_identifier(), d)))
    }

    pub fn extend_successful<I>(&mut self, successful: I)
    where
        I: IntoIterator<Item = (Identifier, StopwatchDetails)>
    {
        self.successful.extend(successful);
    }

    pub fn add_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = ServerError>
    {
        self.extend_uncollected_errors(
            errors.into_iter().map(|e| (e.get_identifier().cloned(), e))
        );
    }

    pub fn extend_uncollected_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = (Option<Identifier>, ServerError)>
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

    pub fn extend_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = (Option<Identifier>, Vec<ServerError>)>
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

impl Codecable<'_> for ServerReply { }

impl Default for ServerReply {
    fn default() -> Self {
        let specific = ServerReplyKind::default();
        Self::new(specific)
    }
}

impl Into<ServerMessage> for ServerReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerMessage {
    pub pid: u32,
    pub reply: ServerReply
}

impl ServerMessage {
    pub fn create(reply: ServerReply) -> Self {
        let pid = process::id();
        Self { pid, reply }
    }
}

impl Codecable<'_> for ServerMessage { }

impl Default for ServerMessage {
    fn default() -> Self {
        let pid = process::id();
        let reply = ServerReply::default();
        Self { pid, reply }
    }
}

impl TryFrom<&[u8]> for ServerMessage {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl TryInto<Vec<u8>> for ServerMessage {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}