use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Names or IDs of stopwatches you want information about.
    /// If empty, then a list of all stopwatches will be returned
    pub identifiers: Vec<String>,

    /// Set verbosity to get more detailed info
    #[arg(short, long)]
    pub verbose: bool
}