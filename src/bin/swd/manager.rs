use std::{collections::HashMap, time::SystemTime};

use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        server_message::ServerReply, start::ServerStartStopwatch
    },
    models::stopwatch::{Stopwatch, State}
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
    _stopwatches: HashMap<Uuid, Stopwatch>
}

impl Manager {
    pub fn new() -> Self {
        let _stopwatches = HashMap::new();
        Self { _stopwatches }
    }
}

pub async fn manage(mut _manager: Manager, mut req_rx: RequestReceiver) {
    debug!("start manage");
    while let Some(request) = req_rx.recv().await {
        trace!("manage received request");
        use ClientRequest::*;
        match request.action {
            Start(start) => {
                let reply = ServerStartStopwatch {
                    sw_id: Uuid::new_v4(),
                    name: start.name,
                    state: State::Playing,
                    start_time: Some(SystemTime::now())
                };
                let response = Response { output: ServerReply::Start(reply) };
                trace!("manage is sending response back for start");
                if let Err(e) = request.res_tx.send(response) {
                    error!("{}", e);
                }
            },
            Default => {
                let response = Response { output: ServerReply::Default };
                trace!("manage is sending response back for default");
                if let Err(e) = request.res_tx.send(response) {
                    error!("{}", e)
                }
            }
        }
    }
    debug!("stop manage");
}
