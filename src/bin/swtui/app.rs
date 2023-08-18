use std::{process, sync::Arc};

use clap::Parser;
use stopwatchd::logging;

use crate::cli as cli;
use crate::keypress::{make_keypress_channels, keypress_detector};
use crate::ui::{Ui, color::init_color};

pub async fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);

    let pid = process::id();
    logging::setup(&format!("swtui.{}", pid), Some(cli.log_level.into()))
        .expect("could not setup logging");
    debug!("swtui is now outputting logs");

    let ui = Ui::default();
    trace!("initialized swtui::ui::Ui");
    init_color();
    trace!("initialized ncurses colors");
    if !cli.show_cursor {
        pancurses::curs_set(0);
        trace!("hiding cursor");
    }
    let (keypress_tx, mut keypress_rx) = make_keypress_channels();
    let keypress_handle = tokio::spawn(keypress_detector(Arc::clone(&ui.window), keypress_tx));
    trace!("spawned keypress_detector");
    debug!("first time resetting ui");
    ui.reset();
    trace!("awaiting a keypress to exit");
    keypress_rx.recv().await;
    trace!("keypress received");
    let _ = keypress_handle.abort();
    let _ = keypress_handle.await;
    trace!("keypress handle aborted");
}
