# Stopwatchd Changelog

## v0.1.0

 - `swd` and `swctl`.
 - systemd `.service` file.
 - Packaging for Debian and Arch Linux.

## v0.2.0

 - Added cargo feature `swd-config` which enables:
   - Allow `swd` to read from a configuration file (by default in `/etc/stopwatchd/swd.conf`).
   - Make `swd` respond to `SIGHUP` to reload the configuration file.