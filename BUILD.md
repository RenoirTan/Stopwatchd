# Building Stopwatchd

A simple `cargo build` should be sufficient for testing purposes. The `swd` and `swctl` binaries should be located in `target/debug` or `target/release` depending on whether you used the `--release` flag or not.

Packaging stopwatchd can be done using `./scripts/package`. It's a bash file that builds the binaries, sets up configuration and other data files, and installs them to the `$pkgdir` directory. Environment variables can be set to modify the behaviour of `./scripts/package`, more details can be found in `./scripts/_common`. For example:

```bash
# pkgdir has to be an absolute path
pkgdir="$(pwd)/pkg"

scripts/package # dump build artefacts in ${pkgdir}

# collect and compress everything into one file
cd ${pkgdir}
tar -czvf stopwatchd.tar.gz .
```

## Debian

Install cargo-deb:

```bash
cargo install cargo-deb
```

Then run:

```bash
scripts/pkg-debian
```

to get a `.deb` package. This package contains a systemd `.service` file and so it may not be useful for Devuan users.

## Arch Linux

Clone the PKGBUILDs from [StopwatchdABS](https://github.com/RenoirTan/StopwatchdABS) and then run `makepkg` in the appropriate directory (depending on if you want the release or git version).

```bash
git clone https://github.com/RenoirTan/StopwatchdABS.git
cd StopwatchdABS/stopwatchd # or stopwatchd-git
makepkg -si # make and then install the package
```

## Other distributions

An example:

```bash
pkgdir=/
prefix=/usr/local
features=users # comma-separated list of cargo features

sudo ./scripts/package
```