use std::{
    future::Future,
    sync::Arc,
    time::Duration
};

use tokio::{
    sync::{
        mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel},
        oneshot
    },
    time::sleep,
};

pub type KeypressSender = UnboundedSender<pancurses::Input>;
pub type KeypressReceiver = UnboundedReceiver<pancurses::Input>;

fn make_keypress_channels() -> (KeypressSender, KeypressReceiver) {
    unbounded_channel()
}

pub type StopKeypressSender = oneshot::Sender<()>;
pub type StopKeypressReceiver = oneshot::Receiver<()>;

fn make_stop_keypress_channels() -> (StopKeypressSender, StopKeypressReceiver) {
    oneshot::channel()
}

struct SyncWindow(Arc<pancurses::Window>);

unsafe impl Sync for SyncWindow { }
unsafe impl Send for SyncWindow { }

fn waiter(sync_window: SyncWindow, inner_tx: KeypressSender) {
    loop {
        let ch = sync_window.0.getch().unwrap();
        let _ = inner_tx.send(ch);
    }
}

async fn looping_keypress_detector(
    sync_window: SyncWindow,
    tx: KeypressSender,
    mut rx: StopKeypressReceiver
) {
    trace!("[swtui::keypress::looping_keypress_detector]");
    let (inner_tx, mut inner_rx) = make_keypress_channels();
    let _detector = std::thread::spawn(move || waiter(sync_window, inner_tx));
    loop {
        trace!("[swtui::keypress::looping_keypress_detector] next iter");
        let ch = tokio::select! {
            _ = &mut rx => {
                trace!("[swtui::keypress::looping_keypress_detector] stop received");
                break;
            },
            ch = inner_rx.recv() => {
                ch.unwrap()
            }
        };
        let _ = tx.send(ch);
        trace!("[swtui::keypress::looping_keypress_detector] transmitted {:?}", ch);
    }
    trace!("[swtui::keypress::looping_keypress_detector] exiting");
}

pub fn keypress_detector(
    window: Arc<pancurses::Window>
) -> (impl Future<Output=()>, KeypressReceiver, StopKeypressSender) {
    trace!("[swtui::keypress::keypress_detector]");
    let (kp_tx, kp_rx) = make_keypress_channels();
    let (stop_tx, stop_rx) = make_stop_keypress_channels();
    (looping_keypress_detector(SyncWindow(window), kp_tx, stop_rx), kp_rx, stop_tx)
}


pub async fn keypress_timeout() {
    sleep(Duration::from_millis(100)).await;
}