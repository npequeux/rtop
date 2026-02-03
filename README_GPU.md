# GPU Monitoring for Intel Arc GPUs

## Current Status
Your Intel Arc GPU (Lunar Lake) is detected, but detailed utilization metrics require additional setup.

## For Accurate GPU Metrics

Install Intel GPU Tools:
```bash
sudo apt install intel-gpu-tools
```

This provides `intel_gpu_top` which rtop will automatically use for:
- Real-time GPU utilization percentage
- Per-engine metrics (render, video, compute)
- Memory usage tracking

## Current Behavior
Without `intel_gpu_top`, rtop uses estimation methods:
- Monitors GPU power state changes
- Counts active DRM clients
- Shows approximate usage levels

The estimation provides basic indication but may not reflect actual load accurately.

## Run with elevated access (optional)
For additional metrics via debugfs:
```bash
sudo ./target/release/rtop
```
