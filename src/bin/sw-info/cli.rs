use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name or UUID of stopwatch.
    /// If empty, then a list of all stopwatches will be returned
    pub identifier: Option<String>,

    /// Set verbosity to get more detailed info
    #[arg(short, long)]
    pub verbose: bool
}