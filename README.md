# Stopwatchd

Command-line stopwatch daemon and controller for Unix-like systems.

## Contents

 - [Building](#building)
 - [Usage](#usage)
 - [Configuring](#configuring)
 - [Cargo Features](#cargo-features)

## Building

See [BUILD.md](BUILD.md) for more information.

## Usage

### `swd`

Stopwatchd consists of a daemon called `swd` to manage active stopwatches. To start it, just run `swd` in a terminal.

As `swd` doesn't do its own forking, it relies on other programs like `systemd` to do it for them. For systems with `systemd`, the command to start `swd` is:

```bash
systemctl start stopwatchd.service
```

In the future, I might add a systemd `.socket` to allow you to start `swd` on demand. Support for other init systems are also waiting in the wings.

### `swctl`

`swctl` is the main command line utility to interact with `swd`. The usage guide will go through the basic functionality of `swctl` and its subcommands.

```bash
swctl start aaa
# id             name   state     total time     laps count   lap time     
# @fb767e46acbb   aaa    playing   00:00:00.000   1            00:00:00.000
```

`swctl start` creates a new stopwatch and prints out the details of the new stopwatch. In the above example, the user requested that the stopwatch be named `aaa`.

```bash
swctl start
# id             name   state     total time     laps count   lap time     
# @ae03ec92332a          playing   00:00:00.000   1            00:00:00.000
```
You could also choose to leave the name blank as shown. In fact, you can have multiple stopwatches with a blank name. However, this is fine since you can also use the id of a stopwatch to identify it.

```bash
swctl info aaa
# id             name   state     total time     laps count   lap time     
# @fb767e46acbb   aaa    playing   00:00:59.505   1            00:00:59.505
```

`swctl info` queries `swd` for information about a stopwatch/some stopwatches. For this example, `swctl info` is being used to obtain details about the stopwatch with the name `aaa`.

```bash
swctl info @ae03ec92332a
# id             name   state     total time     laps count   lap time     
# @ae03ec92332a          playing   00:01:05.638   1            00:01:05.638
```

Alternatively, you can search for a stopwatch using just its id.

```bash
swctl info
# id             name   state     total time     laps count   lap time     
# @b9a22a4c2397   sw-1   playing   00:00:06.569   1            00:00:06.569 
# @dc4141203311   sw-2   playing   00:00:05.613   1            00:00:05.613 
# @9c8e2973244c   sw-3   playing   00:00:04.787   1            00:00:04.787 
# @4c2367a8a514   sw-4   playing   00:00:03.956   1            00:00:03.956 
# @eed7d9f618c1   sw-5   playing   00:00:03.123   1            00:00:03.123
```

If you don't have a list of specific stopwatches to query, `swctl` will look for all stopwatches.

```bash
swctl pause aaa
# id             name   state    total time     laps count   lap time     
# @fb767e46acbb   aaa    paused   00:02:18.797   1            00:02:18.797
```

`swctl pause` temporarily stops the timer on the current lap.

```bash
swctl play aaa
# id             name   state     total time     laps count   lap time     
# @fb767e46acbb   aaa    playing   00:02:18.797   1            00:02:18.797
```

`swctl play` tells the lap to continue timing.

```bash
swctl lap aaa
# id             name   state     total time     laps count   lap time     
# @fb767e46acbb   aaa    playing   00:02:48.137   2            00:00:00.000
```

`swctl lap` adds a lap to the specified stopwatch*es*. 

```bash
swctl stop abcdef
# id             name   state   total time     laps count   lap time     
# @fb767e46acbb   aaa    ended   00:03:44.576   2            00:00:56.438
```

`swctl stop` stops the stopwatch. Unlike `swctl pause`, this permanently prevents the stopwatch from playing again and no new laps can be added.

```bash
swctl info aaa --verbose
#+++
# id           @fb767e46acbb 
# name         aaa          
# state        ended        
# total time   00:03:44.576 
# laps count   2            
# lap time     00:00:56.438 
#---
# id                                     stopwatch id   duration     
# 70536f24-832a-4e30-8a39-919718987dc0   @fb767e46acbb   00:02:48.137 
# 0ed58de2-98d5-406b-8163-08eb8dc6a1fd   @fb767e46acbb   00:00:56.438
#+++
```

`swctl info [name...] --verbose` prints out more verbose information for the specified stopwatches. This also works for `swctl info --verbose`. However, the output could get messy if there are many stopwatches with many laps.

```bash
swctl info aaa --dur-fmt '%M-%S'
# id             name   state   total time   laps count   lap time 
# @fb767e46acbb   aaa    ended   03-44        2            00-56
```

The `--dur-fmt` argument allows you to set a **strftime** format for any duration values outputted to stdout.

```bash
swctl delete aaa
# id             name   state   total time     laps count   lap time     
# @fb767e46acbb   aaa    ended   00:03:44.576   2            00:00:56.438
```

To delete a stopwatch from memory and disk, use `swctl delete`.

### `swtui`

From version 0.5.0 onwards, Stopwatchd has a TUI utility called `swtui` that allows you to view and control stopwatches graphically, built using [pancurses](https://github.com/ihalila/pancurses). By default, `swtui` is compiled and shipped along with the rest of Stopwatchd. If you don't want this, you can disable the `swtui` feature. As always, more information is available in the [build guide](/BUILD.md).

```bash
swtui
```

To use, it simply type `swtui` in your terminal as shown above.

```bash
swtui --focus my-stopwatch
```

`swtui` can be forced to focus on a stopwatch using the `--focus` flag, given its name or UUID. If the stopwatch does not exist, `swtui` will simply not focus on any stopwatch.

```bash
swtui --new another
```

If you need to start a stopwatch straight from the command line, there is always the `--new` flag that allows you to create a stopwatch given its name. `swtui` automatically focuses on the new stopwatch.

## Configuring

`swd` can be configured if the `swd-config` cargo feature is activated. If so, `swd` reads `/etc/stopwatchd.toml`. `swd` sessions spawned by a non-root user is configured using `$XDG_CONFIG_HOME/stopwatchd.toml` instead. A different config file can be specified using the `--config` flag.

```bash
swd --config ~/e.toml
```

The default configuration file is stored in `/usr/share/stopwatchd/swd.toml` which is not read by `swd`. I've included it so that it can act as a backup/example if you need a fresh copy.

## Cargo Features

Feature names in **bold** are compiled by default.

| Feature Name | Description |
| - | - |
| **swd-config** | `swd` can be configured with files. |
| **users** | One `swd` can run for each user on a system instead of only just one per system. |
