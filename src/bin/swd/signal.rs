use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};

use futures::stream::StreamExt;
use signal_hook::consts::signal::{SIGHUP, SIGTERM, SIGINT, SIGQUIT};
use signal_hook_tokio::{Signals, Handle};
use tokio::{
    sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel},
    task::JoinHandle
};

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

pub fn make_signal_handler(restart: Arc<AtomicBool>) -> (Handle, JoinHandle<()>, SignalReceiver) {
    let (signal_tx, signal_rx) = unbounded_channel();
    let signals = get_signals().unwrap();
    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals, signal_tx, restart));

    (handle, signals_task, signal_rx)
}

pub async fn close_signal_handler(handle: Handle, signals_task: JoinHandle<()>) {
    handle.close();
    signals_task.await.unwrap();
}