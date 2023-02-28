use clap::Parser;

/// Stop a stopwatch, preventing it from playing again.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List of stopwatches to stop
    pub identifiers: Vec<String>,

    /// Set verbosity to get more detailed info
    #[arg(short, long)]
    pub verbose: bool
}