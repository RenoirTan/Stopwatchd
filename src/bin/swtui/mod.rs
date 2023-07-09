fn main() {
    #[cfg(feature = "swtui")]
    println!("why");
    #[cfg(not(feature = "swtui"))]
    println!("bruh");
}
