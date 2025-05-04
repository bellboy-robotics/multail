# multail - Modern Terminal Log Viewer

A powerful, feature-rich terminal-based log viewer written in Rust. `multail` combines the best features of traditional log viewers with modern terminal UI capabilities, making it an essential tool for developers and system administrators.

![multail screenshot](screenshot.png)

## Features

### Core Features
- 🚀 **Blazing Fast**: Built in Rust for maximum performance
- 📂 **Directory Navigation**: Browse and select log files from any directory
- 👀 **Real-time Monitoring**: Watch multiple log files simultaneously
- 🎨 **Color-coded Logs**: Automatic color coding for different log levels
- 📝 **Multi-line Support**: Expand/collapse complex log entries
- 🔍 **Pattern Matching**: Highlight and filter logs using regex patterns
- ⏱️ **Timestamp Parsing**: Automatic detection and formatting of timestamps

### Advanced Features
- 📊 **Log Statistics**: View log level distribution and patterns
- 🔄 **Auto-refresh**: Configurable refresh rates for real-time monitoring
- 🎯 **Smart Filtering**: Filter logs by level, pattern, or time range
- 📱 **Responsive UI**: Adapts to terminal size and supports mouse interaction
- 💾 **Log Export**: Save filtered logs to file
- 🎭 **Custom Themes**: Configure colors and display preferences

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/multail.git
cd multail

# Build and install
cargo install --path .
```

### From GitHub Releases
Download the appropriate binary for your platform from the [releases page](https://github.com/yourusername/multail/releases).

### From Cargo
```bash
cargo install multail
```

## Usage

### Basic Usage
```bash
# View logs in the current directory
multail

# View logs in a specific directory
multail /path/to/logs

# Watch specific log files
multail /var/log/syslog /var/log/auth.log
```

### Command Line Options
```bash
multail [OPTIONS] [FILES...]

Options:
    -f, --follow           Follow (tail) the log files
    -n, --lines NUM        Number of lines to show initially
    -p, --pattern PATTERN  Filter logs matching the pattern
    -l, --level LEVEL      Filter by log level (debug, info, warn, error)
    -t, --theme THEME      Use a specific theme (dark, light, custom)
    -h, --help             Show help message
    -V, --version          Show version information
```

### Interactive Controls

#### Navigation
- `↑/↓`: Navigate up/down in lists
- `←/→`: Switch between panels
- `Tab`: Cycle through available panels
- `Enter`: Select/expand current item
- `Space`: Expand/collapse multi-line entries

#### Actions
- `f`: Toggle follow mode
- `r`: Refresh logs
- `s`: Save current view to file
- `q` or `Esc`: Quit
- `/`: Search logs
- `?`: Show help

## Configuration

Create a `~/.config/multail/config.toml` file to customize multail:

```toml
[theme]
background = "black"
foreground = "white"
error = "red"
warn = "yellow"
info = "green"
debug = "blue"

[display]
timestamp_format = "%Y-%m-%d %H:%M:%S"
refresh_rate = 1.0
show_line_numbers = true
wrap_lines = false

[patterns]
error = "ERROR|error|exception"
warn = "WARN|warning"
info = "INFO|info"
debug = "DEBUG|debug"
```

## Log Format Support

multail automatically detects and parses common log formats:

- Standard syslog format
- JSON logs
- Common web server logs (Nginx, Apache)
- Custom formats (configurable via regex)

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by traditional log viewers like `multitail` and `lnav`
- Built with [tui-rs](https://github.com/fdehau/tui-rs) for the terminal interface
- Uses [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling

## Support

- 📚 [Documentation](https://github.com/yourusername/multail/wiki)
- 💬 [Discussions](https://github.com/yourusername/multail/discussions)
- 🐛 [Issue Tracker](https://github.com/yourusername/multail/issues) 