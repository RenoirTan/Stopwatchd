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

 - Stopwatch `id`s now start with an `@` symbol, followed by 12 hexadecimal digits.
   - Example: `@1f2e3d4c5b6a`.
 - Names can no longer contain `@` at the front.
 - Refactor `stopwatchd::communications`, `swctl::formatter` and `swd::manager`.
 - `stopwatchd::identifiers::UuidName` is now `Identifier`.
 - What used to be `Identifier` and `UNMatchKind` has been moved from `stopwatchd::identifier` to `swd::raw_identifier` as `RawIdentifier` and `IdentifierMatch` respectively.
 - As a result the API for communicating with `swd` has changed.
 - Printouts from `swctl` are now different.
 - New `swctl start` command line flag: `--fix-bad-names`. By default, `swd` gives an error if an invalid name is given as the new name of a stopwatch. You can tell `swd` to try and fix the name such that it's not illegal. This may cause the new name to clash with another name though, which I might fix later.
 - `stopwatchd::manager` now iterates over a sequence of identifiers and maps them to an action function.
