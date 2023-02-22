use std::collections::HashMap;

use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply, start::{ServerStartStopwatch, ClientStartStopwatch}
    },
    models::stopwatch::{Stopwatch, UuidNameMatcher}
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
    stopwatches: HashMap<UuidNameMatcher, Stopwatch>
}

impl Manager {
    pub fn new() -> Self {
        let stopwatches = HashMap::new();
        Self { stopwatches }
    }
}

async fn start(manager: &mut Manager, res_tx: &ResponseSender, css: ClientStartStopwatch) {
    let stopwatch = Stopwatch::new(Some(css.name.clone()));
    let reply = ServerStartStopwatch::from(&stopwatch);
    manager.stopwatches.insert(stopwatch.get_matcher(), stopwatch);
    let response = Response { output: ServerReply::Start(reply) };
    trace!("manage is sending response back for start");
    if let Err(e) = res_tx.send(response) {
        error!("{}", e);
    }
    println!("stopwatches: {:?}", manager.stopwatches);
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
            Info(_cis) => {println!("info request received")},
            Default => default(&request.res_tx).await
        }
    }
    debug!("stop manage");
}
