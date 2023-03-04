use std::collections::HashMap;

use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply,
        start::{StartSuccess, StartReply},
        info::{InfoReply, InfoSuccess},
        stop::{StopSuccess, StopReply},
        lap::{LapReply, LapSuccess},
        pause::{PauseReply, PauseSuccess},
        play::{PlayReply, PlaySuccess},
        delete::{DeleteReply, DeleteSuccess}, details::StopwatchDetails
    },
    models::stopwatch::{Stopwatch, Name},
    error::FindStopwatchError,
    identifiers::{UNMatchKind, UuidName, Identifier},
    traits::{FromStopwatch, FromStopwatches}
};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Request {
    pub action: ClientRequest,
    pub res_tx: ResponseSender
}

#[derive(Clone, Debug)]
pub struct Response {
    pub output: ServerReply
}

pub type RequestSender = UnboundedSender<Request>;
pub type RequestReceiver = UnboundedReceiver<Request>;
pub type ResponseSender = UnboundedSender<Response>;
pub type ResponseReceiver = UnboundedReceiver<Response>;

#[inline]
pub fn make_request_channels() -> (RequestSender, RequestReceiver) {
    unbounded_channel()
}

#[inline]
pub fn make_response_channels() -> (ResponseSender, ResponseReceiver) {
    unbounded_channel()
}

pub struct Manager {
    stopwatches: HashMap<Uuid, Stopwatch>,
    access_order: Vec<UuidName> // Last item is most recently accessed
}

impl Manager {
    pub fn new() -> Self {
        let stopwatches = HashMap::new();
        let access_order = Vec::new();
        Self { stopwatches, access_order }
    }

    pub fn add_stopwatch(&mut self, stopwatch: Stopwatch) {
        let un = stopwatch.get_uuid_name();
        self.stopwatches.insert(un.id, stopwatch);
        self.access_order.push(un);
    }

    pub fn has_name(&self, identifier: &str) -> Option<UuidName> {
        if identifier.is_empty() {
            return None;
        }
        for (_, stopwatch) in self.stopwatches.iter() {
            if &*stopwatch.name == identifier {
                return Some(stopwatch.get_uuid_name());
            }
        }
        None
    }

    pub fn has_uuid(&self, identifier: &str) -> Option<UuidName> {
        let my_uuid = match Uuid::parse_str(identifier) {
            Ok(id) => id,
            Err(_) => return None
        };
        for (_, stopwatch) in self.stopwatches.iter() {
            if stopwatch.id == my_uuid {
                return Some(stopwatch.get_uuid_name());
            }
        }
        None
    }

    pub fn has_uuid_or_name(&self, identifier: &str) -> Option<UuidName> {
        // TODO: Might make this more efficient
        if let Some(un) = self.has_name(identifier) {
            Some(un)
        } else if let Some(un) = self.has_uuid(identifier) {
            Some(un)
        } else {
            None
        }
    }

    /// Get the index of the stopwatch that matches `identifier` inside `access_order`
    fn find_ao_index(&self, identifier: &Identifier) -> Result<usize, FindStopwatchError> {
        let mut possible_index = None;
        for (index, uuid_name) in self.access_order.iter().enumerate() {
            match uuid_name.matches(identifier) {
                Some(UNMatchKind::Name) => {
                    return Ok(index);
                },
                Some(UNMatchKind::Uuid) => {
                    if let Some(pi) = possible_index {
                        return Err(self.manager_stopwatches_error(identifier, &[pi, index]));
                    }
                    possible_index = Some(index);
                },
                None => { }
            }
        }
        possible_index.ok_or_else(|| 
            FindStopwatchError { identifier: identifier.clone(), duplicates: vec![] }
        )
    }

    pub fn get_stopwatch_by_identifier(
        &mut self,
        identifier: &Identifier
    ) -> Result<&mut Stopwatch, FindStopwatchError> {
        let ao_index = self.find_ao_index(identifier)?;
        let uuid_name = self.access_order.remove(ao_index);
        match self.stopwatches.get_mut(&uuid_name.id) {
            Some(sw) => {
                self.access_order.push(uuid_name);
                Ok(sw)
            },
            None => Err(FindStopwatchError {
                identifier: identifier.clone(),
                duplicates: vec![]
            })
        }
    }

    pub fn take_stopwatch_by_identifier(
        &mut self,
        identifier: &Identifier
    ) -> Result<Stopwatch, FindStopwatchError> {
        let ao_index = self.find_ao_index(identifier)?;
        let uuid_name = self.access_order.remove(ao_index);
        match self.stopwatches.remove(&uuid_name.id) {
            Some(sw) => Ok(sw),
            None => Err(FindStopwatchError {
                identifier: identifier.clone(), duplicates: vec![]
            })
        }
    }

    fn manager_stopwatches_error(
        &self,
        identifier: &Identifier,
        indices: &[usize]
    ) -> FindStopwatchError {
        let mut duplicates = vec![];
        for index in indices {
            let uuid_name = &self.access_order[*index];
            if let Some(_sw) = self.stopwatches.get(&uuid_name.id) {
                duplicates.push(uuid_name.clone());
            }
        }
        let identifier = identifier.clone();
        FindStopwatchError { identifier, duplicates }
    }

    pub (self) fn stopwatches_by_access_order(&self) -> StopwatchByAccessOrder<'_> {
        StopwatchByAccessOrder {
            stopwatches: &self.stopwatches,
            access_order: &self.access_order,
            index: 0
        }
    }
}


struct StopwatchByAccessOrder<'m> {
    stopwatches: &'m HashMap<Uuid, Stopwatch>,
    access_order: &'m Vec<UuidName>,
    index: usize
}

impl<'m> Iterator for StopwatchByAccessOrder<'m> {
    type Item = &'m Stopwatch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.access_order.len() {
            None
        } else {
            let uuid_name = self.access_order.get(self.index)?;
            self.index += 1;
            self.stopwatches.get(&uuid_name.id)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.access_order.len() - self.index;
        (remaining, Some(remaining))
    }
}

async fn start(manager: &mut Manager, res_tx: &ResponseSender, req: &ClientRequest) {
    let identifier = req.identifiers.first().cloned().unwrap_or_default();
    let name = Name::new(&*identifier);

    // Start stopwatch first, delete if need be
    let stopwatch = Stopwatch::start(Some(name.clone()));

    if let Some(uuid_name) = manager.has_uuid_or_name(&name) {
        trace!("stopwatch with the same name or uuid already exists");
        let reply = StartReply {
            start: Err(FindStopwatchError {
                identifier,
                duplicates: vec![uuid_name]
            })
        };
        let response = Response { output: reply.into() };
        if let Err(e) = res_tx.send(response) {
            error!("{}", e);
        }
        return;
    }

    let reply = StartSuccess::from(&stopwatch).into();
    manager.add_stopwatch(stopwatch);
    let response = Response { output: ServerReply::Start(reply) };
    trace!("manage is sending response back for start");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    }
    println!("stopwatches: {:?}", manager.stopwatches);
}

async fn all(manager: &mut Manager, res_tx: &ResponseSender, req: &ClientRequest) {
    let mut details = HashMap::<Identifier, StopwatchDetails>::new();
    let mut errored = HashMap::<Identifier, FindStopwatchError>::new();
    for identifier in &req.identifiers {
        let identifier = identifier.clone();
        match manager.get_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                if req.specific_args.is_info() {
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                } else if req.specific_args.is_stop() {
                    sw.end();
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                } else if req.specific_args.is_lap() {
                    sw.new_lap(true);
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                } else if req.specific_args.is_pause() {
                    sw.pause();
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                } else if req.specific_args.is_play() {
                    sw.play();
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                }
            },
            Err(e) => {
                errored.insert(identifier, e);
            }
        }
    }

    let reply: ServerReply = if req.specific_args.is_info() {
        let success = details.into_iter().map(|(k, v)| (k, InfoSuccess::from(v))).collect();
        InfoReply { success, errored }.into()
    } else if req.specific_args.is_stop() {
        let success = details.into_iter().map(|(k, v)| (k, StopSuccess::from(v))).collect();
        StopReply { success, errored }.into()
    } else if req.specific_args.is_lap() {
        let success = details.into_iter().map(|(k, v)| (k, LapSuccess::from(v))).collect();
        LapReply { success, errored }.into()
    } else if req.specific_args.is_pause() {
        let success = details.into_iter().map(|(k, v)| (k, PauseSuccess::from(v))).collect();
        PauseReply { success, errored }.into()
    } else if req.specific_args.is_play() {
        let success = details.into_iter().map(|(k, v)| (k, PlaySuccess::from(v))).collect();
        PlayReply { success, errored }.into()
    } else {
        ServerReply::Default
    };
    let response = Response { output: reply };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent result back to user");
    }
}

async fn info_all(manager: &mut Manager, res_tx: &ResponseSender, req: &ClientRequest) {
    trace!("info_all");
    let reply = InfoReply::from_stopwatches(manager.stopwatches_by_access_order(), req.verbose)
        .into();
    let response = Response {
        output: reply
    };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent info list back to user");
    }
}

async fn delete(manager: &mut Manager, res_tx: &ResponseSender, req: &ClientRequest) {
    trace!("got request for delete");
    let mut reply = DeleteReply::new();
    for identifier in &req.identifiers {
        match manager.take_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                reply.add_success(DeleteSuccess::from_stopwatch(&sw, req.verbose));
            },
            Err(e) => {
                reply.add_error(e);
            }
        }
    }
    let response = Response { output: reply.into() };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent stop back to user");
    }
}

async fn default(res_tx: &ResponseSender) {
    let response = Response { output: ServerReply::Default };
    trace!("manage is sending response back for default");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e)
    }
}

pub async fn manage(mut manager: Manager, mut req_rx: RequestReceiver) {
    debug!("start manage");
    while let Some(message) = req_rx.recv().await {
        trace!("manage received message");
        let request = message.action;
        if request.specific_args.is_start() {
            start(&mut manager, &message.res_tx, &request).await;
        } else if request.specific_args.is_info() && request.identifiers.len() == 0 {
            info_all(&mut manager, &message.res_tx, &request).await;
        } else if request.specific_args.is_default() {
            default(&message.res_tx).await;
        } else if request.specific_args.is_delete() {
            delete(&mut manager, &message.res_tx, &request).await;
        } else {
            all(&mut manager, &message.res_tx, &request).await;
        }
    }
    debug!("stop manage");
}
