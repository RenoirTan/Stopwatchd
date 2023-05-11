//! Manages the stopwatch.

use std::collections::HashMap;

use stopwatchd::{
    communication::{
        request::{Request, RequestKind},
        reply::{Reply, ServerError},
        start::StartReply,
        info::{InfoReply, InfoAll},
        delete::DeleteReply,
        details::StopwatchDetails
    },
    models::stopwatch::{Stopwatch, Name, State},
    error::{FindStopwatchError, InvalidState},
    identifiers::{UNMatchKind, UuidName, Identifier}
};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use uuid::Uuid;

use crate::utils::crk_to_srk;

#[derive(Clone, Debug)]
pub struct Request {
    pub action: Request,
    pub res_tx: ResponseSender
}

#[derive(Clone, Debug)]
pub struct Response {
    pub output: Reply
}

pub type RequestSender = UnboundedSender<Request>;
pub type RequestReceiver = UnboundedReceiver<Request>;
pub type ResponseSender = UnboundedSender<Response>;
pub type ResponseReceiver = UnboundedReceiver<Response>;

/// Create channels to send requests to [`Manager`].
#[inline]
pub fn make_request_channels() -> (RequestSender, RequestReceiver) {
    unbounded_channel()
}

/// Create channels for [`Manager`] to reply back with results.
#[inline]
pub fn make_response_channels() -> (ResponseSender, ResponseReceiver) {
    unbounded_channel()
}

/// Contains [`Stopwatch`]es and auxiliary info.
/// 
/// Use [`manage`] to run the manager.
pub struct Manager {
    stopwatches: HashMap<Uuid, Stopwatch>,
    access_order: Vec<UuidName> // Last item is most recently accessed
}

impl Manager {
    /// Vanilla manager.
    pub fn new() -> Self {
        let stopwatches = HashMap::new();
        let access_order = Vec::new();
        Self { stopwatches, access_order }
    }

    /// Add a stopwatch. Doesn't check whether UUID is unique yet.
    pub fn add_stopwatch(&mut self, stopwatch: Stopwatch) {
        let un = stopwatch.get_uuid_name();
        self.stopwatches.insert(un.id, stopwatch);
        self.access_order.push(un);
    }

    /// Check if a [`Stopwatch`] with the name `identifier` exists.
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

    /// Check if a [`Stopwatch`] with the UUID `identifier` exists.
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

    /// Check if a stopwatch with name or UUID `identifier`.
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

    /// Find a [`Stopwatch`] that matches `identifier` and get a reference to
    /// it.
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

    /// Find a [`Stopwatch`] that matches `identifier` and take ownership of it,
    /// removing it from [`Manager`].
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

    /// Generate a [`FindStopwatchError`] for `identifier` and duplicate
    /// stopwatches with access order `indices`. As the access order stored
    /// by [`Manager`] keeps on changing, this might be a source of race
    /// conditions.
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

    /// Get an iterator over the contained [`Stopwatch`]es, with those with the
    /// most recent uses/accesses yielded first.
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

async fn start(manager: &mut Manager, res_tx: &ResponseSender, req: &Request) {
    let given_identifier = req.identifiers.first().cloned();
    let name = given_identifier.clone().map(Name::new);

    // Start stopwatch first, delete if need be
    let stopwatch = Stopwatch::start(name.clone());
    let sw_identifier = &stopwatch.get_uuid_name().as_identifier();

    let mut reply = Reply::new(StartReply.into());

    if let Some(uuid_name) = manager.has_uuid_or_name(&sw_identifier) {
        trace!("stopwatch with the same name or uuid already exists");
        let error = FindStopwatchError {
            identifier: sw_identifier.clone(),
            duplicates: vec![uuid_name]
        };
        reply.extend_uncollected_errors([(given_identifier, error.into())]);
    } else {
        let details = StopwatchDetails::from_stopwatch(&stopwatch, req.verbose);
        manager.add_stopwatch(stopwatch);
        reply.extend_successful([(sw_identifier.clone(), details)]);
    }

    let response = Response { output: reply.into() };
    trace!("manage is sending response back for start");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    }
}

async fn good_or_bad(
    identifier: Identifier,
    stopwatch: &Stopwatch,
    verbose: bool,
    details: &mut HashMap<Identifier, StopwatchDetails>,
    errored: &mut HashMap<Option<Identifier>, ServerError>,
    error_condition: bool
) {
    if error_condition {
        let state = stopwatch.state();
        errored.insert(Some(identifier.clone()), InvalidState { identifier, state }.into());
    } else {
        details.insert(
            identifier,
            StopwatchDetails::from_stopwatch(stopwatch, verbose)
        );
    }
}

async fn all(manager: &mut Manager, res_tx: &ResponseSender, req: &Request) {
    let specific_args = &req.specific_args;
    let mut details = HashMap::<Identifier, StopwatchDetails>::new();
    let mut errored = HashMap::<Option<Identifier>, ServerError>::new();
    let verbose = req.verbose;
    for identifier in &req.identifiers {
        let identifier = identifier.clone();
        let deets = &mut details;
        let errs = &mut errored;
        match manager.get_stopwatch_by_identifier(&identifier) {
            Ok(sw) => match specific_args {
                RequestKind::Info(_) => {
                    details.insert(identifier, StopwatchDetails::from_stopwatch(sw, req.verbose));
                },
                RequestKind::Stop(_) => {
                    let state = sw.end();
                    good_or_bad(identifier, sw, verbose, deets, errs, state.ended()).await;
                },
                RequestKind::Lap(_) => {
                    let state = sw.new_lap(true);
                    good_or_bad(identifier, sw, verbose, deets, errs, state.ended()).await;
                },
                RequestKind::Pause(_) => {
                    let state = sw.pause();
                    let condition = matches!(state, State::Ended | State::Paused);
                    good_or_bad(identifier, sw, verbose, deets, errs, condition).await;
                },
                RequestKind::Play(_) => {
                    let state = sw.play();
                    let condition = matches!(state, State::Ended | State::Playing);
                    good_or_bad(identifier, sw, verbose, deets, errs, condition).await;
                }
                _ => { }
            },
            Err(fse) => {
                errored.insert(Some(identifier), fse.into());
            }
        }
    }

    let specific_reply = crk_to_srk(specific_args);
    let mut reply = Reply::new(specific_reply);
    reply.extend_successful(details);
    reply.extend_uncollected_errors(errored);

    let response = Response { output: reply.into() };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent result back to user");
    }
}

async fn info_all(manager: &mut Manager, res_tx: &ResponseSender, req: &Request) {
    trace!("info_all");
    let mut access_order = vec![];
    let mut details = vec![];

    for sw in manager.stopwatches_by_access_order() {
        access_order.push(sw.get_uuid_name().as_identifier().clone());
        details.push(StopwatchDetails::from_stopwatch(sw, req.verbose));
    }

    let mut reply = Reply::new(InfoReply::All(InfoAll { access_order }).into());
    reply.add_successful(details);
    let response = Response {
        output: reply
    };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent info list back to user");
    }
}

async fn delete(manager: &mut Manager, res_tx: &ResponseSender, req: &Request) {
    trace!("got request for delete");
    let mut reply = Reply::new(DeleteReply.into());
    for identifier in &req.identifiers {
        match manager.take_stopwatch_by_identifier(&identifier) {
            Ok(sw) => {
                reply.extend_successful([(
                    identifier.clone(),
                    StopwatchDetails::from_stopwatch(&sw, req.verbose)
                )]);
            },
            Err(e) => {
                reply.extend_uncollected_errors([(Some(identifier.clone()), e.into())]);
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
    let response = Response { output: Reply::default() };
    trace!("manage is sending response back for default");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e)
    }
}

/// Run a [`Manager`].
pub async fn manage(mut manager: Manager, mut req_rx: RequestReceiver) {
    debug!("start manage");
    while let Some(message) = req_rx.recv().await {
        trace!("manage received message");
        let request = message.action;
        match request.specific_args {
            RequestKind::Start(_) => {
                start(&mut manager, &message.res_tx, &request).await;
            },
            RequestKind::Info(_) if request.identifiers.len() == 0 => {
                info_all(&mut manager, &message.res_tx, &request).await;
            },
            RequestKind::Default => {
                default(&message.res_tx).await;
            },
            RequestKind::Delete(_) => {
                delete(&mut manager, &message.res_tx, &request).await;
            },
            _ => {
                all(&mut manager, &message.res_tx, &request).await;
            }
        }
    }
    debug!("stop manage");
}
