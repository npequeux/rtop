# rtop Quick Start Guide

## Installation

```bash
# Clone and build
git clone <your-repo>
cd rtop
cargo build --release
sudo cp target/release/rtop /usr/local/bin/
```

## First Run

```bash
rtop
```

Press `h` for help, `Space` to pause, `q` to quit.

## Common Use Cases

### 1. Basic Monitoring

```bash
rtop                    # Start with default settings
```

### 2. Custom Configuration

```bash
# Generate config file
rtop --generate-config

# Edit the config
nano ~/.config/rtop/config.toml

# Run with custom config
rtop
```

### 3. Export System Metrics

```bash
# Export to JSON
rtop --export metrics.json

# Export to CSV
rtop --export metrics.csv -f csv

# Export using subcommand
rtop export -o data.json -f json
```

### 4. Monitoring Session

```bash
# Run for 1 hour
rtop --duration 1h

# Run for 30 minutes with logging
rtop --duration 30m --log /tmp/rtop.log

# Minimal mode (lower resource usage)
rtop --minimal
```

### 5. Scripting and Automation

```bash
# Export metrics every 5 minutes (cron job)
*/5 * * * * /usr/local/bin/rtop --export /var/log/rtop/metrics-$(date +\%Y\%m\%d-\%H\%M).json

# Run for a specific duration and exit
rtop --duration 60s --export final-metrics.json
```

### 6. Performance Tuning

```bash
# Slower updates for lower CPU usage
rtop --interval 5000

# No colors for compatibility
rtop --no-color

# View current configuration
rtop show-config
```

## Keyboard Shortcuts

### Essential

- `q` or `Esc` - Quit
- `h` or `F1` - Help screen
- `Space` - Pause/Resume

### Process Sorting

- `p` - Sort by PID
- `c` - Sort by CPU
- `m` - Sort by Memory

## Configuration Tips

### Adjust Refresh Rates

Edit `~/.config/rtop/config.toml`:

```toml
[refresh_rates]
cpu = 500      # Update CPU every 500ms (faster)
disk = 5000    # Update disk every 5s (slower)
```

### Set Alert Thresholds

```toml
[thresholds]
cpu_critical = 90.0     # Alert when CPU > 90%
memory_warning = 75.0   # Warn when memory > 75%
temp_critical = 85.0    # Critical temp at 85°C
```

### Customize Display

```toml
[display]
show_temperature = true     # Show temp panel
max_processes = 30          # Show 30 processes
show_kernel_processes = false  # Hide kernel processes
```

## Troubleshooting

### No Temperature Sensors

```bash
# Load kernel modules (Linux)
sudo modprobe coretemp    # Intel
sudo modprobe k10temp     # AMD
```

### Permission Issues

```bash
# Run with sudo if needed
sudo rtop
```

### High CPU Usage

```bash
# Use minimal mode
rtop --minimal

# Custom slow interval
rtop --interval 3000
```

## Advanced Usage

### Continuous Monitoring and Logging

```bash
# Create log directory
mkdir -p /var/log/rtop

# Run with logging enabled
rtop --log /var/log/rtop/metrics.log --log-interval 10

# Or configure in config.toml:
[export]
enable_logging = true
log_path = "/var/log/rtop/metrics.log"
log_interval = 10000
```

### Data Analysis

```bash
# Export CSV for analysis
rtop --export data.csv -f csv

# Import into spreadsheet or use with tools like:
# - gnuplot
# - pandas (Python)
# - R
# - Excel
```

### Integration with Monitoring Systems

```bash
# Export JSON for Prometheus/Grafana
rtop --export /var/lib/node_exporter/rtop.json

# Periodic export with systemd timer
# Create /etc/systemd/system/rtop-export.service
# and /etc/systemd/system/rtop-export.timer
```

## Performance Benchmarks

- **Binary Size**: ~949 KB
- **Memory Usage**: ~5-10 MB
- **CPU Usage**: <1%
- **Update Latency**: <50ms

## Getting Help

```bash
rtop --help              # Command-line help
rtop show-config         # View configuration
```

Press `h` in the application for interactive help.

## What's New in v2.0

- ✨ Configuration file support
- 🎨 Help overlay system
- ⏸️ Pause/Resume functionality
- 📊 Data export (JSON/CSV)
- 🔧 CLI argument parsing
- 📈 System information (uptime, load)
- 🛡️ Signal handling
- 📝 Logging support
- 🎯 Custom thresholds
- ⚡ Performance improvements

Enjoy monitoring with rtop! 🚀
