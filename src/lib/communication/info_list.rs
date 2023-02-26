use std::time::Duration;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    traits::Codecable,
    models::stopwatch::{Name, State, Stopwatch, FindStopwatchError}
};

use super::server_message::{ServerReply, ServerMessage};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoListReply {
    pub info: Result<InfoListSuccess, FindStopwatchError>
}

impl InfoListReply {
    pub fn found(&self) -> bool {
        self.info.is_ok()
    }
}

impl Into<ServerReply> for InfoListReply {
    fn into(self) -> ServerReply {
        ServerReply::InfoList(self)
    }
}

impl Into<ServerMessage> for InfoListReply {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}

impl Codecable<'_> for InfoListReply { }


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoListSuccess {
    pub items: Vec<InfoListItem>
}

impl Codecable<'_> for InfoListSuccess { }

impl<'m> FromIterator<&'m Stopwatch> for InfoListSuccess {
    fn from_iter<T: IntoIterator<Item = &'m Stopwatch>>(iter: T) -> Self {
        let items = iter.into_iter()
            .map(|s| InfoListItem::from_stopwatch(s))
            .collect();
        InfoListSuccess { items }
    }
}

impl Into<InfoListReply> for InfoListSuccess {
    fn into(self) -> InfoListReply {
        InfoListReply { info: Ok(self) }
    }
}

impl Into<ServerReply> for InfoListSuccess {
    fn into(self) -> ServerReply {
        ServerReply::InfoList(self.into())
    }
}

impl Into<ServerMessage> for InfoListSuccess {
    fn into(self) -> ServerMessage {
        ServerMessage::create(self.into())
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoListItem {
    pub sw_id: Uuid,
    pub name: Name,
    pub state: State,
    pub total_time: Duration,
    pub laps_count: usize
}

impl InfoListItem {
    pub fn from_stopwatch(stopwatch: &Stopwatch) -> Self {
        let sw_id = stopwatch.id;
        let name = stopwatch.name.clone();
        let state = stopwatch.state();
        let total_time = stopwatch.total_time();
        let laps_count = stopwatch.laps();
        Self {
            sw_id,
            name,
            state,
            total_time,
            laps_count
        }
    }
}

impl Codecable<'_> for InfoListItem { }
