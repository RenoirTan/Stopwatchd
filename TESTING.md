## Debugging Interprocess Communication (IPC)

Stopwatchd uses the Concise Binary Object Representation (CBOR) format by default for IPC between `swd` and `swctl`. You can see the messages being sent to and from `swctl` by passing the `--debug-ipc` flag. The `debug-ipc` cargo feature must be explicitly enabled for this to work.
