use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Create a new stopwatch on startup. The new stopwatch will be focused
    /// if created successfully.
    #[arg(long = "new", global = true)]
    pub new_stopwatch: Option<String>,

    /// Focus on a stopwatch on startup.
    #[arg(short = 'f', long = "focus", global = true)]
    pub focus_stopwatch: Option<String>,

    /// Whether to communicate with system swd instead of user-started swd.
    #[cfg(feature = "users")]
    #[arg(long = "system", global = true)]
    pub system_swd: bool
}
