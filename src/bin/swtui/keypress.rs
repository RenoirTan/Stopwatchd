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

async fn inner_keypress_detector<'w>(sync_window: SyncWindow, tx: KeypressSender) {
    let SyncWindow(window) = sync_window;
    window.nodelay(false);
    window.keypad(true);
    while let Some(ch) = window.getch() {
        let _ = tx.send(ch);
    }
}

pub fn keypress_detector(window: Arc<pancurses::Window>, tx: KeypressSender) -> impl Future<Output=()> {
    inner_keypress_detector(SyncWindow(window), tx)
}
