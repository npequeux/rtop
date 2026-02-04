use serde::{Deserialize, Serialize};
/// GPU monitoring support for rtop
/// Supports NVIDIA (via nvidia-smi) and AMD (via rocm-smi) GPUs
use std::collections::VecDeque;
use std::path::Path;
use std::process::Command;

const HISTORY_SIZE: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub index: usize,
    pub name: String,
    pub vendor: String,
    pub utilization: u8,          // GPU utilization percentage
    pub memory_used: u64,         // Memory used in bytes
    pub memory_total: u64,        // Total memory in bytes
    pub temperature: Option<i32>, // Temperature in Celsius
    pub power_usage: Option<f32>, // Power usage in Watts
    pub clock_speed: Option<u32>, // Clock speed in MHz
    pub fan_speed: Option<u8>,    // Fan speed percentage
}

impl Default for GpuInfo {
    fn default() -> Self {
        Self {
            index: 0,
            name: "Unknown GPU".to_string(),
            vendor: "Unknown".to_string(),
            utilization: 0,
            memory_used: 0,
            memory_total: 0,
            temperature: None,
            power_usage: None,
            clock_speed: None,
            fan_speed: None,
        }
    }
}

impl GpuInfo {
    pub fn memory_percent(&self) -> u8 {
        if self.memory_total == 0 {
            0
        } else {
            ((self.memory_used as f64 / self.memory_total as f64) * 100.0) as u8
        }
    }
}

pub struct GpuMonitor {
    gpus: Vec<GpuInfo>,
    vendor: GpuVendor,
    utilization_history: Vec<VecDeque<f64>>,
    memory_history: Vec<VecDeque<f64>>,
    enabled: bool,
}

impl GpuMonitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            gpus: Vec::new(),
            vendor: GpuVendor::Unknown,
            utilization_history: Vec::new(),
            memory_history: Vec::new(),
            enabled: false,
        };

        monitor.detect_gpus();
        monitor
    }

    fn detect_gpus(&mut self) {
        // Try NVIDIA first
        if self.detect_nvidia() {
            self.vendor = GpuVendor::Nvidia;
            self.enabled = true;
            return;
        }

        // Try AMD
        if self.detect_amd() {
            self.vendor = GpuVendor::Amd;
            self.enabled = true;
            return;
        }

        // Try Intel
        if self.detect_intel() {
            self.vendor = GpuVendor::Intel;
            self.enabled = true;
        }
    }

    fn detect_nvidia(&mut self) -> bool {
        let output = Command::new("nvidia-smi")
            .args([
                "--query-gpu=index,name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,clocks.current.graphics,fan.speed",
                "--format=csv,noheader,nounits",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let count = output_str.lines().count();

                for _ in 0..count {
                    self.utilization_history
                        .push(VecDeque::with_capacity(HISTORY_SIZE));
                    self.memory_history
                        .push(VecDeque::with_capacity(HISTORY_SIZE));
                }

                return count > 0;
            }
        }

        false
    }

    fn detect_amd(&mut self) -> bool {
        let output = Command::new("rocm-smi")
            .args([
                "--showid",
                "--showuse",
                "--showmeminfo",
                "vram",
                "--showtemp",
                "--showpower",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                // Basic detection - actual parsing would be more complex
                let output_str = String::from_utf8_lossy(&output.stdout);
                return output_str.contains("GPU");
            }
        }

        false
    }

    fn detect_intel(&mut self) -> bool {
        // Intel GPU detection via sysfs DRM interface
        use std::fs;

        // Check for Intel GPUs via /sys/class/drm
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("card"))
                {
                    let vendor_path = path.join("device/vendor");
                    if vendor_path.exists() {
                        if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                            // 0x8086 is Intel's vendor ID
                            if vendor.trim() == "0x8086" {
                                // Check if it has render capabilities (is a GPU, not just display)
                                let device_path = path.join("device/device");
                                if device_path.exists() {
                                    self.utilization_history
                                        .push(VecDeque::with_capacity(HISTORY_SIZE));
                                    self.memory_history
                                        .push(VecDeque::with_capacity(HISTORY_SIZE));
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn update(&mut self) {
        if !self.enabled {
            return;
        }

        match self.vendor {
            GpuVendor::Nvidia => self.update_nvidia(),
            GpuVendor::Amd => self.update_amd(),
            GpuVendor::Intel => self.update_intel(),
            GpuVendor::Unknown => {}
        }
    }

    fn update_nvidia(&mut self) {
        let output = Command::new("nvidia-smi")
            .args([
                "--query-gpu=index,name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,clocks.current.graphics,fan.speed",
                "--format=csv,noheader,nounits",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                self.gpus.clear();

                for (idx, line) in output_str.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 9 {
                        let gpu_info = GpuInfo {
                            index: parts[0].parse().unwrap_or(idx),
                            name: parts[1].to_string(),
                            vendor: "NVIDIA".to_string(),
                            utilization: parts[2].parse().unwrap_or(0),
                            memory_used: parts[3].parse::<u64>().unwrap_or(0) * 1024 * 1024, // MiB to bytes
                            memory_total: parts[4].parse::<u64>().unwrap_or(0) * 1024 * 1024,
                            temperature: parts[5].parse().ok(),
                            power_usage: parts[6].parse().ok(),
                            clock_speed: parts[7].parse().ok(),
                            fan_speed: parts[8].parse().ok(),
                        };

                        // Update history
                        if idx < self.utilization_history.len() {
                            let hist = &mut self.utilization_history[idx];
                            hist.push_back(gpu_info.utilization as f64);
                            if hist.len() > HISTORY_SIZE {
                                hist.pop_front();
                            }

                            let mem_hist = &mut self.memory_history[idx];
                            mem_hist.push_back(gpu_info.memory_percent() as f64);
                            if mem_hist.len() > HISTORY_SIZE {
                                mem_hist.pop_front();
                            }
                        }

                        self.gpus.push(gpu_info);
                    }
                }
            }
        }
    }

    fn update_amd(&mut self) {
        // AMD GPU update via rocm-smi
        // This is a simplified implementation - full implementation would parse rocm-smi output
        let output = Command::new("rocm-smi")
            .args([
                "--showid",
                "--showuse",
                "--showmeminfo",
                "vram",
                "--showtemp",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                // Parse rocm-smi output (simplified)
                let _output_str = String::from_utf8_lossy(&output.stdout);
                // TODO: Implement proper parsing
            }
        }
    }

    fn update_intel(&mut self) {
        use std::fs;

        // Try intel_gpu_top first for accurate metrics
        if let Some(intel_info) = Self::read_intel_gpu_top() {
            self.gpus.clear();
            for (idx, info) in intel_info.iter().enumerate() {
                if idx < self.utilization_history.len() {
                    let hist = &mut self.utilization_history[idx];
                    hist.push_back(info.utilization as f64);
                    if hist.len() > HISTORY_SIZE {
                        hist.pop_front();
                    }

                    let mem_hist = &mut self.memory_history[idx];
                    mem_hist.push_back(info.memory_percent() as f64);
                    if mem_hist.len() > HISTORY_SIZE {
                        mem_hist.pop_front();
                    }
                }
                self.gpus.push(info.clone());
            }
            return;
        }

        // Fallback: Read Intel GPU info from sysfs
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            self.gpus.clear();
            let mut gpu_idx = 0;

            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") && !name.contains("-") {
                        let vendor_path = path.join("device/vendor");
                        if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                            if vendor.trim() == "0x8086" {
                                // Get device name from lspci
                                let device_path = path.join("device/device");
                                let device_id = fs::read_to_string(&device_path)
                                    .unwrap_or_default()
                                    .trim()
                                    .to_string();

                                let gpu_name = Self::get_intel_gpu_name(&device_id);

                                // Try to estimate utilization from frequency
                                let utilization = Self::estimate_intel_utilization();

                                let mem_total = Self::read_drm_memory(&path);

                                let gpu_info = GpuInfo {
                                    index: gpu_idx,
                                    name: gpu_name,
                                    vendor: "Intel".to_string(),
                                    utilization,
                                    memory_used: 0, // Not available without intel_gpu_top
                                    memory_total: mem_total,
                                    temperature: Self::read_drm_temp(&path),
                                    power_usage: None,
                                    clock_speed: Self::read_intel_frequency(&path),
                                    fan_speed: None,
                                };

                                // Update history
                                if gpu_idx < self.utilization_history.len() {
                                    let hist = &mut self.utilization_history[gpu_idx];
                                    hist.push_back(utilization as f64);
                                    if hist.len() > HISTORY_SIZE {
                                        hist.pop_front();
                                    }

                                    let mem_hist = &mut self.memory_history[gpu_idx];
                                    mem_hist.push_back(0.0);
                                    if mem_hist.len() > HISTORY_SIZE {
                                        mem_hist.pop_front();
                                    }
                                }

                                self.gpus.push(gpu_info);
                                gpu_idx += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_intel_gpu_name(device_id: &str) -> String {
        // Try to get name from lspci
        if let Ok(output) = Command::new("lspci").arg("-d").arg("8086:").output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("VGA") || line.contains("Display") || line.contains("3D") {
                        // Extract GPU name after the colon
                        if let Some(name_part) = line.split(':').nth(2) {
                            return name_part.trim().to_string();
                        }
                    }
                }
            }
        }

        format!("Intel GPU {}", device_id)
    }

    fn read_drm_memory(card_path: &Path) -> u64 {
        use std::fs;

        // Try to read memory info from various possible locations
        let mem_paths = [
            "device/mem_info_vram_total",
            "device/mem_info_gtt_total",
            "gt/gt0/addr_range",
        ];

        for mem_path in &mem_paths {
            let full_path = card_path.join(mem_path);
            if let Ok(mem_str) = fs::read_to_string(&full_path) {
                if let Ok(mem) = mem_str.trim().parse::<u64>() {
                    return mem;
                }
            }
        }

        // For Intel integrated GPUs with shared system memory, show a placeholder
        // Modern Intel iGPUs dynamically allocate from system RAM
        0 // Will show "N/A" in UI
    }

    fn read_drm_temp(card_path: &Path) -> Option<i32> {
        use std::fs;

        // Try various hwmon paths for temperature
        if let Ok(entries) = fs::read_dir(card_path.join("device/hwmon")) {
            for entry in entries.flatten() {
                let temp_path = entry.path().join("temp1_input");
                if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                    if let Ok(temp_millis) = temp_str.trim().parse::<i32>() {
                        return Some(temp_millis / 1000); // Convert from millidegrees
                    }
                }
            }
        }

        None
    }

    fn read_intel_frequency(card_path: &Path) -> Option<u32> {
        use std::fs;

        // Try to read current frequency
        let freq_paths = ["gt/gt0/rps_cur_freq_mhz", "gt_cur_freq_mhz"];

        for freq_path in &freq_paths {
            let full_path = card_path.join(freq_path);
            if let Ok(freq_str) = fs::read_to_string(&full_path) {
                if let Ok(freq) = freq_str.trim().parse::<u32>() {
                    return Some(freq);
                }
            }
        }

        None
    }

    fn read_intel_gpu_top() -> Option<Vec<GpuInfo>> {
        // Try to use intel_gpu_top in JSON mode for accurate metrics
        let output = Command::new("intel_gpu_top")
            .args(["-J", "-s", "100"]) // JSON output, 100ms sample
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                return Self::parse_intel_gpu_top_json(&output_str);
            }
        }

        None
    }

    fn parse_intel_gpu_top_json(json_str: &str) -> Option<Vec<GpuInfo>> {
        // Parse intel_gpu_top JSON output
        // Format: {"period":{"duration":0.1,"unit":"ms"},"engines":{"Render/3D/0":{"busy":15.5,...},...}}
        use serde_json::Value;

        if let Ok(data) = serde_json::from_str::<Value>(json_str) {
            let mut gpus = Vec::new();
            let gpu_name = Self::get_intel_gpu_name("");

            // Extract render engine utilization
            let mut total_util = 0.0;
            let mut count = 0;

            if let Some(engines) = data.get("engines").and_then(|e| e.as_object()) {
                for (_engine_name, engine_data) in engines.iter() {
                    if let Some(busy) = engine_data.get("busy").and_then(|b| b.as_f64()) {
                        total_util += busy;
                        count += 1;
                    }
                }
            }

            let utilization = if count > 0 {
                (total_util / count as f64).min(100.0) as u8
            } else {
                0
            };

            gpus.push(GpuInfo {
                index: 0,
                name: gpu_name,
                vendor: "Intel".to_string(),
                utilization,
                memory_used: 0,
                memory_total: 0,
                temperature: None,
                power_usage: None,
                clock_speed: None,
                fan_speed: None,
            });

            return Some(gpus);
        }

        None
    }

    fn estimate_intel_utilization() -> u8 {
        // Estimate GPU utilization by checking various activity indicators
        use std::fs;
        use std::time::Duration;

        // Method 1: Check runtime active time (if available)
        let runtime_path = "/sys/class/drm/card0/power/runtime_active_time";
        if let Ok(before_str) = fs::read_to_string(runtime_path) {
            if let Ok(before) = before_str.trim().parse::<u64>() {
                std::thread::sleep(Duration::from_millis(100));
                if let Ok(after_str) = fs::read_to_string(runtime_path) {
                    if let Ok(after) = after_str.trim().parse::<u64>() {
                        // Calculate percentage of time active
                        let delta = after.saturating_sub(before);
                        if delta > 0 {
                            // delta is in microseconds, we sampled for 100ms = 100000us
                            let percent = ((delta as f64 / 100000.0) * 100.0).min(100.0) as u8;
                            return percent;
                        }
                    }
                }
            }
        }

        // Method 2: Check DRM clients
        if let Ok(entries) = fs::read_dir("/sys/kernel/debug/dri/0/") {
            let mut active_clients = 0;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("client") {
                    active_clients += 1;
                }
            }

            if active_clients > 0 {
                // Rough estimate: each client might use ~20% on average
                return (active_clients * 20).min(100) as u8;
            }
        }

        // Method 3: Check if render node is open
        if fs::metadata("/dev/dri/renderD128")
            .map(|m| m.len())
            .unwrap_or(0)
            > 0
        {
            // Render node exists and is accessible - GPU might be in use
            // Return a conservative estimate
            return 10;
        }

        0
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn gpu_count(&self) -> usize {
        self.gpus.len()
    }

    #[allow(dead_code)]
    pub fn get_gpu(&self, index: usize) -> Option<&GpuInfo> {
        self.gpus.get(index)
    }

    #[allow(dead_code)]
    pub fn get_all_gpus(&self) -> &[GpuInfo] {
        &self.gpus
    }

    #[allow(dead_code)]
    pub fn get_utilization_history(&self, index: usize) -> Option<Vec<f64>> {
        self.utilization_history
            .get(index)
            .map(|h| h.iter().copied().collect())
    }

    #[allow(dead_code)]
    pub fn get_memory_history(&self, index: usize) -> Option<Vec<f64>> {
        self.memory_history
            .get(index)
            .map(|h| h.iter().copied().collect())
    }

    #[allow(dead_code)]
    pub fn vendor(&self) -> GpuVendor {
        self.vendor
    }

    #[allow(dead_code)]
    pub fn vendor_string(&self) -> &str {
        match self.vendor {
            GpuVendor::Nvidia => "NVIDIA",
            GpuVendor::Amd => "AMD",
            GpuVendor::Intel => "Intel",
            GpuVendor::Unknown => "Unknown",
        }
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_monitor_creation() {
        let monitor = GpuMonitor::new();
        // Should not panic - gpu_count returns usize which is always >= 0
        let _ = monitor.gpu_count();
    }

    #[test]
    fn test_gpu_info_memory_percent() {
        let gpu = GpuInfo {
            memory_total: 1000,
            memory_used: 500,
            ..GpuInfo::default()
        };
        assert_eq!(gpu.memory_percent(), 50);
    }

    #[test]
    fn test_gpu_info_zero_memory() {
        let gpu = GpuInfo::default();
        assert_eq!(gpu.memory_percent(), 0);
    }
}
