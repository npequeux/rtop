//! CPU monitoring with per-core usage tracking and historical data.
//!
//! This module provides real-time CPU usage monitoring for all cores
//! with historical data for graphing.

use std::collections::VecDeque;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// Number of historical data points to maintain for each CPU core.
const HISTORY_SIZE: usize = 61;

/// Monitors CPU usage across all cores with historical tracking.
pub struct CpuMonitor {
    system: System,
    history: Vec<VecDeque<f32>>,
}

impl CpuMonitor {
    /// Creates a new CPU monitor and performs initial CPU measurement.
    ///
    /// Sleeps briefly to ensure accurate initial CPU usage readings.
    pub fn new() -> Self {
        let mut system =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        system.refresh_cpu_all();

        // Wait a bit to get accurate CPU usage
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_cpu_all();

        let cpu_count = system.cpus().len();
        let history = vec![VecDeque::from(vec![0.0; HISTORY_SIZE]); cpu_count];

        Self { system, history }
    }

    /// Updates CPU usage measurements and historical data.
    ///
    /// Should be called periodically to maintain accurate usage tracking.
    pub fn update(&mut self) {
        self.system.refresh_cpu_all();

        for (i, cpu) in self.system.cpus().iter().enumerate() {
            if i < self.history.len() {
                self.history[i].pop_front();
                self.history[i].push_back(cpu.cpu_usage());
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_cpu_count(&self) -> usize {
        self.system.cpus().len()
    }

    #[allow(dead_code)]
    pub fn get_cpu_data(&self, index: usize) -> Option<(&str, f32, &VecDeque<f32>)> {
        if index < self.system.cpus().len() {
            let cpu = &self.system.cpus()[index];
            let usage = cpu.cpu_usage();
            let history = &self.history[index];
            Some((cpu.name(), usage, history))
        } else {
            None
        }
    }

    pub fn get_all_cpu_data(&self) -> Vec<(String, f32, Vec<f32>)> {
        let cpus = self.system.cpus();
        let mut result = Vec::with_capacity(cpus.len());

        for (i, cpu) in cpus.iter().enumerate() {
            let name = format!("CPU{} {:>5.1}%", i + 1, cpu.cpu_usage());
            let usage = cpu.cpu_usage();
            let history: Vec<f32> = self.history[i].iter().copied().collect();
            result.push((name, usage, history));
        }

        result
    }
}
