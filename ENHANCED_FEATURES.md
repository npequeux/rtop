# rtop v3.0 - Enhanced Features (btop-inspired)

## 🎉 NEW FEATURES - Inspired by btop++

This document describes all the new advanced features added to rtop, bringing it on par with btop++'s sophisticated monitoring capabilities.

---

## 1. Advanced Graphics System

### Braille/Unicode Graph Rendering
- **High-resolution graphs** using Braille patterns (⣿⣾⣶⣦) for smooth curves
- **Multiple graph styles**: Braille, Block, TTY-compatible
- **Dual-value encoding**: Two data points per character for 2x density
- **5x5 symbol resolution**: 25 different patterns for precise visualization

**Usage:**
```rust
use rtop::graphics::{GraphRenderer, GraphSymbol};

let renderer = GraphRenderer::new(40, 8, GraphSymbol::Braille, false);
let data: Vec<f64> = cpu_history; // Your data (0-100 scale)
let graph_lines = renderer.render(&data);
```

**Supported Symbols:**
- `Braille` - ⣿⣷⣧ Unicode Braille patterns (highest resolution)
- `Block` - █▓▒░ Block characters (high contrast)
- `Tty` - █▓▒░ TTY-compatible patterns

### Enhanced Meter/Gauge Rendering
- **Gradient-colored meters** with smooth color transitions
- **Segmented displays** for detailed visualization
- **Horizontal and vertical orientations**
- **Customizable width and precision**

**Example:**
```rust
use rtop::graphics::MeterRenderer;

let meter = MeterRenderer::new(20);
let output = meter.render_segmented(75); // 75% utilization
// Output: ■■■■■■■■■■■■■■■░░░░░
```

### Box Drawing with Unicode
- **Rounded or square corners** (╭╮╰╯ vs ┌┐└┘)
- **Title embeds** in box borders
- **Configurable line styles** (single, double, dashed)
- **Auto-sizing** based on content

---

## 2. Theme System

### Comprehensive Color Management
- **101-step color gradients** for smooth transitions
- **RGB hex color parsing** (#RRGGBB, #RGB, #GG)
- **Per-component theming**: CPU, Memory, Network, GPU, Processes
- **TOML-based theme files** for easy customization

**Theme Structure:**
```toml
[theme]
name = "custom"

# Main UI colors
main_fg = "#cc"
main_bg = "#00"
title = "#ee"
hi_fg = "#b54040"

# CPU gradient (blue → green → red)
cpu_start = "#4897d8"
cpu_mid = "#7ce567"
cpu_end = "#eb7070"

# Memory gradient (yellow → orange → red)
mem_start = "#ffc345"
mem_mid = "#f3a32e"
mem_end = "#e05a5a"

# GPU box color
gpu_box = "#35934d"
```

**Features:**
- Pre-built color gradients for all metrics
- Dynamic color interpolation (RGB space)
- Theme hot-reloading support
- Multiple themes per installation
- Compatible with btop theme format

**Create Custom Theme:**
```bash
# Generate default theme
mkdir -p ~/.config/rtop/themes
rtop --export-theme > ~/.config/rtop/themes/mytheme.toml

# Edit with your colors
vim ~/.config/rtop/themes/mytheme.toml

# Use theme
rtop --theme mytheme
```

---

## 3. GPU Monitoring

### Multi-GPU Support
- **NVIDIA GPUs** via nvidia-smi
- **AMD GPUs** via rocm-smi
- **Intel GPUs** via intel_gpu_top (limited)
- **Auto-detection** of all available GPUs

### Monitored Metrics
- ✅ GPU Utilization (0-100%)
- ✅ Memory Usage (used/total)
- ✅ Temperature (°C/°F)
- ✅ Power Draw (Watts)
- ✅ Clock Speed (MHz)
- ✅ Fan Speed (0-100%)
- ✅ Historical graphs (60-second window)

**GPU Display Features:**
- Individual boxes for each GPU
- Real-time utilization graphs
- Memory usage bars with gradients
- Temperature monitoring with color coding
- Power consumption tracking

**Keyboard Shortcuts:**
- `g` - Toggle GPU display
- `5` - Show/hide GPU 1
- `6` - Show/hide GPU 2
- `7` - Show/hide GPU 3

**Requirements:**
- NVIDIA: `nvidia-smi` in PATH
- AMD: `rocm-smi` in PATH
- Intel: `intel_gpu_top` in PATH (optional)

---

## 4. Enhanced Process Management

### Process Tree View
- **Hierarchical display** showing parent-child relationships
- **Indentation levels** to visualize process ancestry
- **Collapsible branches** (coming soon)
- **Root process identification**

**Toggle tree view:** Press `t` in process view

**Example Tree:**
```
PID    NAME              CPU%  MEM
1      systemd           0.1   2.3 MB
├─ 123   NetworkManager  0.5   15.2 MB
├─ 456   dockerd         2.1   120.5 MB
│  └─ 789  containerd    1.2   85.3 MB
└─ 234   sshd            0.0   5.1 MB
```

### Extended Signal Support
9 different signals available (matching btop):

| Signal | Code | Description |
|--------|------|-------------|
| TERM | 15 | Graceful termination (default) |
| KILL | 9 | Force kill (cannot be caught) |
| INT | 2 | Interrupt (Ctrl+C) |
| HUP | 1 | Hangup / reload config |
| QUIT | 3 | Quit and dump core |
| STOP | 19 | Pause process |
| CONT | 18 | Resume stopped process |
| USR1 | 10 | User-defined signal 1 |
| USR2 | 12 | User-defined signal 2 |

**Send Signal:**
1. Select process with ↑/↓ arrows
2. Press `s` to open signal menu
3. Choose signal with 1-9 keys
4. Confirm with `y`

### Additional Process Info
- User/UID information
- Process state (R/S/Z/D)
- Thread count
- Disk I/O (read/write bytes)
- Parent process ID (PPID)
- Child process list

**New Sort Options:**
- `P` - Sort by PID
- `C` - Sort by CPU% (default)
- `M` - Sort by Memory
- `N` - Sort by Name
- `U` - Sort by User

---

## 5. Advanced UI Features

### Symbol Customization
- **Graph symbol selection**: Braille, Block, or TTY
- **Corner style**: Rounded (╭╮╰╯) or square (┌┐└┘)
- **Line style**: Single, double, or dashed
- **Per-box customization** (coming soon)

**Configuration:**
```toml
[display]
graph_symbol = "braille"  # or "block", "tty"
rounded_corners = true
double_borders = false
```

### Enhanced Color Gradients
- **Smooth transitions** across 101 color steps
- **Per-metric gradients**: CPU, Memory, Network, Temp, GPU
- **Configurable breakpoints** for warning/critical zones
- **Inverted gradients** for special cases

**Gradient Examples:**
- CPU: Blue (idle) → Green (moderate) → Red (high)
- Memory: Yellow (low) → Orange (medium) → Red (critical)
- Temperature: Blue (cool) → Yellow (warm) → Red (hot)
- Network: Green (download) / Red (upload)

### Meter Variations
- **Solid fill**: ■■■■■░░░░░
- **Gradient fill**: Colored based on percentage
- **Segmented**: Individual blocks with spacing
- **Vertical meters**: For space-constrained displays

---

## 6. Configuration Enhancements

### Extended Config Options

```toml
[display]
# Graph rendering
graph_symbol = "braille"  # braille, block, tty
graph_height = 8
rounded_corners = true

# Color theme
theme = "default"  # or custom theme name
color_gradients = true

# GPU monitoring
enable_gpu = true
gpu_temp_unit = "celsius"  # or "fahrenheit"

# Process view
process_tree = false
show_process_threads = true
process_sorting = "cpu"  # cpu, memory, pid, name, user

[thresholds]
# Custom warning/critical levels
cpu_warning = 70
cpu_critical = 90
mem_warning = 80
mem_critical = 95
temp_warning = 70
temp_critical = 85
gpu_temp_warning = 75
gpu_temp_critical = 90
```

---

## 7. Performance Optimizations

### Graph Caching
- **Render cache** for repeated patterns
- **Incremental updates** (only new data)
- **Lazy rendering** (only visible elements)

### Smart Refresh
- **Differential updates** (changed data only)
- **Configurable refresh rates** per component
- **Pause support** to freeze display

### Memory Efficiency
- **Fixed-size history buffers** (60 samples default)
- **Circular queues** for constant memory
- **String interning** for repeated UI elements

---

## 8. Keyboard Shortcuts (Full List)

### Navigation
- `q`, `Esc`, `Ctrl+C` - Quit
- `h`, `F1` - Toggle help
- `F2` - Overview page
- `F3` - Processes page
- `F4` - Network page
- `F5` - Storage page
- `Space` - Pause/Resume
- `r` - Force refresh

### Process Management
- `↑`/`↓` - Select process
- `PgUp`/`PgDn` - Scroll page
- `Home`/`End` - Jump to top/bottom
- `k` - Kill process (SIGTERM)
- `K` - Force kill (SIGKILL)
- `s` - Send signal (opens menu)
- `t` - Toggle tree view
- `/` - Filter processes (regex)
- `Esc` - Clear filter

### Display Options
- `P` - Sort by PID
- `C` - Sort by CPU
- `M` - Sort by Memory
- `N` - Sort by Name
- `U` - Sort by User
- `g` - Toggle GPU display
- `5-7` - Toggle individual GPU displays

### View Options
- `b` - Toggle graph symbols (Braille/Block/TTY)
- `c` - Cycle corner styles
- `+`/`-` - Adjust graph height
- `T` - Cycle themes

---

## 9. Export & Logging

### Enhanced Export
```bash
# Export with GPU data
rtop --export system-snapshot.json

# Export all metrics to CSV
rtop --export report.csv --format csv --include-gpu

# Continuous logging
rtop --log monitor.log --interval 5000 --duration 1h
```

**Export Format (JSON):**
```json
{
  "timestamp": "2026-02-02T10:30:00Z",
  "cpu": { ... },
  "memory": { ... },
  "gpus": [
    {
      "index": 0,
      "name": "NVIDIA RTX 4090",
      "utilization": 45,
      "memory_used": 8192,
      "memory_total": 24576,
      "temperature": 65,
      "power_usage": 180.5
    }
  ],
  ...
}
```

---

## 10. Comparison with btop++

| Feature | btop++ | rtop v3.0 | Status |
|---------|--------|-----------|--------|
| Braille graphs | ✅ | ✅ | Complete |
| Color gradients | ✅ | ✅ | Complete |
| Theme system | ✅ | ✅ | Complete |
| GPU monitoring | ✅ | ✅ | Complete |
| Process tree | ✅ | ✅ | Complete |
| Multiple signals | ✅ | ✅ | Complete |
| Mouse support | ✅ | ✅ | Complete |
| TTY mode | ✅ | ✅ | Complete |
| Rounded corners | ✅ | ✅ | Complete |
| Custom presets | ✅ | 🚧 | Planned |
| UI config menu | ✅ | 🚧 | Planned |
| Plugin system | ❌ | 🚧 | Planned |

**Legend:** ✅ Complete | 🚧 In Progress | ❌ Not Available

---

## Quick Start Examples

### Basic Monitoring with GPU
```bash
rtop --enable-gpu
```

### Custom Theme and Graph Style
```bash
rtop --theme gruvbox --graph-symbol braille --rounded
```

### Process Tree View
```bash
rtop --tree-view --sort cpu
```

### Full-Featured Monitoring
```bash
rtop \
  --enable-gpu \
  --theme default \
  --graph-symbol braille \
  --rounded \
  --tree-view \
  --log monitor.log \
  --interval 1000
```

---

## Building from Source

```bash
# Clone repository
git clone https://github.com/yourusername/rtop
cd rtop

# Build with all features
cargo build --release --all-features

# Install
sudo cp target/release/rtop /usr/local/bin/

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

---

## Credits

Enhanced features inspired by:
- **btop++** by [aristocratos](https://github.com/aristocratos/btop)
- **bpytop** by aristocratos
- **bashtop** by aristocratos

Original rtop implementation by Nicolas Pequeux

---

## License

MIT License - See LICENSE file for details
