use clap::Parser;

use super::cli as cli;
use super::ui::Ui;

pub fn start() {
    let cli = cli::Cli::parse();
    print!("{:?}", cli);
    let ui = Ui::default();
    ui.reset();
    ui.window.getch();
}
