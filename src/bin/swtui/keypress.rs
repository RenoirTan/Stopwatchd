use std::sync::Arc;

use futures::Future;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

pub type KeypressSender = UnboundedSender<pancurses::Input>;
pub type KeypressReceiver = UnboundedReceiver<pancurses::Input>;

pub fn make_keypress_channels() -> (KeypressSender, KeypressReceiver) {
    unbounded_channel()
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

async fn looping_keypress_detector(sync_window: SyncWindow, tx: KeypressSender) {
    trace!("[swtui::keypress::looping_keypress_detector]");
    loop {
        trace!("[swtui::keypress::looping_keypress_detector] next iter");
        match inner_keypress_detector(&sync_window).await {
            Some(ch) => {
                let _ = tx.send(ch);
                trace!("[swtui::keypress::looping_keypress_detector] transmitted keypress");
            },
            None => {
                trace!("[swtui::keypress::looping_keypress_detector] bye bye");
                break;
            }
        }
    }
    trace!("[swtui::keypress::looping_keypress_detector] exiting");
}

pub fn keypress_detector(
    window: Arc<pancurses::Window>,
    tx: KeypressSender
) -> impl Future<Output=()> {
    looping_keypress_detector(SyncWindow(window), tx)
}
