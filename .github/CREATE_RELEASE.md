# Creating GitHub Release for v3.0.2

## Steps to create the release

1. Go to <https://github.com/npequeux/rtop/releases/new>
2. Click "Choose a tag" and create a new tag `v3.0.2`
3. Set Release title: `v3.0.2 - Advanced Monitoring & Enhanced UI`
4. Use the following release notes:

---

## 🚀 Major Release: Advanced Monitoring & Enhanced UI

rtop v3.0.2 brings comprehensive hardware acceleration monitoring and btop++-inspired features to your terminal!

### ✨ New Features

#### btop++-inspired Enhancements
- **Advanced Graphics**: Braille/Unicode graphs with 2x data density
- **Theme System**: Color gradients and customizable TOML themes
- **Process Tree View**: Hierarchical process display
- **Enhanced Signals**: 9 different process signals (TERM, KILL, STOP, CONT, etc.)
- **Sophisticated UI**: Rounded corners, gradient meters, multiple graph styles
- **Extended Config**: Per-component customization and theming

#### GPU & NPU Monitoring
- **GPU Monitoring**: Full support for Intel Arc, NVIDIA, and AMD GPUs
  - GPU utilization percentage
  - Memory usage and capacity
  - Temperature monitoring
  - Power consumption
  - Clock speeds
  - Fan speed tracking

- **NPU Monitoring**: Neural Processing Unit detection
  - Intel VPU support
  - AMD XDNA support
  - Apple Neural Engine detection
  - AI accelerator utilization tracking

#### Interactive Features
- **Mouse Support**: Scroll process list with mouse wheel, click to select processes
- **Multi-page Navigation**: F2-F5 keys to switch between Overview/Processes/Network/Storage pages
- **Process Scrolling**: Arrow keys (↑↓), PageUp/PageDown, Home/End for process navigation
- **Process Kill**: Press `k` to kill selected process with confirmation dialog
- **Regex Filtering**: Press `/` to enter filter mode with regex pattern matching

#### Hardware Monitoring
- **Battery Monitor**: Display battery percentage, charging status, and time remaining
- **Disk I/O Stats**: Monitor read/write bytes per second (Linux /proc/diskstats)
- **Per-interface Network**: Individual network interface statistics

### 🔧 Improvements

- **Enhanced Visuals**: Braille markers for smoother CPU graphs
- **Improved Layout**: Memory/swap visualization with overlay
- **Dynamic UI**: Layout adapts to available hardware
- **Optimized Display**: Temperature display in compact column
- **Cleaner Codebase**: Removed unused dependencies
- **Better GPU Support**: Intel Arc GPU support via sysfs DRM interface
- **Fallback Monitoring**: GPU monitoring when specialized tools aren't available
- **Reorganized UI**: Better space utilization across all views

### 🐛 Bug Fixes

- Fixed all compilation warnings
- Improved GPU detection reliability
- Better error handling for missing hardware
- Enhanced terminal state restoration on panic
- Fixed race conditions in update loops
- Improved sensor detection edge cases

### 📦 Installation

```bash
# From source
git clone https://github.com/npequeux/rtop.git
cd rtop
cargo build --release
sudo cp target/release/rtop /usr/local/bin/
```

### 🎯 Requirements

- Rust 1.88+
- Linux (primary support), macOS, Windows
- Optional: `intel_gpu_top` for accurate Intel GPU metrics
- Optional: `nvidia-smi` for NVIDIA GPU support
- Optional: `rocm-smi` for AMD GPU support

### 📚 Documentation

- Comprehensive README with all features
- ENHANCED_FEATURES.md for btop++-inspired features
- Example configuration file
- Troubleshooting guide
- Development instructions

---

## Publishing the Release

1. Check "Set as the latest release"
2. Click "Publish release"

The release will be live and visible to all visitors!
