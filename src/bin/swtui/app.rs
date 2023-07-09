use clap::Parser;

use super::cli as cli;

pub fn start() {
    let cli = cli::Cli::parse();
    print!("{:?}", cli);
}
