//! Manages the stopwatch.

use std::collections::HashMap;

use stopwatchd::{
    communication::{
        client::Request,
        server::{Reply, ServerError},
        reply_specifics::{InfoAll, DeleteAnswer, InfoAnswer, StartAnswer},
        details::StopwatchDetails,
        args_to_default_ans,
        request_specifics::SpecificArgs
    },
    models::stopwatch::{Stopwatch, State},
    error::{FindStopwatchError, InvalidState},
    identifiers::Identifier
};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use uuid::Uuid;

use crate::raw_identifier::{RawIdentifier, IdentifierMatch};

#[derive(Clone, Debug)]
pub struct JobRequest {
    pub action: Request,
    pub res_tx: ResponseSender
}

#[derive(Clone, Debug)]
pub struct JobResponse {
    pub output: Reply
}

pub type JobSender = UnboundedSender<JobRequest>;
pub type JobReceiver = UnboundedReceiver<JobRequest>;
pub type ResponseSender = UnboundedSender<JobResponse>;
pub type ResponseReceiver = UnboundedReceiver<JobResponse>;

/// Create channels to send requests to [`Manager`].
#[inline]
pub fn make_request_channels() -> (JobSender, JobReceiver) {
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
    access_order: Vec<Identifier> // Last item is most recently accessed
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
        let identifier = stopwatch.identifier.clone();
        self.stopwatches.insert(identifier.id, stopwatch);
        self.access_order.push(identifier);
    }

    /// Check if `raw` identifier matches one of the stopwatches by name or
    /// by UUID if `stop_when_uuid_matches` is true.
    pub fn has_uuid_or_name(
        &self,
        raw: &RawIdentifier,
        stop_when_uuid_matches: bool
    ) -> Option<(Identifier, IdentifierMatch)> {
        for identifier in &self.access_order {
            match raw.matches(identifier) {
                Some(IdentifierMatch::Name) =>
                    return Some((identifier.clone(), IdentifierMatch::Name)),
                Some(IdentifierMatch::Uuid) if stop_when_uuid_matches =>
                    return Some((identifier.clone(), IdentifierMatch::Uuid)),
                _ => continue
            }
        }
        None
    }

    /// Get the index of the stopwatch that matches `identifier` inside `access_order`
    fn find_ao_index(&self, raw: &RawIdentifier) -> Result<usize, FindStopwatchError> {
        let mut possible_index = None;
        for (index, identifier) in self.access_order.iter().enumerate() {
            match raw.matches(identifier) {
                Some(IdentifierMatch::Name) => {
                    return Ok(index);
                },
                Some(IdentifierMatch::Uuid) => {
                    if let Some(pi) = possible_index {
                        return Err(self.manager_stopwatches_error(raw, &[pi, index]));
                    }
                    possible_index = Some(index);
                },
                None => { }
            }
        }
        possible_index.ok_or_else(|| 
            FindStopwatchError { raw_identifier: raw.to_string(), duplicates: vec![] }
        )
    }

    /// Find a [`Stopwatch`] that matches `identifier` and get a reference to
    /// it.
    pub fn get_stopwatch_by_raw_id(
        &mut self,
        raw: &RawIdentifier
    ) -> Result<&mut Stopwatch, FindStopwatchError> {
        let ao_index = self.find_ao_index(raw)?;
        let uuid_name = self.access_order.remove(ao_index);
        match self.stopwatches.get_mut(&uuid_name.id) {
            Some(sw) => {
                self.access_order.push(uuid_name);
                Ok(sw)
            },
            None => Err(FindStopwatchError {
                raw_identifier: raw.to_string(),
                duplicates: vec![]
            })
        }
    }

    /// Find a [`Stopwatch`] that matches `identifier` and take ownership of it,
    /// removing it from [`Manager`].
    pub fn take_stopwatch_by_raw_id(
        &mut self,
        raw: &RawIdentifier
    ) -> Result<Stopwatch, FindStopwatchError> {
        let ao_index = self.find_ao_index(raw)?;
        let uuid_name = self.access_order.remove(ao_index);
        match self.stopwatches.remove(&uuid_name.id) {
            Some(sw) => Ok(sw),
            None => Err(FindStopwatchError {
                raw_identifier: raw.to_string(), duplicates: vec![]
            })
        }
    }

    /// Generate a [`FindStopwatchError`] for `identifier` and duplicate
    /// stopwatches with access order `indices`. As the access order stored
    /// by [`Manager`] keeps on changing, this might be a source of race
    /// conditions.
    fn manager_stopwatches_error(
        &self,
        raw: &RawIdentifier,
        indices: &[usize]
    ) -> FindStopwatchError {
        let mut duplicates = vec![];
        for index in indices {
            let uuid_name = &self.access_order[*index];
            if let Some(_sw) = self.stopwatches.get(&uuid_name.id) {
                duplicates.push(uuid_name.clone());
            }
        }
        FindStopwatchError { raw_identifier: raw.to_string(), duplicates }
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
    access_order: &'m Vec<Identifier>,
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
    let name = req.common_args.raw_identifiers.first().cloned();

    // Start stopwatch first, delete if need be
    let stopwatch = Stopwatch::start(name.clone());
    let sw_raw_id: RawIdentifier = stopwatch.identifier.clone().into();

    let mut reply = Reply::new(StartAnswer.into());

    // TODO: --no-name-and-uuid-clash -> name of new stopwatch must not clash
    //       with another stopwatch's uuid.
    if let Some((identifier, _match_kind)) = manager.has_uuid_or_name(&sw_raw_id, false) {
        trace!("stopwatch with the same name or uuid already exists");
        let error = FindStopwatchError {
            raw_identifier: sw_raw_id.into(),
            duplicates: vec![identifier]
        };
        // TODO: If name is None, error gets categorised as a system error.
        reply.extend_uncollected_errors([(name, error.into())]);
    } else {
        let details = StopwatchDetails::from_stopwatch(&stopwatch, req.common_args.verbose);
        manager.add_stopwatch(stopwatch);
        reply.extend_successful([(sw_raw_id.clone(), details)]);
    }

    let response = JobResponse { output: reply.into() };
    trace!("manage is sending response back for start");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    }
}

async fn good_or_bad(
    raw_str: String,
    stopwatch: &Stopwatch,
    verbose: bool,
    details: &mut HashMap<String, StopwatchDetails>,
    errored: &mut HashMap<Option<String>, ServerError>,
    error_condition: bool
) {
    if error_condition {
        let state = stopwatch.state();
        errored.insert(
            Some(raw_str.clone()),
            InvalidState { raw_identifier: raw_str, state }.into()
        );
    } else {
        details.insert(
            raw_str,
            StopwatchDetails::from_stopwatch(stopwatch, verbose)
        );
    }
}

async fn all(manager: &mut Manager, res_tx: &ResponseSender, req: &Request) {
    let specific_args = &req.specific_args;
    let mut details = HashMap::<String, StopwatchDetails>::new();
    let mut errored = HashMap::<Option<String>, ServerError>::new();
    let verbose = req.common_args.verbose;
    for raw_str in &req.common_args.raw_identifiers {
        let raw = RawIdentifier::new(raw_str);
        let raw_str = raw_str.clone();
        let deets = &mut details;
        let errs = &mut errored;
        match manager.get_stopwatch_by_raw_id(&raw) {
            Ok(sw) => match specific_args {
                SpecificArgs::Info(_) => {
                    details.insert(raw_str, StopwatchDetails::from_stopwatch(sw, verbose));
                },
                SpecificArgs::Stop(_) => {
                    let state = sw.end();
                    good_or_bad(raw_str, sw, verbose, deets, errs, state.ended()).await;
                },
                SpecificArgs::Lap(_) => {
                    let state = sw.new_lap(true);
                    good_or_bad(raw_str, sw, verbose, deets, errs, state.ended()).await;
                },
                SpecificArgs::Pause(_) => {
                    let state = sw.pause();
                    let condition = matches!(state, State::Ended | State::Paused);
                    good_or_bad(raw_str, sw, verbose, deets, errs, condition).await;
                },
                SpecificArgs::Play(_) => {
                    let state = sw.play();
                    let condition = matches!(state, State::Ended | State::Playing);
                    good_or_bad(raw_str, sw, verbose, deets, errs, condition).await;
                }
                _ => { }
            },
            Err(fse) => {
                errored.insert(Some(raw_str), fse.into());
            }
        }
    }

    let specific_reply = args_to_default_ans(specific_args);
    let mut reply = Reply::new(specific_reply);
    reply.extend_successful(details);
    reply.extend_uncollected_errors(errored);

    let response = JobResponse { output: reply.into() };
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
        access_order.push(sw.identifier.clone().into());
        details.push(StopwatchDetails::from_stopwatch(sw, req.common_args.verbose));
    }

    let mut reply = Reply::new(InfoAnswer::All(InfoAll { access_order }).into());
    reply.add_successful(details);
    let response = JobResponse {
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
    let mut reply = Reply::new(DeleteAnswer.into());
    for raw_str in &req.common_args.raw_identifiers {
        let raw = RawIdentifier::new(raw_str);
        match manager.take_stopwatch_by_raw_id(&raw) {
            Ok(sw) => {
                reply.extend_successful([(
                    raw_str.clone(),
                    StopwatchDetails::from_stopwatch(&sw, req.common_args.verbose)
                )]);
            },
            Err(e) => {
                reply.extend_uncollected_errors([(Some(raw_str.clone()), e.into())]);
            }
        }
    }
    let response = JobResponse { output: reply.into() };
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    } else {
        trace!("sent stop back to user");
    }
}

/// Run a [`Manager`].
pub async fn manage(mut manager: Manager, mut req_rx: JobReceiver) {
    debug!("start manage");
    while let Some(message) = req_rx.recv().await {
        trace!("manage received message");
        let request = message.action;
        match request.specific_args {
            SpecificArgs::Start(_) => {
                start(&mut manager, &message.res_tx, &request).await;
            },
            SpecificArgs::Info(_) if request.common_args.raw_identifiers.len() == 0 => {
                info_all(&mut manager, &message.res_tx, &request).await;
            },
            SpecificArgs::Delete(_) => {
                delete(&mut manager, &message.res_tx, &request).await;
            },
            _ => {
                all(&mut manager, &message.res_tx, &request).await;
            }
        }
    }
    debug!("stop manage");
}
