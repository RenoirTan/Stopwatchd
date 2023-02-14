use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::stopwatch::{Name, State, Stopwatch}
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientStartStopwatch {
    pub verbose: bool
}

impl Codecable<'_> for ClientStartStopwatch { }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStartStopwatch {
    pub sw_id: Uuid,
    pub name: Option<Name>,
    pub state: State
}

impl Codecable<'_> for ServerStartStopwatch { }

impl From<&Stopwatch> for ServerStartStopwatch {
    fn from(stopwatch: &Stopwatch) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name;
        let state = stopwatch.state();
        Self { sw_id, name, state }
    }
}