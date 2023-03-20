# Stopwatchd Todo List

 - [ ] Add a way for user to start their own `swd` instance instead of a public one. (i.e. `systemctl start --user` should be an option too).
 - [x] Allow `swd` to be configured from a file.
   - [x] Respond to `SIGHUP`.
   - [x] Update config on `SIGHUP`.
 - [ ] Work on `sw-attach`, maybe with ncurses.
 - [ ] Other init systems:
   - [ ] openrc
   - [ ] ...