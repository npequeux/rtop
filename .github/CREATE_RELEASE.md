# Creating GitHub Release for v3.0.0

## Steps to create the release:

1. Go to https://github.com/npequeux/mytop/releases/new
2. Click "Choose a tag" and select `v3.0.0`
3. Set Release title: `v3.0.0 - GPU & NPU Support`
4. Use the following release notes:

---

## 🚀 Major Release: GPU & NPU Monitoring Support

rtop v3.0.0 brings comprehensive hardware acceleration monitoring to your terminal!

### ✨ New Features

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

- **Enhanced Visuals**:
  - Braille markers for smoother CPU graphs
  - Improved memory/swap visualization with overlay
  - Dynamic layout adapts to available hardware
  - Optimized temperature display in compact column

### 🔧 Improvements

- Cleaner codebase with removed unused dependencies
- Better Intel Arc GPU support via sysfs DRM interface
- Fallback GPU monitoring when specialized tools aren't available
- Reorganized UI layout for better space utilization

### 🐛 Bug Fixes

- Fixed all compilation warnings
- Improved GPU detection reliability
- Better error handling for missing hardware

### 📦 Installation

```bash
# From source
git clone https://github.com/npequeux/mytop.git
cd mytop
cargo build --release
sudo cp target/release/rtop /usr/local/bin/
```

### 🎯 Requirements

- Rust 1.70+
- Linux (primary support)
- Optional: `intel_gpu_top` for accurate Intel GPU metrics
- Optional: `nvidia-smi` for NVIDIA GPU support
- Optional: `rocm-smi` for AMD GPU support

---

5. Check "Set as the latest release"
6. Click "Publish release"

The release will be live and visible to all visitors!
