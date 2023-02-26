use std::collections::HashMap;

use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply, start::{StartSuccess, StartRequest, StartReply},
        info::{InfoRequest, InfoReply, InfoSuccess}, info_list::InfoListSuccess
    },
    models::stopwatch::{Stopwatch, UNMatchKind, FindStopwatchError, UuidName}
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

    pub fn add_most_recently_accessed(&mut self, uuid_name: UuidName) {
        self.access_order.push(uuid_name);
    }

    pub fn has_name(&self, identifier: &str) -> Option<UuidName> {
        if identifier.is_empty() {
            return None;
        }
        for (_, stopwatch) in self.stopwatches.iter() {
            if *stopwatch.name == identifier {
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

    fn get_stopwatches_indices_by_identifier(
        &self,
        identifier: &str,
        only_one: bool
    ) -> (Vec<usize>, UNMatchKind) {
        let mut name_matches = vec![];
        let mut uuid_matches = vec![];
        // Reverse because last item is the most recently accessed
        'a: for index in (0..self.access_order.len()).rev() {
            let uuid_name = &self.access_order[index];
            let match_kind = uuid_name.matches(identifier);
            // Prefer name matches over uuid matches
            // So if a name match was found already, ignore uuid matches
            match match_kind {
                Some(UNMatchKind::Name) => {
                    name_matches.push(index);
                    if only_one {
                        break 'a;
                    }
                },
                Some(UNMatchKind::Uuid) => {
                    if name_matches.len() > 0 {
                        continue;
                    }
                    uuid_matches.push(index);
                    // There may be match by name after this,
                    // so we don't skip even if only_one is true
                },
                None => continue
            }
        }
        if name_matches.len() > 0 {
            (name_matches, UNMatchKind::Name)
        } else {
            (uuid_matches, UNMatchKind::Uuid)
        }
    }

    pub fn get_stopwatch_by_identifier(
        &mut self,
        identifier: &str
    ) -> Result<(Option<&mut Stopwatch>, UuidName), FindStopwatchError> {
        let (indices, _) = self.get_stopwatches_indices_by_identifier(identifier, false);
        if indices.len() != 1 {
            Err(self.manager_stopwatches_error(identifier.to_string(), &indices))
        } else {
            // Removed from access order, must be added back in
            let uuid_name = self.access_order.remove(indices[0]);
            Ok((self.stopwatches.get_mut(&uuid_name.id), uuid_name))
        }
    }

    fn manager_stopwatches_error(
        &self,
        identifier: String,
        indices: &[usize]
    ) -> FindStopwatchError {
        let mut duplicates = vec![];
        for index in indices {
            let uuid_name = &self.access_order[*index];
            if let Some(stopwatch) = self.stopwatches.get(&uuid_name.id) {
                duplicates.push((stopwatch.id, stopwatch.name.clone()));
            }
        }
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
        if self.index == self.access_order.len() {
            None
        } else {
            let uuid = self.access_order.get(self.index)?;
            self.index += 1;
            self.stopwatches.get(&uuid.id)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.access_order.len() - self.index;
        (remaining, Some(remaining))
    }
}

async fn start(manager: &mut Manager, res_tx: &ResponseSender, req: StartRequest) {
    // Start stopwatch first, delete if need be
    let stopwatch = Stopwatch::start(Some(req.name.clone()));

    if let Some(uuid_name) = manager.has_uuid_or_name(&req.name) {
        trace!("stopwatch with the same name or uuid already exists");
        let reply = StartReply {
            start: Err(FindStopwatchError {
                identifier: (*req.name).clone(),
                duplicates: vec![(uuid_name.id, uuid_name.name)]
            })
        };
        let response = Response { output: reply.into() };
        if let Err(e) = res_tx.send(response) {
            error!("{}", e);
        }
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

async fn info(manager: &mut Manager, res_tx: &ResponseSender, req: InfoRequest) {
    trace!("got request for info");
    // If an identifier given, search for just that one stopwatch
    match req.identifier {
        Some(_) => info_one(manager, res_tx, req).await,
        None => info_list(manager, res_tx, req).await
    }
}

async fn info_one(manager: &mut Manager, res_tx: &ResponseSender, req: InfoRequest) {
    trace!("info_one");
    let identifier = req.identifier.unwrap().clone();
    let (response, uuid_name) = match manager.get_stopwatch_by_identifier(&identifier) {
        Ok((Some(sw), uuid_name)) => {
            let reply = InfoSuccess::from_stopwatch(&sw, req.verbose).into();
            (Response { output: reply }, Some(uuid_name))
        },
        Ok((None, _)) => {
            warn!("found a uuid/name match but stopwatch was not in hashmap");
            let fse = FindStopwatchError {
                identifier,
                duplicates: vec![]
            };
            let response = Response {
                output: ServerReply::Info(InfoReply { info: Err(fse) })
            };
            // don't send UuidName so that it can be removed from the access order
            (response, None)
        },
        Err(fse) => {
            let response = Response {
                output: ServerReply::Info(InfoReply { info: Err(fse) })
            };
            (response, None)
        }
    };
    // UuidName must be added here to please the borrow checker
    if let Some(un) = uuid_name {
        manager.add_most_recently_accessed(un);
    }
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent info back to user");
    }
}

async fn info_list(manager: &mut Manager, res_tx: &ResponseSender, _: InfoRequest) {
    trace!("info_list");
    let reply = InfoListSuccess::from_iter(manager.stopwatches_by_access_order()).into();
    let response = Response {
        output: reply
    };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent info list back to user");
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
        use ClientRequest::*;
        match message.action {
            Start(start_req) => start(&mut manager, &message.res_tx, start_req).await,
            Info(info_req) => info(&mut manager, &message.res_tx, info_req).await,
            Default => default(&message.res_tx).await
        }
    }
    debug!("stop manage");
}
