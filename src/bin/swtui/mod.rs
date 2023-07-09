#[cfg(feature = "swtui")]
mod app;
#[cfg(feature = "swtui")]
mod cli;
#[cfg(feature = "swtui")]
mod ui;

fn main() {
    #[cfg(feature = "swtui")]
    app::start();
    #[cfg(not(feature = "swtui"))]
    println!("bruh");
}
