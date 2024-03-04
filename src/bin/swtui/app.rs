use std::{process, sync::Arc};

use clap::Parser;
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid, pidfile_path},
    runtime::{get_uid, server_socket_path}
};

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

    // TODO: make this optional
    logging::set_panic_hook();

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
    ui.ssock_path = ssock_path;
    ui.refresh_list().await;
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
        if ui.prompt_state.visible {
            match ch {
                // ESC
                // TODO: make sure this is OS-agnostic
                pancurses::Input::Character('\u{1b}') => {
                    ui.prompt_state.visible = false;
                },
                pancurses::Input::Character('\n') => {
                    ui.prompt_state.visible = false;
                    ui.start_stopwatch().await;
                    ui.prompt_state.reset();
                },
                pancurses::Input::Character(c) => {
                    ui.prompt_state.add_char(c);
                },
                pancurses::Input::KeyBackspace => {
                    ui.prompt_state.backspace();
                }
                _ => {} // TODO: WHAT
            }
        } else {
            match ch {
                pancurses::Input::KeyF9 => {
                    panic!("[swtui::app::start] F9");
                },
                pancurses::Input::KeyF10 => {
                    break;
                },
                pancurses::Input::KeyLeft => {
                    ui.set_focus_active(false).await;
                },
                pancurses::Input::KeyRight => {
                    ui.set_focus_active(true).await;
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
                    ui.toggle_state().await;
                },
                pancurses::Input::Character('n') if !ui.is_focus_active() => {
                    ui.prompt_name();
                },
                pancurses::Input::Character('s') if ui.is_focus_active() => {
                    ui.stop_stopwatch().await;
                },
                pancurses::Input::Character('d') if ui.is_focus_active() => {
                    ui.delete_stopwatch().await;
                }
                _ => {}
            }
        }
        ui.draw();
        ui.refresh_list().await;
        ui.refresh_stopwatch().await;
    }
    trace!("[swtui::app::start] keypress received");
    stop_keypress_tx.send(()).unwrap();
    keypress_handle.await.unwrap();
    trace!("[swtui::app::start] keypress handle aborted");
}
