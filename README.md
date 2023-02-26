# Stopwatchd

Command-line stopwatch daemon and controller for Unix-like systems.

## Usage

Stopwatchd consists of a daemon called `swd` to manage active stopwatches. `sw` is the main command line utility to interact with `swd`. Do note that each of `sw`'s subcommands has a corresponding binary whose name is `sw-` and the subcommand's name behind. See below for more examples.

```bash
sw start
# abcdef
```

`sw start` (or `sw-start`) creates a new stopwatch and prints out the name of the new stopwatch. In this case, we will be using abcdef as the name of the stopwatch for the following examples. You can provide a name for the stopwatch using `sw start [name]`, but the command will error out if that name has already been taken.

```bash
sw info abcdef
# 300
```

`sw info` (or `sw-info`) queries `swd` for information from a stopwatch (or even a bunch of stopwatches). By default, the time printed out will be in terms of seconds; so after 5 minutes, running `sw info abcdef` should show you 300.

```bash
sw lap abcdef
# 1 310
```

`sw lap` or (`sw-lap`) adds a lap to the specified stopwatch and echoes the number of laps completed and time elapsed since the stopwatch started and when the last lap was completed (in this example: 310 seconds).

```bash
sw stop abcdef
# 360
```

`sw stop` or (`sw-stop`) stops the stopwatch and print the time elapsed from start to finish.

```bash
sw info abcdef --all
# Duration
# 360
# Laps
# 1> 310
```

`sw info [name] --all` prints out more verbose information for a particularly stopwatch.

```bash
sw info abcdef --all --format '%m:%s'
# Duration
# 6:00
# Laps
# 1> 5:10
```

The `--format` argument allows you to set a datetime format for any time values outputted to stdout.

```bash
sw del abcdef
# abcdef
```

To delete a stopwatch from memory and disk, use `sw del` (or `sw-del`).

```bash
sw start
# bcdefg
sw attach bcdefg
# Press enter for a new lap
#       ctrl+z for pause
#       ctrl+c to stop
```

Live interaction with a stopwatch is also possible by attaching your console to the stopwatch using the `sw attach` (or `sw-attach`) command. If you want to attach a console when you start a console, just run `sw start --attach`.