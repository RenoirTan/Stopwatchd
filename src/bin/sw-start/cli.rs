use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the new stopwatch, at most 6 (ASCII) characters long
    pub name: Option<String>,

    /// Set verbosity to get more detailed info
    #[arg(short, long)]
    pub verbose: bool
}