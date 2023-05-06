//! Signal handling utilities.

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

/// Signals `swd` listens to.
pub const RELEVANT_SIGNALS: [i32; 4] = [SIGHUP, SIGTERM, SIGINT, SIGQUIT];

/// Create [`Signals`] from [`RELEVANT_SIGNALS`].
pub fn get_signals() -> Result<Signals, io::Error> {
    Signals::new(&RELEVANT_SIGNALS)
}

/// Send a message when a signal is detected.
pub type SignalSender = UnboundedSender<()>;
/// Receive a `()` message when a signal is detected.
pub type SignalReceiver = UnboundedReceiver<()>;

/// Captures signals and sends a message through `sender`.
/// 
/// # Arguments
/// signals - [`Signals`] we care about. Generate the relevant signals from
/// [`get_signals`].
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

/// Create a signal handler and receiver that spits out a message when a
/// relevant signal is detected.
/// 
/// Pass `handle` and `signals_task` to [`close_signal_handler`] upon
/// termination.
/// 
/// # Arguments
/// restart - An [`AtomicBool`] that `signals_task` modifies. If `restart` is
/// `true` it means `SIGHUP` was detected and `swd` should be restarted.
/// Otherwise, another signal was detected and `swd` should be terminated.
/// 
/// # Returns
/// (handle, signals_task, signal_rx)
/// 
/// handle - See [`Handle`] for more information.
/// 
/// signals_task - Async thread that listens for signals.
/// 
/// signal_rx - Messages are received through this receiver when a signal is
/// detected by signals_task.
pub fn make_signal_handler(restart: Arc<AtomicBool>) -> (Handle, JoinHandle<()>, SignalReceiver) {
    let (signal_tx, signal_rx) = unbounded_channel();
    let signals = get_signals().unwrap();
    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals, signal_tx, restart));

    (handle, signals_task, signal_rx)
}

/// Clean up `handle` and `signals_task` from `make_signal_handler`.
pub async fn close_signal_handler(handle: Handle, signals_task: JoinHandle<()>) {
    handle.close();
    signals_task.await.unwrap();
}