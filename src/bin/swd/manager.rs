use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply, start::{ServerStartStopwatch, ClientStartStopwatch}, info::{ClientInfoStopwatch, ServerInfoStopwatch, ServerInfoStopwatchInner}
    },
    models::stopwatch::{Stopwatch, UNMatchKind, FindStopwatchError}
};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

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
    stopwatches: Vec<Stopwatch> // Last item is most recently accessed
}

impl Manager {
    pub fn new() -> Self {
        let stopwatches = Vec::new();
        Self { stopwatches }
    }

    pub fn add_stopwatch(&mut self, stopwatch: Stopwatch) {
        self.stopwatches.push(stopwatch);
    }

    fn get_stopwatches_indices_by_identifier(
        &self,
        identifier: &str,
        only_one: bool
    ) -> (Vec<usize>, UNMatchKind) {
        let mut name_matches = vec![];
        let mut uuid_matches = vec![];
        // Reverse because last item is the most recently accessed
        'a: for index in (0..self.stopwatches.len()).rev() {
            let match_kind = self.stopwatches[index].matches_identifier(identifier);
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
    ) -> Result<Stopwatch, FindStopwatchError> {
        let (indices, _) = self.get_stopwatches_indices_by_identifier(identifier, false);
        if indices.len() != 1 {
            return Err(self.manager_stopwatches_error(identifier.to_string(), &indices));
        } else {
            Ok(self.stopwatches.remove(indices[0]))
        }
    }

    fn manager_stopwatches_error(
        &self,
        identifier: String,
        indices: &[usize]
    ) -> FindStopwatchError {
        let mut duplicates = vec![];
        for index in indices {
            let stopwatch = &self.stopwatches[*index];
            duplicates.push((stopwatch.id, stopwatch.name.clone()));
        }
        FindStopwatchError { identifier, duplicates }
    }
}

async fn start(manager: &mut Manager, res_tx: &ResponseSender, css: ClientStartStopwatch) {
    let stopwatch = Stopwatch::new(Some(css.name.clone()));
    let reply = ServerStartStopwatch::from(&stopwatch);
    manager.add_stopwatch(stopwatch);
    let response = Response { output: ServerReply::Start(reply) };
    trace!("manage is sending response back for start");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    }
    println!("stopwatches: {:?}", manager.stopwatches);
}

async fn info(manager: &mut Manager, res_tx: &ResponseSender, cis: ClientInfoStopwatch) {
    match manager.get_stopwatch_by_identifier(&cis.identifier) {
        Ok(sw) => {
            let reply = ServerInfoStopwatchInner::from_stopwatch(&sw, cis.verbose).into();
            let response = Response { output: reply };
            if let Err(e) = res_tx.send(response) {
                error!("{}", e);
            }
            manager.add_stopwatch(sw);
        },
        Err(fse) => {
            let response = Response {
                output: ServerReply::Info(ServerInfoStopwatch { info: Err(fse) })
            };
            if let Err(e) = res_tx.send(response) {
                error!("{}", e);
            }
        }
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
    while let Some(request) = req_rx.recv().await {
        trace!("manage received request");
        use ClientRequest::*;
        match request.action {
            Start(css) => start(&mut manager, &request.res_tx, css).await,
            Info(cis) => info(&mut manager, &request.res_tx, cis).await,
            Default => default(&request.res_tx).await
        }
    }
    debug!("stop manage");
}
