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
    debug!("[swtui::app::start] swtui is now outputting logs");

    let ui = Ui::default();
    trace!("[swtui::app::start] initialized swtui::ui::Ui");
    init_color();
    trace!("[swtui::app::start] initialized ncurses colors");
    if !cli.show_cursor {
        pancurses::curs_set(0);
        trace!("[swtui::app::start] hiding cursor");
    }
    let (keypress_tx, mut keypress_rx) = make_keypress_channels();
    let keypress_handle = tokio::spawn(keypress_detector(Arc::clone(&ui.window), keypress_tx));
    trace!("[swtui::app::start] spawned keypress_detector");
    debug!("[swtui::app::start] first time resetting ui");
    ui.reset();
    trace!("[swtui::app::start] awaiting a keypress to exit");
    keypress_rx.recv().await;
    trace!("[swtui::app::start] keypress received");
    let _ = keypress_handle.abort();
    // let _ = keypress_handle.await;
    trace!("[swtui::app::start] keypress handle aborted");
}
