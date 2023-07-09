#[cfg(feature = "swtui")]
mod app;
#[cfg(feature = "swtui")]
mod cli;

fn main() {
    #[cfg(feature = "swtui")]
    app::start();
    #[cfg(not(feature = "swtui"))]
    println!("bruh");
}
