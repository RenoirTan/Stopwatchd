use std::collections::HashMap;

use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply,
        start::{StartSuccess, StartRequest, StartReply},
        info::{InfoRequest, InfoReply, InfoSuccess},
        stop::{StopRequest, StopSuccess, StopReply},
        lap::{LapRequest, LapReply, LapSuccess},
        pause::{PauseRequest, PauseReply, PauseSuccess}
    },
    models::stopwatch::Stopwatch,
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

async fn start(manager: &mut Manager, res_tx: &ResponseSender, req: StartRequest) {
    // Start stopwatch first, delete if need be
    let stopwatch = Stopwatch::start(Some(req.name.clone()));

    if let Some(uuid_name) = manager.has_uuid_or_name(&req.name) {
        trace!("stopwatch with the same name or uuid already exists");
        let reply = StartReply {
            start: Err(FindStopwatchError {
                identifier: Identifier::new((*req.name).clone()),
                duplicates: vec![uuid_name]
            })
        };
        let response = Response { output: reply.into() };
        if let Err(e) = res_tx.send(response) {
            error!("{}", e);
            return; // Exit before the bad stopwatch gets added
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
    if req.identifiers.len() > 0 {
        info_specified(manager, res_tx, req).await
    } else {
        info_all(manager, res_tx, req).await
    }
}

async fn info_specified(manager: &mut Manager, res_tx: &ResponseSender, req: InfoRequest) {
    trace!("info_specified");
    let mut reply = InfoReply::new();
    for identifier in &req.identifiers {
        match manager.get_stopwatch_by_identifier(identifier) {
            Ok(sw) => {
                let success = InfoSuccess::from_stopwatch(sw, req.verbose);
                reply.add_success(success);
            },
            Err(fse) => reply.add_error(fse)
        }
    }
    let response = Response { output: reply.into() };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent info back to user");
    }
}

async fn info_all(manager: &mut Manager, res_tx: &ResponseSender, req: InfoRequest) {
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

async fn stop(manager: &mut Manager, res_tx: &ResponseSender, req: StopRequest) {
    trace!("got request for stop");
    let mut reply = StopReply::new();
    for identifier in &req.identifiers {
        match manager.get_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                sw.end();
                reply.add_success(StopSuccess::from_stopwatch(sw, req.verbose));
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

async fn lap(manager: &mut Manager, res_tx: &ResponseSender, req: LapRequest) {
    trace!("got request for lap");
    let mut reply = LapReply::new();
    for identifier in &req.identifiers {
        match manager.get_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                sw.new_lap(true);
                reply.add_success(LapSuccess::from_stopwatch(sw, req.verbose));
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

async fn pause(manager: &mut Manager, res_tx: &ResponseSender, req: PauseRequest) {
    trace!("got request for pause");
    let mut reply = PauseReply::new();
    for identifier in &req.identifiers {
        match manager.get_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                sw.pause();
                reply.add_success(PauseSuccess::from_stopwatch(sw, req.verbose));
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
        use ClientRequest::*;
        match message.action {
            Start(start_req) => start(&mut manager, &message.res_tx, start_req).await,
            Info(info_req) => info(&mut manager, &message.res_tx, info_req).await,
            Stop(stop_req) => stop(&mut manager, &message.res_tx, stop_req).await,
            Lap(lap_req) => lap(&mut manager, &message.res_tx, lap_req).await,
            Pause(pause_req) => pause(&mut manager, &message.res_tx, pause_req).await,
            Default => default(&message.res_tx).await
        }
    }
    debug!("stop manage");
}
