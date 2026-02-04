# rtop

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://github.com/npequeux/rtop/workflows/Rust/badge.svg)](https://github.com/npequeux/rtop/actions)
[![Docker](https://github.com/npequeux/rtop/workflows/Docker/badge.svg)](https://github.com/npequeux/rtop/actions)
[![codecov](https://codecov.io/gh/npequeux/rtop/branch/master/graph/badge.svg)](https://codecov.io/gh/npequeux/rtop)
[![GitHub release](https://img.shields.io/github/v/release/npequeux/rtop.svg)](https://github.com/npequeux/rtop/releases)
[![Docker Image](https://ghcr-badge.egpl.dev/npequeux/rtop/latest_tag?trim=major&label=docker)](https://github.com/npequeux/rtop/pkgs/container/rtop)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://npequeux.github.io/rtop/rtop/)
[![GitHub stars](https://img.shields.io/github/stars/npequeux/rtop.svg?style=social)](https://github.com/npequeux/rtop)

Advanced system monitoring dashboard for terminal written in Rust - **Now with btop++-inspired features!**

## 🎉 NEW in v3.0

rtop now includes advanced features inspired by btop++:

- 🎨 **Advanced Graphics**: Braille/Unicode graphs with 2x data density
- 🌈 **Theme System**: Color gradients and customizable TOML themes
- 🎮 **GPU Monitoring**: NVIDIA/AMD/Intel GPU support with real-time stats
- 🌲 **Process Tree View**: Hierarchical process display
- 📶 **Enhanced Signals**: 9 different process signals (TERM, KILL, STOP, CONT, etc.)
- 🎯 **Sophisticated UI**: Rounded corners, gradient meters, multiple graph styles
- ⚙️ **Extended Config**: Per-component customization and theming

See [ENHANCED_FEATURES.md](ENHANCED_FEATURES.md) for complete documentation.

## Features

### Core Monitoring

- **Real-time CPU monitoring**: Track CPU usage for all cores with Braille/Unicode historical graphs
- **Memory & Swap monitoring**: Visual gauges and historical charts with color gradients
- **GPU monitoring**: NVIDIA/AMD GPU utilization, memory, temperature, and power (NEW!)
- **Temperature monitoring**: Auto-detect and display all system temperature sensors
- **Network statistics**: Real-time transfer rates with per-interface monitoring
- **Disk usage**: Visual progress bars with usage alerts
- **Battery monitoring**: Display battery percentage, charging status, and time remaining
- **Disk I/O**: Monitor read/write bytes per second for all disks
- **Process management**: View, sort, filter, kill processes with tree view support
- **System information**: Hostname, OS, kernel version, uptime, and load averages

### Interactive Features (v3.0)

- **Mouse Support**: Scroll process list with mouse wheel, click to select processes
- **Multi-page Navigation**: Switch between Overview, Processes, Network, and Storage pages with F2-F5
- **Process Scrolling**: Navigate with ↑↓ arrows, PageUp/PageDown, Home/End keys
- **Process Tree View**: Hierarchical display of parent-child process relationships (NEW!)
- **Enhanced Signals**: Send TERM, KILL, INT, HUP, QUIT, STOP, CONT, USR1, USR2 to processes (NEW!)
- **Regex Filtering**: Press `/` to filter processes by regex pattern
- **Visual Feedback**: Selected process highlighting, scroll indicators, page navigation hints
- **GPU Display**: Real-time GPU utilization, memory, temperature with graphs (NEW!)

### Advanced Graphics & Theming (NEW!)

- **Braille Graphs**: High-resolution unicode graphs (⣿⣾⣶⣦) with 2x data density
- **Multiple Graph Styles**: Braille, Block, TTY-compatible rendering
- **Color Gradients**: 101-step smooth color transitions for all metrics
- **Theme System**: TOML-based themes with per-component color customization
- **Rounded Corners**: Beautiful UI with ╭╮╰╯ box drawing characters
- **Gradient Meters**: Color-coded meters showing warning/critical zones

### Advanced Features

- **Help overlay**: Press `h` or `F1` for interactive help screen with all shortcuts
- **Pause/Resume**: Space bar to freeze display updates
- **Configurable thresholds**: Customize warning and critical levels
- **Data export**: Export metrics to JSON or CSV formats
- **Logging support**: Continuous monitoring with interval-based logging
- **CLI options**: Run with specific duration, minimal mode, no-color mode
- **Configuration file**: Customize refresh rates, colors, and display options
- **Signal handling**: Graceful shutdown on Ctrl+C/SIGTERM

## Requirements

- Linux / macOS / Windows
- Rust 1.88 or higher (for building from source)

## Installation

### Docker (Recommended)

```bash
# Pull the latest image
docker pull ghcr.io/npequeux/rtop:latest

# Run with host access
docker run -it --rm --pid=host --privileged ghcr.io/npequeux/rtop:latest

# Or use docker compose (see docker-compose.yml)
docker compose up
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/npequeux/rtop/releases):

- Linux x86_64: `rtop-linux-x86_64.tar.gz`
- Linux ARM64: `rtop-linux-aarch64.tar.gz`
- macOS Intel: `rtop-macos-x86_64.tar.gz`
- macOS Apple Silicon: `rtop-macos-aarch64.tar.gz`
- Windows: `rtop-windows-x86_64.zip`

### From Source

```bash
git clone https://github.com/npequeux/rtop.git
cd rtop
cargo build --release
sudo cp target/release/rtop /usr/local/bin/
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

Start rtop with the `rtop` command:

```bash
rtop
```

### Command Line Options

```bash
rtop --help                          # Show all available options
rtop --minimal                       # Run in minimal mode (slower updates)
rtop --no-color                      # Disable colors (monochrome mode)
rtop --interval 2000                 # Set custom update interval (ms)
rtop --export metrics.json           # Export current metrics and exit
rtop --export out.csv -f csv         # Export as CSV format
rtop --duration 1h                   # Run for 1 hour then exit
rtop --log /var/log/rtop.log         # Enable logging to file
rtop --generate-config               # Generate default config file
rtop -vvv                            # Enable verbose logging (debug mode)
```

### Subcommands

```bash
rtop show-config                     # Display current configuration
rtop init-config                     # Create default config file
rtop export -o data.json -f json     # Export metrics to file
```

### Keyboard Shortcuts

#### Navigation & Control

- `q`, `Esc`, or `Ctrl+C`: Quit the application
- `h` or `F1`: Toggle help screen
- `F2` - `F5`: Switch pages (Overview / Processes / Network / Storage)
- `Space`: Pause/Resume updates
- `r`: Force refresh all monitors

#### Process Management

- `↑` `↓`: Navigate up/down in process list
- `PgUp` / `PgDn`: Scroll page up/down (10 processes)
- `Home` / `End`: Jump to first/last process
- `k`: Kill selected process (with confirmation)
- `/`: Enter filter mode (type regex pattern)
- **Mouse**: Scroll with wheel, click to select

#### Process Sorting

- `p`: Sort processes by PID
- `c`: Sort processes by CPU usage
- `m`: Sort processes by Memory usage

#### Data Export

- `e`: Export current metrics to configured file

## Configuration

rtop supports configuration via `~/.config/rtop/config.toml`

Generate a default configuration file:

```bash
rtop --generate-config
```

### Configuration Options

```toml
[refresh_rates]
cpu = 1000          # CPU update interval in milliseconds
memory = 1000       # Memory update interval
network = 1000      # Network update interval
disk = 2000         # Disk update interval (less frequent)
process = 2000      # Process list update interval
temp = 1000         # Temperature sensor interval

[colors]
theme = "cyan"      # Color theme: "cyan", "green", "blue"
enable_colors = true

[display]
show_temperature = true      # Show temperature panel
show_network = true          # Show network panel
show_disk = true             # Show disk panel
max_processes = 20           # Maximum processes to display
show_kernel_processes = false
show_self = true             # Show rtop in process list

[thresholds]
cpu_warning = 60.0    # CPU warning threshold (%)
cpu_critical = 80.0   # CPU critical threshold (%)
memory_warning = 70.0
memory_critical = 90.0
temp_warning = 65.0   # Temperature warning (°C)
temp_critical = 80.0  # Temperature critical (°C)
disk_warning = 80.0

[export]
enable_logging = false
log_path = "/var/log/rtop/metrics.log"
log_interval = 5000   # Log interval in milliseconds
```

## UI Overview

rtop features a modern, color-coded interface with:

- **Dynamic colors**: Visual feedback based on system load (green/yellow/red)
- **Rounded borders**: Clean, polished appearance
- **Real-time graphs**: Historical data visualization with 60-second windows
- **Temperature display**: Automatic detection with individual sensor tracking
- **Responsive layout**: Adapts based on available sensors and terminal size
- **Status bar**: Shows uptime, load average, total processes
- **Help overlay**: Interactive help accessible with `h` key
- **Pause indicator**: Visual feedback when updates are paused

## Architecture

The project is organized into several modules:

### Core Modules

- **monitor/**: System information collection
  - `cpu.rs`: CPU usage monitoring with per-core tracking
  - `memory.rs`: Memory and swap usage monitoring
  - `network.rs`: Network transfer statistics
  - `disk.rs`: Disk usage and availability
  - `process.rs`: Process information with sorting capabilities
  - `temp.rs`: Temperature sensor monitoring (multi-sensor support)
  - `system.rs`: System information (uptime, load, hostname)

### Application Modules

- **ui.rs**: Terminal user interface with ratatui
  - Help overlay system
  - Pause/resume functionality
  - Dynamic layout based on available sensors
  - Color-coded visual feedback
  
- **config.rs**: Configuration management
  - TOML-based configuration
  - Default values with customization
  - Per-module refresh rate control
  
- **cli.rs**: Command-line interface with clap
  - Argument parsing
  - Subcommand handling
  - Duration parsing utilities
  
- **export.rs**: Data export functionality
  - JSON format support
  - CSV format support
  - Metrics collection and serialization
  
- **error.rs**: Error handling with thiserror
  - Custom error types
  - Error context and propagation
  
- **utils.rs**: Utility functions
  - Byte formatting
  - Color definitions
  
- **main.rs**: Application entry point
  - Signal handling
  - Configuration loading
  - Terminal setup and cleanup

## Export Formats

### JSON Export

```bash
rtop --export metrics.json
```

Example output:

```json
{
  "timestamp": "2026-02-01T10:30:45+00:00",
  "cpu": {
    "cores": [
      {"id": 0, "usage": 45.2},
      {"id": 1, "usage": 32.1}
    ],
    "average": 38.65
  },
  "memory": {
    "total": 16777216000,
    "used": 8388608000,
    "percent": 50.0
  },
  "system": {
    "hostname": "mycomputer",
    "uptime": 86400,
    "load_average": [1.5, 1.3, 1.2]
  }
}
```

### CSV Export

```bash
rtop --export metrics.csv -f csv
```

Example output:

```csv
timestamp,cpu_avg,memory_percent,swap_percent,network_rx_rate,network_tx_rate,uptime,load_1m,load_5m,load_15m
2026-02-01T10:30:45+00:00,38.65,50.00,5.20,1048576.00,524288.00,86400,1.50,1.30,1.20
```

## Performance

- **Binary size**: ~949 KB (stripped, optimized)
- **Memory usage**: ~5-10 MB RSS
- **CPU usage**: <1% on modern systems
- **Update latency**: <50ms for UI responsiveness

Optimizations:

- Fat LTO compilation
- Single codegen unit
- Minimal tokio features
- Pre-allocated vectors in hot paths
- Efficient system call batching

## Development

### Building from Source

```bash
git clone <your-repo>
cd rtop
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Development Mode

```bash
cargo run
```

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui): Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm): Cross-platform terminal manipulation
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo): System information library
- [tokio](https://tokio.rs/): Async runtime (minimal features)
- [clap](https://github.com/clap-rs/clap): Command-line argument parsing
- [serde](https://serde.rs/): Serialization/deserialization
- [toml](https://github.com/toml-rs/toml): TOML parser
- [thiserror](https://github.com/dtolnay/thiserror): Error handling
- [tracing](https://github.com/tokio-rs/tracing): Logging framework
- [chrono](https://github.com/chronotope/chrono): Date/time handling
- [dirs](https://github.com/soc/dirs-rs): System directories

## Troubleshooting

### Temperature Sensors Not Showing

Temperature sensors may not be available on all systems. On Linux, you may need to:

```bash
sudo modprobe coretemp  # For Intel CPUs
sudo modprobe k10temp   # For AMD CPUs
```

### Permission Denied Errors

Some features may require elevated permissions:

```bash
sudo rtop  # Run with sudo if needed
```

### High CPU Usage

If rtop is using too much CPU, try:

```bash
rtop --minimal           # Slower update intervals
rtop --interval 2000     # Custom interval (2 seconds)
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Version History

- **v2.0.0** (2026-02-01)
  - Added configuration file support
  - CLI argument parsing with clap
  - Data export (JSON/CSV)
  - Help overlay system
  - Pause/resume functionality
  - System information monitoring
  - Signal handling
  - Logging support
  - Enhanced error handling

- **v1.0.0** (Initial release)
  - Basic monitoring features
  - CPU, memory, network, disk, processes
  - Temperature sensor support
  - TUI with ratatui

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

## Original Project

This is a Rust reimplementation of [gtop](https://github.com/aksakalli/gtop) originally written in Node.js.
