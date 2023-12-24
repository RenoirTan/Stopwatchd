use std::{process, sync::Arc};

use clap::Parser;
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid, pidfile_path},
    runtime::{get_uid, server_socket_path}
};
use tokio::net::UnixStream;

use crate::{
    cli,
    keypress::keypress_detector,
    ui::{Ui, color::init_color}
};

pub async fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);

    let pid = process::id();
    logging::setup(&format!("swtui.{}", pid), Some(cli.log_level.into()))
        .expect("could not setup logging");
    debug!("[swtui::app::start] swtui is now outputting logs");

    #[cfg(not(feature = "users"))]
    let uid = get_uid();
    #[cfg(feature = "users")]
    let uid = if cli.system_swd { None } else { get_uid() };

    let swd_pid = {
        let ppath = pidfile_path(uid);
        let mut pidfile = open_pidfile(false, uid)
            .expect(&format!("could not open pidfile: {:?}", ppath));
        get_swd_pid(&mut pidfile)
            .expect(&format!("could not get swd PID from {:?}", ppath))
    };
    debug!("swd_pid is {}", swd_pid);

    let ssock_path = server_socket_path(Some(swd_pid), uid);

    let mut ui = Ui::default();
    ui.refresh_list(&ssock_path).await;
    ui.list_panel_state.selected = 0;
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
            },
            pancurses::Input::Character(' ') if ui.is_focus_active() => {
                ui.toggle_state();
            },
            _ => {}
        }
        ui.draw();
        ui.refresh_list(&ssock_path).await;
    }
    trace!("[swtui::app::start] keypress received");
    stop_keypress_tx.send(()).unwrap();
    keypress_handle.await.unwrap();
    trace!("[swtui::app::start] keypress handle aborted");
}
