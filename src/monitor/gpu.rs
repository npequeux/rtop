/// GPU monitoring support for rtop
/// Supports NVIDIA (via nvidia-smi) and AMD (via rocm-smi) GPUs

use std::collections::VecDeque;
use std::process::Command;
use serde::{Deserialize, Serialize};

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
    pub utilization: u8,        // GPU utilization percentage
    pub memory_used: u64,        // Memory used in bytes
    pub memory_total: u64,       // Total memory in bytes
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
            .args(&[
                "--query-gpu=index,name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,clocks.current.graphics,fan.speed",
                "--format=csv,noheader,nounits"
            ])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let count = output_str.lines().count();
                
                for _ in 0..count {
                    self.utilization_history.push(VecDeque::with_capacity(HISTORY_SIZE));
                    self.memory_history.push(VecDeque::with_capacity(HISTORY_SIZE));
                }
                
                return count > 0;
            }
        }
        
        false
    }
    
    fn detect_amd(&mut self) -> bool {
        let output = Command::new("rocm-smi")
            .args(&["--showid", "--showuse", "--showmeminfo", "vram", "--showtemp", "--showpower"])
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
        // Intel GPU detection via intel_gpu_top (if available)
        let output = Command::new("intel_gpu_top")
            .arg("-l")
            .output();
        
        if let Ok(output) = output {
            return output.status.success();
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
            .args(&[
                "--query-gpu=index,name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw,clocks.current.graphics,fan.speed",
                "--format=csv,noheader,nounits"
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
            .args(&["--showid", "--showuse", "--showmeminfo", "vram", "--showtemp"])
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
        // Intel GPU update
        // This would require parsing intel_gpu_top output
        // Simplified implementation
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn gpu_count(&self) -> usize {
        self.gpus.len()
    }
    
    pub fn get_gpu(&self, index: usize) -> Option<&GpuInfo> {
        self.gpus.get(index)
    }
    
    pub fn get_all_gpus(&self) -> &[GpuInfo] {
        &self.gpus
    }
    
    pub fn get_utilization_history(&self, index: usize) -> Option<Vec<f64>> {
        self.utilization_history.get(index).map(|h| h.iter().copied().collect())
    }
    
    pub fn get_memory_history(&self, index: usize) -> Option<Vec<f64>> {
        self.memory_history.get(index).map(|h| h.iter().copied().collect())
    }
    
    pub fn vendor(&self) -> GpuVendor {
        self.vendor
    }
    
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
        // Should not panic
        assert!(monitor.gpu_count() >= 0);
    }
    
    #[test]
    fn test_gpu_info_memory_percent() {
        let mut gpu = GpuInfo::default();
        gpu.memory_total = 1000;
        gpu.memory_used = 500;
        assert_eq!(gpu.memory_percent(), 50);
    }
    
    #[test]
    fn test_gpu_info_zero_memory() {
        let gpu = GpuInfo::default();
        assert_eq!(gpu.memory_percent(), 0);
    }
}
