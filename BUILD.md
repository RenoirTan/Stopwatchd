# Building Stopwatchd

A simple `cargo build` should be sufficient for testing purposes. The `swd` and `swctl` binaries should be located in `target/debug` or `target/release` depending on whether you used the `--release` flag or not.

Building and packaging stopwatchd can be done using `./scripts/build` and `./scripts/package` respectively. `package` is a bash script that installs build artefacts to the `$pkgdir` directory. Environment variables can be set to modify the behaviour of these scripts, more details can be found in `./scripts/_common`. For example:

```bash
# pkgdir has to be an absolute path
pkgdir="$(pwd)/pkg"

scripts/build # create build artefacts
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
features='users,swd-config' # comma-separated list of cargo features

./scripts/build
sudo ./scripts/package
```