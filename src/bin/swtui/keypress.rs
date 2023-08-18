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
    trace!("swtui::keypress::inner_keypress_detector");
    let SyncWindow(window) = sync_window;
    window.nodelay(false);
    window.keypad(true);
    trace!("start listening for keypress detector");
    let ch = window.getch();
    trace!("keypress detected from window");
    ch
}

async fn looping_keypress_detector(sync_window: SyncWindow, tx: KeypressSender) {
    trace!("swtui::keypress::looping_keypress_detector");
    loop {
        match inner_keypress_detector(&sync_window).await {
            Some(ch) => {
                tx.send(ch);
                trace!("transmitted keypress");
            },
            None => {
                trace!("bye bye");
                break;
            }
        }
    }
}

pub fn keypress_detector(
    window: Arc<pancurses::Window>,
    tx: KeypressSender
) -> impl Future<Output=()> {
    looping_keypress_detector(SyncWindow(window), tx)
}
