//! Ways to refer to a [`Stopwatch`].

use std::fmt;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::util::get_uuid_node;
#[allow(unused)]
use crate::models::stopwatch::Stopwatch; // for see also documentation

/// Identifies a [`Stopwatch`] using a [`Uuid`] and [`Name`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub id: Uuid,
    pub name: String
}

impl Identifier {
    /// Create a new identifier.
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        if self.name.is_empty() {
            get_uuid_node(&self.id).to_string()
        } else {
            self.name
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printout = if self.name.is_empty() {
            get_uuid_node(&self.id).to_string()
        } else {
            self.name.clone()
        };
        write!(f, "{}", printout)
    }
}