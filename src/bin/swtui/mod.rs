#[macro_use]
extern crate log;

#[cfg(feature = "swtui")]
mod app;
#[cfg(feature = "swtui")]
mod cli;
#[cfg(feature = "swtui")]
mod keypress;
#[cfg(feature = "swtui")]
mod ui;
#[cfg(feature = "swtui")]
mod util;

#[tokio::main]
async fn main() {
    #[cfg(feature = "swtui")]
    {
        app::start().await;
        trace!("die");
    }
    #[cfg(not(feature = "swtui"))]
    println!("NO SWTUI");
}
