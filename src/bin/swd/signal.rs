use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};

use futures::stream::StreamExt;
use signal_hook::consts::signal::{SIGHUP, SIGTERM, SIGINT, SIGQUIT};
use signal_hook_tokio::Signals;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

pub const RELEVANT_SIGNALS: [i32; 4] = [SIGHUP, SIGTERM, SIGINT, SIGQUIT];

pub fn get_signals() -> Result<Signals, io::Error> {
    Signals::new(&RELEVANT_SIGNALS)
}

pub type SignalSender = UnboundedSender<()>;
pub type SignalReceiver = UnboundedReceiver<()>;

pub async fn handle_signals(mut signals: Signals, sender: SignalSender, restart: Arc<AtomicBool>) {
    while let Some(signal) = signals.next().await {
        info!("signal {} received", signal);
        match signal {
            SIGHUP => {
                let _ = sender.send(());
            },
            SIGTERM | SIGINT | SIGQUIT => {
                restart.store(false, Ordering::Relaxed);
                let _ = sender.send(());
            },
            _ => unreachable!()
        };
    }
}