use clap::Parser;

use crate::cli as cli;
use crate::ui::{Ui, color::init_color};

pub fn start() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);
    let ui = Ui::default();
    init_color();
    if !cli.show_cursor {
        pancurses::curs_set(0);
    }
    ui.reset();
    ui.window.getch();
}
