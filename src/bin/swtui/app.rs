use std::sync::Arc;

use clap::Parser;

use crate::cli as cli;
use crate::keypress::{make_keypress_channels, keypress_detector};
use crate::ui::{Ui, color::init_color};

pub async fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);
    let ui = Ui::default();
    init_color();
    if !cli.show_cursor {
        pancurses::curs_set(0);
    }
    let (keypress_tx, mut keypress_rx) = make_keypress_channels();
    let keypress_handle = tokio::spawn(keypress_detector(Arc::clone(&ui.window), keypress_tx));
    ui.reset();
    keypress_rx.recv().await;
    let _ = keypress_handle.abort();
}
