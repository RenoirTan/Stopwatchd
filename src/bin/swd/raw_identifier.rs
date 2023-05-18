//! [`RawIdentifier`] and [`IdentifierMatch`].
#![allow(unused)]

use std::{fmt, str::FromStr};
use std::ops::Deref;

use serde::{Serialize, Deserialize};
use stopwatchd::{util::{raw_identifier_to_uuid_node, get_uuid_node}, identifiers::{Identifier, UniqueId}};
use uuid::Uuid;

/// [`RawIdentifier`]s are unresolved references to a [`Stopwatch`] that are
/// passed into `swctl` from the command line.
/// Identifiers can either be the name of a stopwatch or the id of the
/// stopwatch. Finding a stopwatch that matches a [`RawIdentifier`] is handled
/// by `swd`.
#[derive(Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub struct RawIdentifier {
    raw: String,
    possible_id: Option<UniqueId>
}

impl RawIdentifier {
    /// Create a new [`Identifier`] from raw input which can be passed by a
    /// user from the command line.
    pub fn new<S: Into<String>>(raw: S) -> Self {
        let mut me = Self { raw: raw.into(), possible_id: None };
        me.calculate_id();
        me
    }

    /// Borrow this raw identifier as a [`str`].
    pub fn get_identifier(&self) -> &str {
        &self.raw
    }

    /// Check if the raw identifier looks like a [`UniqueId`].
    pub fn calculate_id(&mut self) -> Option<UniqueId> {
        self.possible_id = UniqueId::from_str(&self.raw).ok();
        self.possible_id
    }

    /// Return a UUID "node" if this raw identifier is like one.
    pub fn get_possible_id(&self) -> Option<UniqueId> {
        self.possible_id
    }

    /// Whether this raw identifier matches a name.
    pub fn matches_name(&self, name: &str) -> bool {
        self.raw == name
    }

    /// Whether this raw identifier matches a [`UniqueId`].
    pub fn matches_id(&self, id: &UniqueId) -> bool {
        match self.possible_id {
            Some(pos_id) => pos_id == *id,
            None => false
        }
    }

    /// Whether this raw identifier matches an [`Identifier`].
    pub fn matches(&self, identifier: &Identifier) -> Option<IdentifierMatch> {
        if self.matches_name(&identifier.name) {
            Some(IdentifierMatch::Name)
        } else if self.matches_id(&identifier.id) {
            Some(IdentifierMatch::Uuid)
        } else {
            None
        }
    }
}

impl Default for RawIdentifier {
    fn default() -> Self {
        Self::new("")
    }
}

impl Deref for RawIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<String> for RawIdentifier {
    fn from(raw: String) -> Self {
        Self::new(raw)
    }
}

impl From<Identifier> for RawIdentifier {
    fn from(identifier: Identifier) -> Self {
        Self::from(<Identifier as Into<String>>::into(identifier))
    }
}

impl Into<String> for RawIdentifier {
    fn into(self) -> String {
        self.raw
    }
}

impl fmt::Display for RawIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl PartialEq<RawIdentifier> for RawIdentifier {
    fn eq(&self, other: &RawIdentifier) -> bool {
        self.raw == other.raw
    }
}

/// Whether a [`Stopwatch`]'s name or UUID matched a string.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdentifierMatch {
    Name,
    Uuid
}

impl IdentifierMatch {
    pub fn name_matched(self) -> bool {
        matches!(self, IdentifierMatch::Name)
    }

    pub fn uuid_matched(self) -> bool {
        matches!(self, IdentifierMatch::Uuid)
    }
}