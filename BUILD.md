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

PKGBUILD is WIP.

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

Then, copy all of the built files to your `/bin` and `/lib/systemd/system` folders where necessary.