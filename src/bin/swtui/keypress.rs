use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration
};

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

struct KeypressDetector {
    inner: Arc<Mutex<KeypressDetectorInner>>
}

struct KeypressDetectorInner {
    pub sync_window: SyncWindow,
    pub ch: Option<pancurses::Input>
}

impl KeypressDetector {
    fn new(sync_window: SyncWindow) -> Self {
        let inner = Arc::new(Mutex::new(KeypressDetectorInner { sync_window, ch: None }));
        Self { inner }
    }
}

impl Future for KeypressDetector {
    type Output = pancurses::Input;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        match inner.ch.take() {
            None => {
                inner.sync_window.0.nodelay(true);
                inner.sync_window.0.keypad(true);
                inner.ch = inner.sync_window.0.getch();
                // wait 10ms before waking up again
                // unfortunately i don't think there is a way for pancurses to
                // generate interrupts for keypresses
                // so i have to use a timer to check back occasionally
                let waker = cx.waker().clone();
                let duration = Duration::from_millis(10);
                thread::spawn(move || {
                    thread::sleep(duration);
                    waker.wake();
                });
                Poll::Pending
            },
            Some(ch) => {
                Poll::Ready(ch)
            }
        }
    }
}

fn detect_keypress(window: Arc<pancurses::Window>) -> impl Future<Output = pancurses::Input> {
    KeypressDetector::new(SyncWindow(window))
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
            ch = detect_keypress(sync_window.0.clone()) => {
                ch
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
