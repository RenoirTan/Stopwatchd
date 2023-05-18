//! Ways to refer to a [`Stopwatch`].

use std::{fmt, str::FromStr, ops};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[allow(unused)]
use crate::models::stopwatch::Stopwatch; // for see also documentation

pub type UniqueIdBytes = [u8; 6];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotUniqueIdError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UniqueId(UniqueIdBytes);

impl UniqueId {
    pub fn new(bytes: UniqueIdBytes) -> Self {
        Self(bytes)
    }

    pub fn generate() -> Self {
        Self::from(Uuid::new_v4())
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", hex::encode(&self.0))
    }
}

impl FromStr for UniqueId {
    type Err = NotUniqueIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("@") { return Err(NotUniqueIdError); }
        let mut bytes = UniqueIdBytes::default();
        match hex::decode_to_slice(&s[1..], &mut bytes) {
            Ok(_) => Ok(Self::new(bytes)),
            Err(_) => Err(NotUniqueIdError)
        }
    }
}

impl From<Uuid> for UniqueId {
    fn from(uuid: Uuid) -> Self {
        let bytes: UniqueIdBytes = uuid.as_bytes()[10..16].try_into()
            .expect("<UniqueId as From>::<Uuid>::from stopped working");
        Self::new(bytes)
    }
}

/// If a name starts with '@'.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BadNameError;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Result<Self, BadNameError> {
        let name: String = name.into();
        if bad_name(&name) {
            Err(BadNameError)
        } else {
            Ok(Self(name))
        }
    }

    pub fn fixed<S: Into<String>>(unchecked: S) -> Self {
        let mut unchecked: String = unchecked.into();
        if bad_name(&unchecked) {
            unchecked.remove(0);
        }
        Self(unchecked)
    }
}

impl ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifies a [`Stopwatch`] using a [`Uuid`] and [`Name`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub id: UniqueId,
    pub name: Name
}

impl Identifier {
    /// Create a new identifier.
    pub fn new(id: UniqueId, name: Name) -> Self {
        Self { id, name }
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        if self.name.is_empty() {
            self.id.to_string()
        } else {
            self.name.into()
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printout = if self.name.is_empty() {
            self.id.to_string()
        } else {
            self.name.to_string()
        };
        write!(f, "{}", printout)
    }
}

fn bad_name(name: &str) -> bool {
    name.starts_with("@")
}