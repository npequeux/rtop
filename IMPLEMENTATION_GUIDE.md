# Implementation Guide - rtop v3.0 Enhanced Features

This guide explains how to use and integrate the new btop++-inspired features in rtop.

---

## Table of Contents

1. [Using Advanced Graphics](#1-using-advanced-graphics)
2. [Creating Custom Themes](#2-creating-custom-themes)
3. [GPU Monitoring Setup](#3-gpu-monitoring-setup)
4. [Process Tree View](#4-process-tree-view)
5. [Sending Process Signals](#5-sending-process-signals)
6. [Configuration Examples](#6-configuration-examples)
7. [Development Guide](#7-development-guide)

---

## 1. Using Advanced Graphics

### Graph Rendering

The new graphics module provides high-resolution graphs using Braille patterns:

```rust
use rtop::graphics::{GraphRenderer, GraphSymbol};

// Create a Braille graph renderer
let renderer = GraphRenderer::new(
    40,                      // width in characters
    8,                       // height in lines
    GraphSymbol::Braille,    // symbol type
    false                    // inverted (false = upward)
);

// Prepare your data (0-100 scale)
let cpu_history: Vec<f64> = vec![30.0, 45.0, 60.0, 75.0, 50.0, 40.0];

// Render the graph
let graph_lines = renderer.render(&cpu_history);

// Display each line
for line in graph_lines {
    println!("{}", line);
}
```

**Output:**
```
⣿⣾⣶⣦⣤⣄⣀
⡿⡷⡶⡦⡤⡄⡀
⠿⠷⠶⠦⠤⠄⠀
```

### Meter Rendering

Create gradient-colored meters:

```rust
use rtop::graphics::MeterRenderer;

let meter = MeterRenderer::new(20); // 20 characters wide

// Simple meter
let simple = meter.render(75); // 75%
// Output: ■■■■■■■■■■■■■■■

// Segmented meter with background
let segmented = meter.render_segmented(75);
// Output: ■■■■■■■■■■■■■■■░░░░░
```

### Box Drawing

Draw beautiful boxes with Unicode characters:

```rust
use rtop::graphics::BoxDrawer;

let drawer = BoxDrawer::new(true); // true = rounded corners

let box_lines = drawer.draw_box(
    30,                // width
    10,                // height
    Some("CPU Info")   // optional title
);

for line in box_lines {
    println!("{}", line);
}
```

**Output:**
```
╭─CPU Info─────────────────╮
│                          │
│                          │
│                          │
╰──────────────────────────╯
```

---

## 2. Creating Custom Themes

### Theme File Format

Create a file `~/.config/rtop/themes/mytheme.toml`:

```toml
name = "mytheme"

# Main UI colors (hex format)
main_fg = "#cccccc"      # Main foreground
main_bg = "#000000"      # Main background
title = "#eeeeee"        # Title text
hi_fg = "#ff6b6b"        # Highlight
selected_bg = "#4a4a4a"  # Selected item background
selected_fg = "#ffffff"  # Selected item foreground
inactive_fg = "#666666"  # Inactive elements
div_line = "#444444"     # Divider lines

# Graph elements
graph_text = "#888888"
meter_bg = "#333333"

# Box colors
cpu_box = "#4ecdc4"      # Cyan-ish
mem_box = "#ffe66d"      # Yellow-ish
net_box = "#a8dadc"      # Blue-ish
proc_box = "#f1c40f"     # Gold-ish
gpu_box = "#2ecc71"      # Green-ish

# Process colors
proc_misc = "#95e1d3"
proc_start = "#a8e6cf"
proc_mid = "#7fcdbb"
proc_end = "#41b6c4"

# CPU gradient (blue → yellow → red)
cpu_start = "#4a90e2"
cpu_mid = "#f5a623"
cpu_end = "#d0021b"

# Memory gradient (green → yellow → red)
mem_start = "#7ed321"
mem_mid = "#f8e71c"
mem_end = "#d0021b"

# Network gradient (cyan → blue → purple)
net_start = "#50e3c2"
net_mid = "#4a90e2"
net_end = "#bd10e0"

# Temperature gradient (blue → orange → red)
temp_start = "#4a90e2"
temp_mid = "#f5a623"
temp_end = "#d0021b"

# Disk gradients
download_start = "#7ed321"
download_mid = "#7ed321"
download_end = "#417505"

upload_start = "#d0021b"
upload_mid = "#d0021b"
upload_end = "#8b0000"
```

### Using Custom Theme

```bash
# Specify theme on command line
rtop --theme mytheme

# Or set in config file
# ~/.config/rtop/config.toml
[display]
theme = "mytheme"
```

### Creating Gradients Programmatically

```rust
use rtop::theme::{ColorGradient, Theme};
use ratatui::style::Color;

// Create a custom gradient
let gradient = ColorGradient::new(
    Color::Blue,      // start
    Color::Red,       // end
    101               // steps (0-100)
);

// Get color at specific percentage
let color_at_50 = gradient.at(50); // Mid-point color

// Or use theme gradients
let theme = Theme::default();
let cpu_gradient = theme.cpu_gradient();
let color = cpu_gradient.at(75); // Color for 75% CPU
```

---

## 3. GPU Monitoring Setup

### Prerequisites

**NVIDIA GPUs:**
```bash
# Install nvidia drivers and tools
sudo apt install nvidia-driver nvidia-utils
# or
sudo pacman -S nvidia nvidia-utils

# Verify nvidia-smi works
nvidia-smi
```

**AMD GPUs:**
```bash
# Install ROCm
sudo apt install rocm-smi
# or
sudo pacman -S rocm-smi-lib

# Verify rocm-smi works
rocm-smi
```

**Intel GPUs:**
```bash
# Install intel-gpu-tools
sudo apt install intel-gpu-tools
# or
sudo pacman -S intel-gpu-tools

# Verify intel_gpu_top works
sudo intel_gpu_top -l
```

### Using GPU Monitor

```rust
use rtop::monitor::GpuMonitor;

// Create GPU monitor (auto-detects GPUs)
let mut gpu_monitor = GpuMonitor::new();

// Check if GPUs were detected
if gpu_monitor.is_enabled() {
    println!("Found {} GPU(s)", gpu_monitor.gpu_count());
    println!("Vendor: {}", gpu_monitor.vendor_string());
    
    // Update GPU stats
    gpu_monitor.update();
    
    // Get GPU info
    if let Some(gpu) = gpu_monitor.get_gpu(0) {
        println!("GPU 0: {}", gpu.name);
        println!("Utilization: {}%", gpu.utilization);
        println!("Memory: {} / {} MB", 
            gpu.memory_used / 1024 / 1024,
            gpu.memory_total / 1024 / 1024
        );
        
        if let Some(temp) = gpu.temperature {
            println!("Temperature: {}°C", temp);
        }
        
        if let Some(power) = gpu.power_usage {
            println!("Power: {:.1}W", power);
        }
    }
    
    // Get historical data
    if let Some(history) = gpu_monitor.get_utilization_history(0) {
        println!("Utilization history: {:?}", history);
    }
}
```

### Keyboard Shortcuts

- `g` - Toggle all GPU displays
- `5` - Toggle GPU 0 display
- `6` - Toggle GPU 1 display
- `7` - Toggle GPU 2 display

---

## 4. Process Tree View

### Enabling Tree View

```bash
# Command line
rtop --tree-view

# Or in config
[process]
tree_view = true
```

### Using in Code

```rust
use rtop::monitor::ProcessMonitor;

let mut proc_monitor = ProcessMonitor::new();

// Toggle tree view
proc_monitor.toggle_tree_view();

// Update processes
proc_monitor.update();

// Get sorted/tree-organized processes
let processes = proc_monitor.get_sorted_processes();

for proc in processes {
    // Tree depth indicates nesting level
    let indent = "  ".repeat(proc.tree_depth);
    println!("{}├─ {} (PID: {})", indent, proc.name, proc.pid);
    
    if !proc.children.is_empty() {
        println!("{}   └─ {} children", indent, proc.children.len());
    }
}
```

### Tree View Output Example

```
PID    NAME                  CPU%   MEM      
1      systemd               0.1    2.3 MB   
├─ 123   NetworkManager      0.5    15.2 MB  
├─ 456   dockerd             2.1    120.5 MB 
│  ├─ 789  containerd        1.2    85.3 MB  
│  └─ 790  docker-proxy      0.3    12.1 MB  
└─ 234   sshd                0.0    5.1 MB   
   └─ 567  sshd              0.1    8.2 MB   
```

---

## 5. Sending Process Signals

### Available Signals

```rust
use rtop::monitor::{ProcessMonitor, ProcessSignal};

let proc_monitor = ProcessMonitor::new();

// Send SIGTERM (graceful termination)
proc_monitor.send_signal(1234, ProcessSignal::Term)?;

// Send SIGKILL (force kill)
proc_monitor.send_signal(1234, ProcessSignal::Kill)?;

// Pause process
proc_monitor.send_signal(1234, ProcessSignal::Stop)?;

// Resume process
proc_monitor.send_signal(1234, ProcessSignal::Cont)?;

// Send interrupt
proc_monitor.send_signal(1234, ProcessSignal::Int)?;

// Convenience method for killing
proc_monitor.kill_process(1234, false)?; // SIGTERM
proc_monitor.kill_process(1234, true)?;  // SIGKILL
```

### Signal Menu (Interactive)

1. Navigate to process with ↑/↓
2. Press `s` to open signal menu
3. Select signal:
   - `1` - TERM (15)
   - `2` - KILL (9)
   - `3` - INT (2)
   - `4` - HUP (1)
   - `5` - QUIT (3)
   - `6` - STOP (19)
   - `7` - CONT (18)
   - `8` - USR1 (10)
   - `9` - USR2 (12)
4. Confirm with `y`

---

## 6. Configuration Examples

### Full Configuration File

`~/.config/rtop/config.toml`:

```toml
[refresh_rates]
cpu = 1000       # CPU update interval (ms)
memory = 1000    # Memory update interval
disk = 5000      # Disk update interval
network = 1000   # Network update interval
process = 2000   # Process list update interval
temperature = 2000
gpu = 1000

[display]
# Graph settings
graph_symbol = "braille"  # braille, block, or tty
graph_height = 8
rounded_corners = true
double_borders = false

# Theme
theme = "default"
color_gradients = true

# GPU
enable_gpu = true
gpu_temp_unit = "celsius"  # or "fahrenheit"
show_gpu_memory_graph = true
show_gpu_temp_graph = true

# Process view
process_tree = false
show_process_threads = true
process_sorting = "cpu"  # cpu, memory, pid, name, user
show_process_command = true

# UI options
show_battery = true
show_temps = true
show_network_graphs = true
show_disk_io = true

[colors]
cpu_warning = 70
cpu_critical = 90
mem_warning = 80
mem_critical = 95
temp_warning = 70
temp_critical = 85
gpu_temp_warning = 75
gpu_temp_critical = 90

[export]
format = "json"  # json or csv
include_history = true

[logging]
enabled = false
level = "info"   # info, debug, trace
file = "/var/log/rtop.log"
```

### Minimal Config

```toml
[refresh_rates]
cpu = 1000
memory = 1000
process = 2000

[display]
graph_symbol = "braille"
theme = "default"
```

---

## 7. Development Guide

### Adding a New Graph Style

1. Add symbol patterns to `src/graphics.rs`:

```rust
pub const MY_STYLE_UP: [&str; 25] = [
    "▁", "▂", "▃", "▄", "▅",
    // ... 20 more patterns
];
```

2. Add to `GraphSymbol` enum:

```rust
pub enum GraphSymbol {
    Braille,
    Block,
    Tty,
    MyStyle,  // New!
}
```

3. Update `get_symbols()` method:

```rust
impl GraphSymbol {
    pub fn get_symbols(&self, inverted: bool) -> &'static [&'static str; 25] {
        match (self, inverted) {
            (GraphSymbol::MyStyle, false) => &symbols::MY_STYLE_UP,
            (GraphSymbol::MyStyle, true) => &symbols::MY_STYLE_DOWN,
            // ... existing patterns
        }
    }
}
```

### Adding New Theme Colors

1. Update `Theme` struct in `src/theme.rs`:

```rust
pub struct Theme {
    // ... existing fields
    
    pub my_new_color: String,
    pub my_gradient_start: String,
    pub my_gradient_end: String,
}
```

2. Update `Default` implementation:

```rust
impl Default for Theme {
    fn default() -> Self {
        Self {
            // ... existing defaults
            my_new_color: "#ff00ff".to_string(),
            my_gradient_start: "#00ff00".to_string(),
            my_gradient_end: "#ff0000".to_string(),
        }
    }
}
```

3. Add gradient method:

```rust
impl Theme {
    pub fn my_gradient(&self) -> ColorGradient {
        ColorGradient::from_colors(vec![
            Self::parse_color(&self.my_gradient_start),
            Self::parse_color(&self.my_gradient_end),
        ])
    }
}
```

### Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test graphics
cargo test theme
cargo test gpu

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

---

## Examples

### Complete Usage Example

```rust
use rtop::{
    graphics::{GraphRenderer, GraphSymbol, MeterRenderer},
    theme::{ThemeManager, ColorGradient},
    monitor::{CpuMonitor, GpuMonitor},
};

fn main() {
    // Initialize monitors
    let mut cpu_monitor = CpuMonitor::new();
    let mut gpu_monitor = GpuMonitor::new();
    
    // Initialize theme
    let mut theme_manager = ThemeManager::new();
    let theme = theme_manager.current();
    
    // Create renderers
    let graph_renderer = GraphRenderer::new(40, 8, GraphSymbol::Braille, false);
    let meter_renderer = MeterRenderer::new(20);
    
    // Main loop
    loop {
        // Update data
        cpu_monitor.update();
        if gpu_monitor.is_enabled() {
            gpu_monitor.update();
        }
        
        // Get CPU data
        let cpu_usage = cpu_monitor.get_usage();
        let cpu_history = cpu_monitor.get_history();
        
        // Render CPU graph
        let graph = graph_renderer.render(cpu_history);
        for line in graph {
            println!("{}", line);
        }
        
        // Render CPU meter with gradient
        let cpu_gradient = theme.cpu_gradient();
        let color = cpu_gradient.at(cpu_usage as u8);
        let meter = meter_renderer.render(cpu_usage as u8);
        println!("CPU: {}{}%", meter, cpu_usage);
        
        // GPU info
        if let Some(gpu) = gpu_monitor.get_gpu(0) {
            println!("GPU: {} @ {}%", gpu.name, gpu.utilization);
        }
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

---

## Troubleshooting

### GPU Not Detected

1. Check if nvidia-smi/rocm-smi is in PATH:
```bash
which nvidia-smi
which rocm-smi
```

2. Verify GPU tools work:
```bash
nvidia-smi --query-gpu=name --format=csv
rocm-smi --showid
```

3. Check permissions:
```bash
ls -l /dev/nvidia*
# Should be readable by your user
```

### Braille Characters Not Displaying

1. Check terminal font supports Unicode:
   - Use: Fira Code, JetBrains Mono, or Cascadia Code
   - Avoid: Basic terminal fonts

2. Set UTF-8 locale:
```bash
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
```

3. Fall back to block style:
```bash
rtop --graph-symbol block
```

### Theme Not Loading

1. Check theme file location:
```bash
ls ~/.config/rtop/themes/
```

2. Verify TOML syntax:
```bash
cat ~/.config/rtop/themes/mytheme.toml | toml-validate
```

3. Check theme name matches:
```toml
name = "mytheme"  # Must match filename (mytheme.toml)
```

---

## Resources

- [btop++ Repository](https://github.com/aristocratos/btop)
- [Ratatui Documentation](https://docs.rs/ratatui)
- [Sysinfo Crate](https://docs.rs/sysinfo)
- [Unicode Box Drawing](https://en.wikipedia.org/wiki/Box-drawing_character)
- [Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

To add new features:
1. Fork the repository
2. Create feature branch
3. Add tests
4. Update documentation
5. Submit pull request

---

## License

MIT License - See LICENSE file
