# lv-rs - Log Viewer in Rust

A terminal-based log viewer written in Rust. This is a Rust port of the original JavaScript log viewer.

## Features

- Browse and select log files from a directory
- View log entries with timestamps, levels, and messages
- Color-coded log levels (DEBUG, INFO, WARN, ERROR)
- Expand/collapse multi-line log entries
- Real-time file watching (tail mode)
- Terminal-based user interface

## Installation

```bash
cargo install --path .
```

## Usage

```bash
lv-rs [DIRECTORY]
```

If no directory is specified, the current directory will be used.

## Controls

- `↑/↓`: Navigate up/down in the current list
- `←/→`: Switch between file list and log view
- `Space`: Expand/collapse multi-line log entries
- `q` or `Esc`: Quit the application

## Log Format

The log viewer expects log entries in the following format:

```
[TIMESTAMP] [LEVEL] [MESSAGE]
```

Where:
- `TIMESTAMP` is an ISO formatted timestamp
- `LEVEL` can be `DEBUG`, `INFO`, `WARN`, or `ERROR`
- `MESSAGE` is the log message text

## Building

```bash
cargo build --release
```

## License

MIT 