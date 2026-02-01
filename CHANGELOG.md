# Changelog

All notable changes to rtop will be documented in this file.

## [2.1.0] - 2026-02-01

### Added

#### Interactive Navigation
- **Mouse Support**: Scroll process list with mouse wheel, click to select processes
- **Multi-page Navigation**: F2-F5 keys to switch between Overview/Processes/Network/Storage pages
- **Process Scrolling**: Arrow keys (↑↓), PageUp/PageDown, Home/End for process navigation
- **Visual Indicators**: Selected process highlight and scroll position display

#### Process Management
- **Process Kill**: Press `k` to kill selected process with confirmation dialog
- **Regex Filtering**: Press `/` to enter filter mode with regex pattern matching
- **Filter Indicator**: Active filter displayed in footer bar
- **Smart Selection**: Mouse and keyboard selection with visual feedback

#### Hardware Monitoring
- **Battery Monitor**: Display battery percentage, charging status, and time remaining
- **Battery Icons**: Visual charging/discharging icons with color-coded percentage
- **Disk I/O Stats**: Monitor read/write bytes per second (Linux /proc/diskstats)
- **Per-interface Network**: Individual network interface statistics

#### User Interface
- **Page Indicators**: Current page displayed in header with F-key hints
- **Confirmation Dialogs**: Safe process kill with Y/N confirmation
- **Enhanced Footer**: Battery status, active filters, and more context
- **Scrollbar Indicators**: Visual feedback for list position

### Changed
- Upgraded crossterm to support mouse event handling (event-stream feature)
- Enhanced help overlay with new navigation shortcuts
- Improved process list rendering with selection highlighting
- Updated dependencies: regex 1.10, battery 0.7

### Technical
- Added `ViewPage` enum for page-based rendering
- Extended `App` struct with mouse, battery, and diskio monitors
- Refactored input handling for complex event processing
- Added centered popup helper for dialogs

## [2.0.0] - 2026-02-01

### Added

#### Configuration System
- TOML-based configuration file support (`~/.config/rtop/config.toml`)
- Per-module refresh rate configuration
- Customizable color themes
- Display options (show/hide panels)
- Configurable alert thresholds for CPU, memory, temperature, disk
- `--generate-config` command to create default configuration
- `show-config` subcommand to display current configuration

#### Command-Line Interface
- Full CLI argument parsing with clap
- `--help` flag for comprehensive help
- `--version` flag to display version
- `--minimal` mode for reduced system load
- `--no-color` mode for monochrome output
- `--interval` to override refresh rates
- `--export` to export metrics and exit
- `--format` to specify export format (json/csv)
- `--log` to enable logging to file
- `--log-interval` to control logging frequency
- `--duration` to run for a specific time period
- `-v, -vv, -vvv` for verbose logging levels

#### Interactive Features
- Help overlay system (press `h` or `F1`)
- Pause/Resume functionality (press `Space`)
- Interactive keyboard shortcuts display
- Process filtering (start typing to filter)

#### System Monitoring
- System information monitor (hostname, OS, kernel)
- Uptime display with human-readable formatting
- Load average (1m, 5m, 15m) in footer bar
- Total process count display
- Self-monitoring option

#### Data Export
- JSON export format with comprehensive metrics
- CSV export format for easy data analysis
- Export subcommand for scripted use
- Real-time metrics collection API
- Structured metrics with timestamps

#### UI Enhancements
- Footer/status bar with system information
- Pause indicator in status bar
- Help overlay with comprehensive shortcuts
- Enhanced visual feedback
- Better layout responsiveness

#### Developer Features
- Proper error handling with thiserror
- Structured logging with tracing
- Signal handling for graceful shutdown (SIGTERM, SIGINT)
- Modular architecture with clear separation
- Export API for programmatic access

### Changed
- Bumped version to 2.0.0
- Upgraded to latest dependencies
- Improved error messages and user feedback
- Enhanced code organization and modularity
- Better terminal size handling
- Optimized rendering performance

### Fixed
- Memory leaks in long-running sessions
- Terminal state restoration on panic
- Race conditions in update loops
- Sensor detection edge cases

### Performance
- Reduced binary size to ~949 KB
- Optimized hot paths with pre-allocation
- Minimal tokio features for smaller footprint
- Efficient system call batching
- Sub-1% CPU usage during normal operation

### Documentation
- Comprehensive README with all features
- Example configuration file
- Troubleshooting guide
- Development instructions
- API documentation

## [1.0.0] - 2025-12-01

### Initial Release

#### Core Features
- Real-time CPU monitoring with per-core graphs
- Memory and swap monitoring with gauges
- Temperature sensor support (auto-detection)
- Network transfer statistics
- Disk usage monitoring
- Process list with sorting (PID, CPU, Memory)
- Terminal UI with ratatui
- Color-coded visual feedback
- Historical graphs (60-second windows)
- Rounded borders and modern design

#### Keyboard Shortcuts
- `q`, `Esc`, `Ctrl+C` to quit
- `p` to sort by PID
- `c` to sort by CPU
- `m` to sort by Memory

#### Technical
- Rust-based implementation
- Cross-platform support (Linux, macOS, Windows)
- Optimized build profile
- Clean architecture
