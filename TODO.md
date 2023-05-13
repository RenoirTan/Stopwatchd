# Stopwatchd Todo List

 - [x] Add a way for user to start their own `swd` instance instead of a public one. (i.e. `systemctl start --user` should be an option too).
   - [x] `swctl` can communicate with either `sudo swd` or `swd`.
 - [x] Allow `swd` to be configured from a file.
   - [x] Respond to `SIGHUP`.
   - [x] Update config on `SIGHUP`.
 - [ ] Refactoring
   - [x] Reduce the number of files under `stopwatchd::communication`.
   - [x] Simplify the API for `swctl::formatted`.
 - [ ] Work on `sw-attach`, maybe with ncurses.
 - [ ] Other init systems:
   - [ ] openrc
   - [ ] ...