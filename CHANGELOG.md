# Stopwatchd Changelog

## v0.1.0

 - `swd` and `swctl`.
 - systemd `.service` file.
 - Packaging for Debian and Arch Linux.

## v0.2.0

 - Added cargo feature `swd-config` which enables:
   - Allow `swd` to read from a configuration file (by default in `/etc/stopwatchd/swd.conf`).
   - Make `swd` respond to `SIGHUP` to reload the configuration file.

## v0.3.0

 - Allow each user to have their own `swd` session.
 - `swctl` can choose to either communicate with the user-started `swd` or the system `swd` using the `--root` flag.
 - Add Debian maintainer scripts

## v0.4.0

 - Refactor `stopwatchd::communications` and `swctl::formatter`.
 - As a result the API for communicating with `swd` has changed.
 - Printouts from `swctl` are now different.