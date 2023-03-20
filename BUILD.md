# Building Stopwatchd

A simple `cargo build` should be sufficient for testing purposes. The `swd` and `swctl` binaries should be located in `target/debug` or `target/release` depending on whether you used the `--release` flag or not.

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

## Other systemd distributions

Compile stopwatchd as per normal.

```bash
cargo build --release
```

Then, run:

```bash
scripts/mk-systemd-service
```

to create `out/stopwatchd.service`.

Then, copy the following files to their respective destinations:

```bash
install -Dm 755 target/debug/swd /usr/bin/swd
install -Dm 755 target/debug/swctl /usr/bin/swctl
install -Dm 644 README.md /usr/share/doc/stopwatchd/README.md
install -Dm 644 out/stopwatchd.service /lib/systemd/system/stopwatchd.service
install -Dm 644 res/conf/swd.conf /etc/stopwatchd/swd.conf
```