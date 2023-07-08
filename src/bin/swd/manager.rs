//! Manages the stopwatch.

use std::{
    collections::{HashMap, hash_map::Entry},
    ops
};

use stopwatchd::{
    communication::{
        client::Request,
        server::{Reply, ServerError},
        reply_specifics::*,
        details::StopwatchDetails,
        request_specifics::SpecificArgs
    },
    models::stopwatch::{Stopwatch, State},
    error::{FindStopwatchError, InvalidState},
    identifiers::{Identifier, UniqueId, Name}
};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

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

// (state, raw identifier, stopwatch if found)
pub type ActionGetStopwatch = fn(&mut ActionState, String, Option<&Stopwatch>);
pub type ActionGetMutStopwatch = fn(&mut ActionState, String, Option<&mut Stopwatch>);
pub type ActionTakeStopwatch = fn(&mut ActionState, String, Option<Stopwatch>);

/// State that gets passed to any of the "Action" functions
pub struct ActionState<'rq> {
    pub reply: Reply,
    pub request: &'rq Request
}

impl<'rq> ActionState<'rq> {
    pub fn new(reply: Reply, request: &'rq Request) -> Self {
        Self { reply, request }
    }
}

/// Contains [`Stopwatch`]es and auxiliary info.
/// 
/// Use [`manage`] to run the manager.
pub struct Manager {
    stopwatches: HashMap<UniqueId, Stopwatch>,
    access_order: AccessOrder,
    name_registry: NameRegistry
}

impl Manager {
    pub fn new() -> Self {
        Self {
            stopwatches: HashMap::new(),
            access_order: AccessOrder::new(),
            name_registry: NameRegistry::new()
        }
    }

    pub fn iter_access_order_id(&self) -> impl Iterator<Item = &UniqueId> {
        self.access_order.iter()
    }

    pub fn add_stopwatch(&mut self, stopwatch: Stopwatch) -> Result<(), Identifier> {
        let id = stopwatch.identifier.id;
        match self.stopwatches.entry(id) {
            Entry::Occupied(o) => Err(o.get().identifier.clone()),
            Entry::Vacant(v) => {
                match self.name_registry.register(&stopwatch.identifier) {
                    Ok(()) => {
                        self.access_order.access_stopwatch(stopwatch.identifier.id);
                        v.insert(stopwatch);
                        Ok(())
                    },
                    Err(oid) => Err(Identifier::new(oid, stopwatch.identifier.name.clone()))
                }
            }
        }
    }

    pub fn get_stopwatch(
        &mut self,
        raw_identifier: &RawIdentifier
    ) -> Option<(&Stopwatch, IdentifierMatch)> {
        let (id, match_kind) = self.get_id(raw_identifier)?;
        let sw = self.stopwatches.get(&id)?;
        self.access_order.access_stopwatch(id);
        Some((sw, match_kind))
    }

    pub fn get_mut_stopwatch(
        &mut self,
        raw_identifier: &RawIdentifier
    ) -> Option<(&mut Stopwatch, IdentifierMatch)> {
        let (id, match_kind) = self.get_id(raw_identifier)?;
        let sw = self.stopwatches.get_mut(&id)?;
        self.access_order.access_stopwatch(id);
        Some((sw, match_kind))
    }

    pub fn take_stopwatch(
        &mut self,
        raw_identifier: &RawIdentifier
    ) -> Option<(Stopwatch, IdentifierMatch)> {
        let (id, match_kind) = self.get_id(raw_identifier)?;
        let sw = self.stopwatches.remove(&id)?;
        self.access_order.delete_stopwatch(id);
        let _ = self.name_registry.delete(&sw.identifier);
        Some((sw, match_kind))
    }

    fn get_id(&self, raw_identifier: &RawIdentifier) -> Option<(UniqueId, IdentifierMatch)> {
        match raw_identifier.clone().to_possible_id_or_name() {
            Ok(id) => Some((id, IdentifierMatch::Uuid)),
            Err(name) => Some((self.name_registry.get(&name)?, IdentifierMatch::Name))
        }
    }

    pub fn get_all_stopwatches_and(&self, state: &mut ActionState<'_>, action: ActionGetStopwatch) {
        for id in self.iter_access_order_id() {
            let sw = self.stopwatches.get(id).unwrap();
            action(state, sw.identifier.to_string(), Some(sw)) // guaranteed to be Some, panic if None
        }
    } 
 
    pub fn get_stopwatches_and<I>(
        &mut self,
        state: &mut ActionState<'_>,
        identifiers: I,
        action: ActionGetStopwatch
    )
    where
        I: IntoIterator<Item = String>
    {
        for raw_str in identifiers.into_iter() {
            let raw_identifier = RawIdentifier::new(raw_str.clone());
            let sw = self.get_stopwatch(&raw_identifier).map(|(sw, _mk)| sw);
            action(state, raw_str, sw);
        }
    }

    pub fn get_mut_stopwatches_and<I>(
        &mut self,
        state: &mut ActionState,
        identifiers: I,
        action: ActionGetMutStopwatch
    )
    where
        I: IntoIterator<Item = String>
    {
        for raw_str in identifiers.into_iter() {
            let raw_identifier = RawIdentifier::new(raw_str.clone());
            let sw = self.get_mut_stopwatch(&raw_identifier).map(|(sw, _mk)| sw);
            action(state, raw_str, sw);
        }
    }

    pub fn take_stopwatches_and<I>(
        &mut self,
        state: &mut ActionState,
        identifiers: I,
        action: ActionTakeStopwatch
    )
    where
        I: IntoIterator<Item = String>
    {
        for raw_str in identifiers.into_iter() {
            let raw_identifier = RawIdentifier::new(raw_str.clone());
            let sw = self.take_stopwatch(&raw_identifier).map(|(sw, _mk)| sw);
            action(state, raw_str, sw);
        }
    }
}

/// Associates each [`Name`] with a [`UniqueId`].
#[derive(Clone, Debug)]
pub struct NameRegistry {
    pub registry: HashMap<Name, UniqueId>
}

impl NameRegistry {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register(&mut self, identifier: &Identifier) -> Result<(), UniqueId> {
        let name = identifier.name.clone();
        let id = identifier.id;
        match self.registry.entry(name) {
            Entry::Occupied(o) => Err(*o.get()),
            Entry::Vacant(v) => { v.insert(id); Ok(()) }
        }
    }

    pub fn get(&self, name: &Name) -> Option<UniqueId> {
        self.registry.get(name).map(|id| *id)
    }

    pub fn delete(&mut self, identifier: &Identifier) -> Result<usize, UniqueId> {
        match self.registry.remove(&identifier.name) {
            Some(id) if id != identifier.id => Err(id),
            Some(_) => Ok(1),
            None => Ok(0)
        }
    }
}

impl Default for NameRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Order in which [`Stopwatch`]es were last accessed.
pub struct AccessOrder {
    pub order: Vec<UniqueId>
}

impl AccessOrder {
    pub fn new() -> Self {
        Self { order: vec![] }
    }

    pub fn iter(&self) -> impl Iterator<Item = &UniqueId> {
        self.order.iter().rev()
    } 

    pub fn access_stopwatch(&mut self, id: UniqueId) -> Option<usize> {
        let index = self.delete_stopwatch(id);
        self.order.push(id);
        index
    }

    pub fn delete_stopwatch(&mut self, id: UniqueId) -> Option<usize> {
        let index = self.order.iter().position(|u| *u == id);
        if let Some(i) = index {
            self.order.remove(i);
        }
        index
    }
}

impl ops::Deref for AccessOrder {
    type Target = Vec<UniqueId>;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

/// Manager function to start a [`Stopwatch`].
async fn start(manager: &mut Manager, req: &Request) -> Reply {
    // Create a new reply. Can populate with success or error later.
    let mut reply = Reply::new(StartAnswer.into());

    // Retrieve arguments.
    let start_args = match req.specific_args {
        SpecificArgs::Start(ref sa) => sa,
        _ => panic!("fuck")
    };
    let given_name = req.common_args.raw_identifiers.first().cloned()
        .unwrap_or_else(|| String::new());
    
    // Calculate name of the new stopwatch.
    let name: Option<Name> = if start_args.fix_bad_names {
        Some(Name::fixed(given_name.clone()))
    } else {
        match Name::new(given_name.clone()) {
            Ok(n) => Some(n),
            Err(e) => {
                reply.extend_uncollected_errors(
                    [(Some(given_name.clone()), ServerError::BadName(e))]
                );
                None
            }
        }
    };

    // Create new stopwatch.
    if let Some(name) = name {
        // Start stopwatch first, delete if need be
        let stopwatch = Stopwatch::start(name.clone());
        let details = StopwatchDetails::from_stopwatch(&stopwatch, req.common_args.verbose);

        match manager.add_stopwatch(stopwatch) {
            Ok(()) => {
                reply.extend_successful([(Into::<String>::into(name), details)]);
            },
            Err(identifier) => {
                trace!("stopwatch with the same name or uuid already exists");
                let error = FindStopwatchError {
                    raw_identifier: given_name.clone(),
                    duplicates: vec![identifier]
                };
                reply.extend_uncollected_errors([(Some(given_name), error.into())]);
            }
        }
    }

    reply
}

fn not_found(reply: &mut Reply, raw_identifier: String) {
    reply.add_errors([FindStopwatchError { raw_identifier, duplicates: vec![] }.into()]);
}

fn info_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<&Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let verbose = request.common_args.verbose;
            reply.extend_successful([(raw_id, StopwatchDetails::from_stopwatch(sw, verbose))]);
            if let SpecificAnswer::Info(InfoAnswer::All(ref mut all)) = reply.specific_answer {
                all.access_order.push(sw.identifier.to_string())
            }
        },
        None => {
            not_found(reply, raw_id);
        }
    }
}

/// Short for add_to_reply_maybe_invalid_state
///
/// Helper function for [`all`].
/// 
/// If error condition is met, add an [`InvalidState`] error to `errored`.
/// Otherwise, add a [`StopwatchDetails`] to `details` to signify success.
/// 
/// # Arguments
/// * raw_identifier - Raw identifier used to match a stopwatch.
/// * stopwatch - [`Stopwatch`] object.
/// * details - Map of [`StopwatchDetails`], stores success messages.
/// * errored - Map of [`ServerError`], stores error messages.
/// * error_condition - Error condition. See the description of this function.
fn atrmis(
    reply: &mut Reply,
    raw_identifier: String,
    stopwatch: &Stopwatch,
    verbose: bool,
    original_state: State,
    error_condition: bool
) {
    if error_condition {
        reply.extend_uncollected_errors([(
            Some(raw_identifier.clone()),
            InvalidState { raw_identifier, state: original_state }.into()
        )]);
    } else {
        reply.extend_successful([(
            raw_identifier,
            StopwatchDetails::from_stopwatch(stopwatch, verbose)
        )]);
    }
}

fn stop_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<&mut Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let verbose = request.common_args.verbose;
            let state = sw.end();
            atrmis(reply, raw_id, sw, verbose, state, state == State::Ended);
        },
        None => not_found(reply, raw_id)
    }
}

fn play_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<&mut Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let v = request.common_args.verbose;
            let state = sw.play();
            atrmis(reply, raw_id, sw, v, state, matches!(state, State::Playing | State::Ended));
        },
        None => not_found(reply, raw_id)
    }
}

fn pause_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<&mut Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let v = request.common_args.verbose;
            let state = sw.pause();
            atrmis(reply, raw_id, sw, v, state, matches!(state, State::Paused | State::Ended));
        },
        None => not_found(reply, raw_id)
    }
}

fn lap_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<&mut Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let v = request.common_args.verbose;
            let state = sw.new_lap(true);
            atrmis(reply, raw_id, sw, v, state, state.ended());
        },
        None => not_found(reply, raw_id)
    }
}

fn delete_action<'rq>(state: &mut ActionState<'rq>, raw_id: String, sw: Option<Stopwatch>) {
    let ActionState { reply, request } = state;
    match sw {
        Some(sw) => {
            let v = request.common_args.verbose;
            atrmis(reply, raw_id, &sw, v, sw.state(), false);
        },
        None => not_found(reply, raw_id)
    }
}

/// Run a [`Manager`].
pub async fn manage(mut manager: Manager, mut req_rx: JobReceiver) {
    debug!("start manage");
    while let Some(message) = req_rx.recv().await {
        trace!("manage received message");
        let req = message.action;
        let identifiers = &req.common_args.raw_identifiers;
        let reply = match req.specific_args {
            SpecificArgs::Start(_) => start(&mut manager, &req).await,
            SpecificArgs::Info(_) => if identifiers.len() == 0 {
                Reply::new(InfoAnswer::All(InfoAll::default()).into())
            } else {
                Reply::new(InfoAnswer::Basic.into())
            },
            SpecificArgs::Stop(_) => Reply::new(StopAnswer.into()),
            SpecificArgs::Play(_) => Reply::new(PlayAnswer.into()),
            SpecificArgs::Pause(_) => Reply::new(PauseAnswer.into()),
            SpecificArgs::Lap(_) => Reply::new(LapAnswer.into()),
            SpecificArgs::Delete(_) => Reply::new(DeleteAnswer.into())
        };
        let mut state = ActionState::new(reply, &req);
        let raw_ids = identifiers.iter().map(String::clone);
        match req.specific_args {
            SpecificArgs::Start(_) => {},
            SpecificArgs::Info(_) => if identifiers.len() == 0 {
                manager.get_all_stopwatches_and(&mut state, info_action);
            } else {
                manager.get_stopwatches_and(&mut state, raw_ids, info_action);
            },
            SpecificArgs::Stop(_) => manager.get_mut_stopwatches_and(&mut state, raw_ids, stop_action),
            SpecificArgs::Play(_) => manager.get_mut_stopwatches_and(&mut state, raw_ids, play_action),
            SpecificArgs::Pause(_) => manager.get_mut_stopwatches_and(&mut state, raw_ids, pause_action),
            SpecificArgs::Lap(_) => manager.get_mut_stopwatches_and(&mut state, raw_ids, lap_action),
            SpecificArgs::Delete(_) => manager.take_stopwatches_and(&mut state, raw_ids, delete_action)
        }
        if let Err(e) = message.res_tx.send(JobResponse { output: state.reply.into() }) {
            error!("{}", e);
        } else {
            debug!("manage just handled a request and sent back a response");
        }
    }
    debug!("stop manage");
}
