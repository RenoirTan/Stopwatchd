# Testing Stopwatchd

The current way I'm doing integration tests is to create a package for a
distro, installing it and then running it because it's easier than figuring out
how to hook Stopwatchd up to `autotools` and running `make install`.

For now, a `.deb` file can be created for Stopwatchd using `scripts/pkg-debian`.
The script requires that you have `cargo-deb` installed.

Arch Linux users can create a .pkg.tar.zst from the PKGBUILDs in [this repo](https://github.com/RenoirTan/StopwatchdABS).