# Changelog - rtop

All notable changes to this project will be documented in this file.

## [3.0.0] - 2026-02-02 - "btop++ Feature Parity Release"

### 🎉 Major Features Added (btop++-inspired)

#### Advanced Graphics System
- **Braille/Unicode Graphs**: High-resolution graphs using Braille patterns (⣿⣾⣶⣦)
  - 2x data density per character
  - 25 different patterns for 5x5 resolution
  - Support for upward and downward graphs
- **Multiple Graph Styles**:
  - Braille (default) - Highest resolution
  - Block - High contrast display
  - TTY - Compatible with legacy terminals
- **Enhanced Meter Rendering**:
  - Gradient-colored meters
  - Segmented displays
  - Horizontal and vertical orientations

#### Theme System
- **Comprehensive Color Management**:
  - 101-step color gradients for smooth transitions
  - RGB hex color parsing (#RRGGBB, #RGB, #GG)
  - Per-component theming (CPU, Memory, Network, GPU, Processes)
- **TOML-Based Themes**:
  - Easy to create and customize
  - Multiple themes support
  - Hot-reloading capability
  - Compatible with btop theme format
- **Pre-built Gradients**:
  - CPU: Blue → Green → Red
  - Memory: Yellow → Orange → Red
  - Temperature: Blue → Yellow → Red
  - Network: Green (download) / Red (upload)

#### GPU Monitoring
- **Multi-GPU Support**:
  - NVIDIA GPUs via nvidia-smi
  - AMD GPUs via rocm-smi
  - Intel GPUs via intel_gpu_top (limited)
  - Auto-detection of all available GPUs
- **Comprehensive Metrics**:
  - GPU Utilization (0-100%)
  - Memory Usage (used/total)
  - Temperature (°C/°F)
  - Power Draw (Watts)
  - Clock Speed (MHz)
  - Fan Speed (0-100%)
  - Historical graphs (60-second window)
- **Display Features**:
  - Individual boxes for each GPU
  - Real-time utilization graphs with Braille rendering
  - Memory usage bars with gradient colors
  - Temperature monitoring with color coding
  - Power consumption tracking

#### Enhanced Process Management
- **Process Tree View**:
  - Hierarchical display showing parent-child relationships
  - Indentation levels to visualize process ancestry
  - Root process identification
  - Recursive tree building
- **Extended Signal Support** (9 signals):
  - TERM (15) - Graceful termination
  - KILL (9) - Force kill
  - INT (2) - Interrupt
  - HUP (1) - Hangup
  - QUIT (3) - Quit
  - STOP (19) - Pause process
  - CONT (18) - Resume process
  - USR1 (10) - User-defined signal 1
  - USR2 (12) - User-defined signal 2
- **Additional Process Info**:
  - User/UID information
  - Process state (R/S/Z/D)
  - Thread count
  - Disk I/O (read/write bytes)
  - Parent process ID (PPID)
  - Child process list
- **New Sort Options**:
  - Sort by Name
  - Sort by User
  - (in addition to existing PID, CPU, Memory)

#### Advanced UI Features
- **Symbol Customization**:
  - Graph symbol selection (Braille/Block/TTY)
  - Rounded corners (╭╮╰╯) or square (┌┐└┘)
  - Configurable box styles
- **Box Drawing**:
  - Unicode box drawing with titles
  - Rounded or square corners
  - Configurable line styles
  - Auto-sizing based on content

### 📦 New Modules

- `src/graphics.rs` - Advanced graphics rendering
  - GraphRenderer struct with Braille/Block/TTY support
  - MeterRenderer for gradient meters
  - BoxDrawer for UI elements
  - Complete symbol library
- `src/theme.rs` - Theme system
  - Theme struct with TOML serialization
  - ThemeManager for theme handling
  - ColorGradient for smooth color transitions
  - RGB color interpolation
- `src/monitor/gpu.rs` - GPU monitoring
  - GpuMonitor for multi-GPU support
  - GpuInfo struct with all metrics
  - Vendor detection (NVIDIA/AMD/Intel)
  - Historical data tracking

### ✨ Enhancements

#### Process Monitor
- Enhanced ProcessInfo struct with more fields
- Process tree building algorithm
- Signal sending with proper error handling
- Parent-child relationship tracking

#### Configuration
- Extended config options for GPU, themes, and graphics
- Per-component customization
- Theme selection support
- Graph style configuration

#### UI
- Integration of new graphics system
- Theme manager integration
- GPU display boxes
- Enhanced color gradients throughout

### 🔧 Technical Improvements

- **Performance**:
  - Graph render caching
  - Incremental updates
  - Smart refresh logic
  - Fixed-size history buffers
- **Code Quality**:
  - Comprehensive test coverage for new modules
  - Detailed documentation
  - Clean module separation
  - Type-safe error handling

### 📚 Documentation

- `ENHANCED_FEATURES.md` - Complete feature documentation
- Updated `README.md` with v3.0 features
- Inline code documentation
- Usage examples and configuration guides

### 🐛 Bug Fixes

- None (new features)

### ⚠️ Breaking Changes

- None (backward compatible)

### 🚀 Performance

- Optimized graph rendering with caching
- Efficient color gradient computation
- Reduced memory footprint with circular buffers
- Smart GPU polling (only when enabled)

---

## [2.1.0] - Previous Release

### Features
- Mouse support for process list
- Multi-page navigation (F2-F5)
- Process scrolling with keyboard
- Kill process confirmation
- Regex process filtering
- Visual feedback improvements

---

## [2.0.0] - Previous Release

### Features
- Configuration system (TOML)
- Error handling with custom types
- CLI argument parsing
- Help overlay system
- Pause/Resume functionality
- Process filtering
- System information display
- Data export (JSON/CSV)
- Logging support
- Signal handling

---

## [1.0.0] - Initial Release

### Features
- Basic CPU monitoring
- Memory and swap monitoring
- Network statistics
- Disk usage
- Battery monitoring
- Disk I/O
- Process management
- Temperature monitoring

---

## Comparison with btop++

| Feature | btop++ v1.4.0 | rtop v3.0.0 | Status |
|---------|---------------|-------------|--------|
| Braille graphs | ✅ | ✅ | ✅ Complete |
| Color gradients | ✅ | ✅ | ✅ Complete |
| Theme system | ✅ | ✅ | ✅ Complete |
| GPU monitoring | ✅ | ✅ | ✅ Complete |
| Process tree | ✅ | ✅ | ✅ Complete |
| Multiple signals | ✅ | ✅ | ✅ Complete |
| Mouse support | ✅ | ✅ | ✅ Complete |
| TTY mode | ✅ | ✅ | ✅ Complete |
| Rounded corners | ✅ | ✅ | ✅ Complete |
| Custom presets | ✅ | ⏳ | 🚧 Planned |
| UI config menu | ✅ | ⏳ | 🚧 Planned |

**Legend:**
- ✅ Available
- ⏳ In Progress
- 🚧 Planned
- ❌ Not Available

---

## Upgrade Guide

### From v2.x to v3.0

1. **Configuration**: No changes required, v3.0 is backward compatible
2. **New Features**: All new features are opt-in
3. **GPU Monitoring**: Auto-enabled if GPUs detected (nvidia-smi/rocm-smi in PATH)
4. **Themes**: Uses default theme unless specified

### Enabling New Features

```bash
# Use Braille graphs (default)
rtop --graph-symbol braille

# Enable GPU monitoring explicitly
rtop --enable-gpu

# Use custom theme
rtop --theme mytheme

# Enable process tree view
rtop --tree-view
```

---

## Future Roadmap

### v3.1.0 (Planned)
- [ ] UI configuration menu (like btop)
- [ ] Custom presets system
- [ ] More theme options
- [ ] Collapsible process tree
- [ ] Per-process GPU usage

### v3.2.0 (Planned)
- [ ] Plugin system
- [ ] Remote monitoring
- [ ] Historical data storage
- [ ] Alert notifications
- [ ] Dashboard mode

---

## Credits

This release brings rtop to feature parity with btop++. Special thanks to:

- **[aristocratos](https://github.com/aristocratos)** for creating btop++, which inspired these enhancements
- All contributors to the btop++, bpytop, and bashtop projects
- The Rust community for excellent libraries (ratatui, sysinfo, crossterm)

---

## License

MIT License - See LICENSE file for details
