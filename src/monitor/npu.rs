use std::collections::VecDeque;
/// NPU (Neural Processing Unit) monitoring support for rtop
/// Supports Intel AI Boost, AMD XDNA, and detection of AI accelerators
use std::process::Command;

const HISTORY_SIZE: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum NpuVendor {
    Intel,    // Intel AI Boost (Meteor Lake+)
    Amd,      // AMD XDNA
    Apple,    // Apple Neural Engine
    Qualcomm, // Qualcomm AI Engine
    Unknown,
}

#[derive(Debug, Clone)]
pub struct NpuInfo {
    #[allow(dead_code)]
    pub index: usize,
    pub name: String,
    pub vendor: String,
    pub utilization: u8,          // NPU utilization percentage
    pub power_usage: Option<f32>, // Power usage in Watts
    pub tops: Option<u32>,        // Performance in TOPS (Trillions of Operations Per Second)
}

impl Default for NpuInfo {
    fn default() -> Self {
        Self {
            index: 0,
            name: "Unknown NPU".to_string(),
            vendor: "Unknown".to_string(),
            utilization: 0,
            power_usage: None,
            tops: None,
        }
    }
}

pub struct NpuMonitor {
    npus: Vec<NpuInfo>,
    vendor: NpuVendor,
    enabled: bool,
    utilization_history: Vec<VecDeque<f64>>,
}

impl NpuMonitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            npus: Vec::new(),
            vendor: NpuVendor::Unknown,
            enabled: false,
            utilization_history: Vec::new(),
        };

        monitor.detect_npus();
        monitor
    }

    fn detect_npus(&mut self) {
        // Try to detect Intel NPU
        if self.detect_intel_npu() {
            self.vendor = NpuVendor::Intel;
            self.enabled = true;
            return;
        }

        // Try to detect AMD XDNA
        if self.detect_amd_npu() {
            self.vendor = NpuVendor::Amd;
            self.enabled = true;
            return;
        }

        // Check for Apple Neural Engine (macOS only)
        #[cfg(target_os = "macos")]
        if self.detect_apple_npu() {
            self.vendor = NpuVendor::Apple;
            self.enabled = true;
            return;
        }

        // Check for generic AI accelerators via sysfs
        if self.detect_generic_npu() {
            self.enabled = true;
        }
    }

    fn detect_intel_npu(&mut self) -> bool {
        // Intel NPU detection via sysfs or lspci
        if let Ok(output) = Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("VPU") || output_str.contains("Neural") {
                let npu = NpuInfo {
                    index: 0,
                    name: "Intel AI Boost".to_string(),
                    vendor: "Intel".to_string(),
                    utilization: 0,
                    power_usage: None,
                    tops: Some(10), // Intel AI Boost typically ~10 TOPS
                };

                self.npus.push(npu);
                self.utilization_history
                    .push(VecDeque::from(vec![0.0; HISTORY_SIZE]));
                return true;
            }
        }

        false
    }

    fn detect_amd_npu(&mut self) -> bool {
        // AMD XDNA detection
        if let Ok(output) = Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("XDNA") || output_str.contains("IPU") {
                let npu = NpuInfo {
                    index: 0,
                    name: "AMD XDNA".to_string(),
                    vendor: "AMD".to_string(),
                    utilization: 0,
                    power_usage: None,
                    tops: Some(16), // AMD XDNA typically ~16 TOPS
                };

                self.npus.push(npu);
                self.utilization_history
                    .push(VecDeque::from(vec![0.0; HISTORY_SIZE]));
                return true;
            }
        }

        false
    }

    #[cfg(target_os = "macos")]
    fn detect_apple_npu(&mut self) -> bool {
        // Apple Neural Engine detection via system_profiler
        if let Ok(output) = Command::new("system_profiler")
            .arg("SPHardwareDataType")
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("Neural Engine") {
                let npu = NpuInfo {
                    index: 0,
                    name: "Apple Neural Engine".to_string(),
                    vendor: "Apple".to_string(),
                    utilization: 0,
                    power_usage: None,
                    tops: Some(15), // Approximate
                };

                self.npus.push(npu);
                self.utilization_history
                    .push(VecDeque::from(vec![0.0; HISTORY_SIZE]));
                return true;
            }
        }

        false
    }

    #[cfg(not(target_os = "macos"))]
    #[allow(dead_code)]
    fn detect_apple_npu(&mut self) -> bool {
        false
    }

    fn detect_generic_npu(&mut self) -> bool {
        // Check for generic AI accelerators in /sys/class/
        if std::path::Path::new("/sys/class/accel").exists() {
            // Generic accelerator detected
            let npu = NpuInfo {
                index: 0,
                name: "AI Accelerator".to_string(),
                vendor: "Generic".to_string(),
                utilization: 0,
                power_usage: None,
                tops: None,
            };

            self.npus.push(npu);
            self.utilization_history
                .push(VecDeque::from(vec![0.0; HISTORY_SIZE]));
            return true;
        }

        false
    }

    pub fn update(&mut self) {
        if !self.enabled {
            return;
        }

        match self.vendor {
            NpuVendor::Intel => self.update_intel(),
            NpuVendor::Amd => self.update_amd(),
            NpuVendor::Apple => self.update_apple(),
            _ => self.update_generic(),
        }
    }

    fn update_intel(&mut self) {
        // Try to read Intel NPU utilization from sysfs or performance counters
        // This is a placeholder - actual implementation would need Intel NPU drivers
        for (i, npu) in self.npus.iter_mut().enumerate() {
            // Simulated for now - would need actual Intel NPU API
            npu.utilization = 0;

            if let Some(history) = self.utilization_history.get_mut(i) {
                history.pop_front();
                history.push_back(npu.utilization as f64);
            }
        }
    }

    fn update_amd(&mut self) {
        // Try to read AMD XDNA utilization
        // Placeholder - would need AMD XDNA drivers/API
        for (i, npu) in self.npus.iter_mut().enumerate() {
            npu.utilization = 0;

            if let Some(history) = self.utilization_history.get_mut(i) {
                history.pop_front();
                history.push_back(npu.utilization as f64);
            }
        }
    }

    fn update_apple(&mut self) {
        // Apple Neural Engine monitoring
        // Placeholder - would need Apple frameworks
        for (i, npu) in self.npus.iter_mut().enumerate() {
            npu.utilization = 0;

            if let Some(history) = self.utilization_history.get_mut(i) {
                history.pop_front();
                history.push_back(npu.utilization as f64);
            }
        }
    }

    fn update_generic(&mut self) {
        // Generic accelerator monitoring
        for (i, npu) in self.npus.iter_mut().enumerate() {
            npu.utilization = 0;

            if let Some(history) = self.utilization_history.get_mut(i) {
                history.pop_front();
                history.push_back(npu.utilization as f64);
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn npu_count(&self) -> usize {
        self.npus.len()
    }

    pub fn get_all_npus(&self) -> &[NpuInfo] {
        &self.npus
    }

    #[allow(dead_code)]
    pub fn vendor(&self) -> NpuVendor {
        self.vendor
    }

    pub fn vendor_string(&self) -> &str {
        match self.vendor {
            NpuVendor::Intel => "Intel",
            NpuVendor::Amd => "AMD",
            NpuVendor::Apple => "Apple",
            NpuVendor::Qualcomm => "Qualcomm",
            NpuVendor::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npu_monitor_creation() {
        let monitor = NpuMonitor::new();
        // Should not panic
        let _ = monitor.npu_count();
    }
}
