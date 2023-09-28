use std::{process, sync::Arc};

use clap::Parser;
use stopwatchd::logging;

use crate::cli as cli;
use crate::keypress::keypress_detector;
use crate::ui::list_panel::ListPanelState;
use crate::ui::{Ui, color::init_color};

pub async fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);

    let pid = process::id();
    logging::setup(&format!("swtui.{}", pid), Some(cli.log_level.into()))
        .expect("could not setup logging");
    debug!("[swtui::app::start] swtui is now outputting logs");

    let mut ui = Ui::default();
    ui.list_panel_state = ListPanelState::generate_fake_names(100);
    ui.list_panel_state.selected = 10;
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
    ui.draw();
    trace!("[swtui::app::start] awaiting F10 to exit");
    while let Some(ch) = keypress_rx.recv().await {
        match ch {
            pancurses::Input::KeyF10 => {
                break;
            },
            pancurses::Input::KeyLeft => {
                ui.set_focus_active(false);
            },
            pancurses::Input::KeyRight => {
                ui.set_focus_active(true);
            },
            // when active window is list panel
            pancurses::Input::KeyDown if !ui.is_focus_active() => {
                ui.scroll(false);
            },
            pancurses::Input::KeyUp if !ui.is_focus_active() => {
                ui.scroll(true);
            },
            pancurses::Input::KeyHome if !ui.is_focus_active() => {
                ui.scroll_home();
            },
            pancurses::Input::KeyEnd if !ui.is_focus_active() => {
                ui.scroll_end();
            }
            _ => {}
        }
        ui.draw();
    }
    trace!("[swtui::app::start] keypress received");
    stop_keypress_tx.send(()).unwrap();
    keypress_handle.await.unwrap();
    trace!("[swtui::app::start] keypress handle aborted");
}
