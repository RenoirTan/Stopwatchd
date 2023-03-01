use std::fmt;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{models::stopwatch::Name, util::get_uuid_node};

#[derive(Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    raw: String,
    possible_node: Option<u64>
}

impl Identifier {
    pub fn new(raw: String) -> Self {
        let mut me = Self { raw, possible_node: None };
        me.calculate_node();
        me
    }

    pub fn from_uuid_name(uuid_name: &UuidName) -> Self {
        if uuid_name.name.is_empty() {
            let uuid = get_uuid_node(&uuid_name.id);
            let raw = format!("{:X}", uuid);
            Self { raw, possible_node: Some(uuid) }
        } else {
            Self::new((*uuid_name.name).clone())
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.raw
    }

    pub fn to_string(&self) -> String {
        self.raw.clone()
    }

    pub fn calculate_node(&mut self) -> Option<u64> {
        let possible = u64::from_str_radix(&self.raw, 16).ok()?;
        self.possible_node = Some(possible);
        self.possible_node
    }

    pub fn get_possible_node(&self) -> Option<u64> {
        self.possible_node
    }

    pub fn matches_name(&self, name: &Name) -> bool {
        self.raw == **name
    }

    pub fn matches_uuid(&self, uuid: &Uuid) -> bool {
        match self.possible_node {
            Some(node) => node == get_uuid_node(uuid),
            None => false
        }
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

impl PartialEq<Name> for Identifier {
    fn eq(&self, other: &Name) -> bool {
        self.raw == **other
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UuidName {
    pub id: Uuid,
    pub name: Name
}

impl UuidName {
    pub fn matches(&self, identifier: &Identifier) -> Option<UNMatchKind> {
        if identifier.matches_name(&self.name) {
            Some(UNMatchKind::Name)
        } else if identifier.matches_uuid(&self.id) {
            Some(UNMatchKind::Uuid)
        } else {
            None
        }
    }

    pub fn as_identifier(&self) -> Identifier {
        Identifier::from_uuid_name(self)
    }
}