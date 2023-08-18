use std::{process, sync::Arc};

use clap::Parser;
use stopwatchd::logging;

use crate::cli as cli;
use crate::keypress::keypress_detector;
use crate::ui::{Ui, color::init_color};

pub async fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);

    let pid = process::id();
    logging::setup(&format!("swtui.{}", pid), Some(cli.log_level.into()))
        .expect("could not setup logging");
    debug!("[swtui::app::start] swtui is now outputting logs");

    let ui = Ui::default();
    trace!("[swtui::app::start] initialized swtui::ui::Ui");
    init_color();
    trace!("[swtui::app::start] initialized ncurses colors");
    if !cli.show_cursor {
        pancurses::curs_set(0);
        trace!("[swtui::app::start] hiding cursor");
    }
    let (keypress_fut, mut keypress_rx, stop_keypress_tx) = keypress_detector(Arc::clone(&ui.window));
    let keypress_handle = tokio::spawn(keypress_fut);
    trace!("[swtui::app::start] spawned keypress_detector");
    debug!("[swtui::app::start] first time resetting ui");
    ui.reset();
    trace!("[swtui::app::start] awaiting a keypress to exit");
    keypress_rx.recv().await;
    trace!("[swtui::app::start] keypress received");
    stop_keypress_tx.send(()).unwrap();
    keypress_handle.await.unwrap();
    trace!("[swtui::app::start] keypress handle aborted");
}
