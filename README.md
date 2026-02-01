# rtop

System monitoring dashboard for terminal written in Rust.

## Features

- **Real-time CPU monitoring**: Track CPU usage for all cores with historical graphs
- **Memory & Swap monitoring**: Visual gauges and historical charts
- **Temperature monitoring**: View all system temperature sensors with individual tracking and historical graphs (when available)
- **Network statistics**: Real-time network transfer rates and totals
- **Disk usage**: Monitor disk space usage
- **Process management**: View and sort running processes by PID, CPU, or Memory

## Requirements

- Linux / macOS / Windows
- Rust 1.70 or higher (for building from source)

## Installation

### From Source

```bash
git clone <your-repo>
cd rtop
source "$HOME/.cargo/env"
cargo build --release
sudo cp target/release/rtop /usr/local/bin/
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

Start rtop with the `rtop` command:

```bash
rtop
```

### Keyboard Shortcuts

- `q` or `Esc` or `Ctrl+C`: Quit the application
- `p`: Sort processes by PID
- `c`: Sort processes by CPU usage
- `m`: Sort processes by Memory usage
UI Overview

rtop features a modern, color-coded interface with:
- **Dynamic colors**: Visual feedback based on system load (green/yellow/red)
- **Rounded borders**: Clean, polished appearance
- **Real-time graphs**: Historical data visualization with 60-second windows
- **Temperature display**: Automatic detection of all thermal sensors with individual graphs and current values
- **Responsive layout**: Adapts based on available sensors and terminal size

## Architecture

The project is organized into several modules:

- **monitor/**: System information collection modules
  - `cpu.rs`: CPU usage monitoring
  - `memory.rs`: Memory and swap monitoring
  - `network.rs`: Network statistics
  - `disk.rs`: Disk usage monitoring
  - `process.rs`: Process information and sorting
  - `temp.rs`: Temperature sensor monitoring with multi-sensor track
  - `process.rs`: Process information and sorting
- **ui.rs**: Terminal user interface using ratatui
- **utils.rs**: Utility functions (byte formatting, colors)
- **main.rs**: Application entry point and main loop

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui): Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm): Cross-platform terminal manipulation
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo): System information library
- [tokio](https://tokio.rs/): Async runtime
- [anyhow](https://github.com/dtolnay/anyhow): Error handling

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

MIT License - see LICENSE file for details

## Original Project

This is a Rust reimplementation of [gtop](https://github.com/aksakalli/gtop) originally written in Node.js.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
