use std::sync::Arc;

use futures::Future;
use tokio::sync::{
    mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel},
    oneshot
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

async fn inner_keypress_detector(sync_window: &SyncWindow) -> Option<pancurses::Input> {
    trace!("[swtui::keypress::inner_keypress_detector] entry");
    let SyncWindow(window) = sync_window;
    window.nodelay(false);
    window.keypad(true);
    trace!("[swtui::keypress::inner_keypress_detector] waiting .getch()");
    let ch = window.getch();
    trace!("[swtui::keypress::inner_keypress_detector] keypress: {:?}", ch);
    ch
}

async fn looping_keypress_detector(
    sync_window: SyncWindow,
    tx: KeypressSender,
    mut rx: StopKeypressReceiver
) {
    trace!("[swtui::keypress::looping_keypress_detector]");
    loop {
        trace!("[swtui::keypress::looping_keypress_detector] next iter");
        let ch = tokio::select! {
            _ = &mut rx => {
                trace!("[swtui::keypress::looping_keypress_detector] stop received");
                break;
            },
            ch = inner_keypress_detector(&sync_window) => {
                ch
            }
        };
        match ch {
            Some(ch) => {
                let _ = tx.send(ch);
                trace!("[swtui::keypress::looping_keypress_detector] transmitted Some");
            },
            None => {
                trace!("[swtui::keypress::looping_keypress_detector] got None");
                break;
            }
        }
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
