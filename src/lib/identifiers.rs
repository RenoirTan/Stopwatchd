//! Ways to refer to a [`Stopwatch`].

use std::{fmt, ops::Deref};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{models::stopwatch::Name, util::get_uuid_node};
#[allow(unused)]
use crate::models::stopwatch::Stopwatch; // for see also documentation

/// Identifiers are unresolved references to a [`Stopwatch`] that are passed
/// into `swctl` from the command line. Identifiers can either be the name of
/// a stopwatch or the id of the stopwatch. Finding a stopwatch that matches
/// an [`Identifier`] is handled by `swd`.
#[derive(Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    raw: String,
    possible_node: Option<u64>
}

impl Identifier {
    /// Create a new [`Identifier`] from raw input which can be passed by a
    /// user from the command line.
    pub fn new<S: Into<String>>(raw: S) -> Self {
        let mut me = Self { raw: raw.into(), possible_node: None };
        me.calculate_node();
        me
    }

    /// Generate an identifier for a [`Stopwatch`] using its UUID and name.
    /// 
    /// ```ignore
    /// let name = "sw-1";
    /// let stopwatch = Stopwatch::new(Some(name));
    /// let uuid_name = stopwatch.get_uuid_name();
    /// let identifier = Identifier::from_uuid_name(&uuid_name);
    /// ```
    pub fn from_uuid_name(uuid_name: &UuidName) -> Self {
        if uuid_name.name.is_empty() {
            let uuid = get_uuid_node(&uuid_name.id);
            let raw = format!("{:X}", uuid);
            Self { raw, possible_node: Some(uuid) }
        } else {
            Self::new((*uuid_name.name).to_string())
        }
    }

    /// Borrow this identifier as a [`str`].
    pub fn get_identifier(&self) -> &str {
        &self.raw
    }

    /// Convert this identifier into an owned [`String`].
    pub fn to_string(&self) -> String {
        self.raw.clone()
    }

    /// Check if the raw identifier looks like the "node" in a UUID.
    /// If so, return the node as a [`u64`].
    pub fn calculate_node(&mut self) -> Option<u64> {
        let possible = u64::from_str_radix(&self.raw, 16).ok()?;
        self.possible_node = Some(possible);
        self.possible_node
    }

    /// Return a UUID "node" if this identifier is like one.
    pub fn get_possible_node(&self) -> Option<u64> {
        self.possible_node
    }

    /// Whether this identifier matches a [`Name`].
    pub fn matches_name(&self, name: &Name) -> bool {
        self.raw == **name
    }

    /// Whether this identifier matches a [`Uuid`]'s node.
    pub fn matches_uuid(&self, uuid: &Uuid) -> bool {
        match self.possible_node {
            Some(node) => node == get_uuid_node(uuid),
            None => false
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::new("")
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<T: AsRef<str>> From<T> for Identifier {
    fn from(raw: T) -> Self {
        Self::new(raw.as_ref().to_string())
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl PartialEq<Identifier> for Identifier {
    fn eq(&self, other: &Identifier) -> bool {
        self.raw == other.raw
    }
}

impl<T: AsRef<str>> PartialEq<T> for Identifier {
    fn eq(&self, other: &T) -> bool {
        self.raw == other.as_ref()
    }
}

/// Whether a [`Stopwatch`]'s name or UUID matched an [`Identifier`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UNMatchKind {
    Name,
    Uuid
}

impl UNMatchKind {
    pub fn name_matched(self) -> bool {
        matches!(self, UNMatchKind::Name)
    }

    pub fn uuid_matched(self) -> bool {
        matches!(self, UNMatchKind::Uuid)
    }
}

/// [`Uuid`] and [`Name`] of a [`Stopwatch`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UuidName {
    pub id: Uuid,
    pub name: Name
}

impl UuidName {
    /// Checks if an [`Identifier`] matches the UUID or name of a stopwatch.
    /// Returns [`Some`] if a match is found.
    pub fn matches(&self, identifier: &Identifier) -> Option<UNMatchKind> {
        if identifier.matches_name(&self.name) {
            Some(UNMatchKind::Name)
        } else if identifier.matches_uuid(&self.id) {
            Some(UNMatchKind::Uuid)
        } else {
            None
        }
    }

    /// See [`Identifier::from_uuid_name`].
    pub fn as_identifier(&self) -> Identifier {
        Identifier::from_uuid_name(self)
    }
}