use sysinfo::{System, CpuRefreshKind, RefreshKind};
use std::collections::VecDeque;

const HISTORY_SIZE: usize = 61;

pub struct CpuMonitor {
    system: System,
    history: Vec<VecDeque<f32>>,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );
        system.refresh_cpu_all();
        
        // Wait a bit to get accurate CPU usage
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_cpu_all();
        
        let cpu_count = system.cpus().len();
        let history = vec![VecDeque::from(vec![0.0; HISTORY_SIZE]); cpu_count];
        
        Self { system, history }
    }
    
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
        self.system
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                let name = format!("CPU{} {:>5.1}%", i + 1, cpu.cpu_usage());
                let usage = cpu.cpu_usage();
                let history: Vec<f32> = self.history[i].iter().copied().collect();
                (name, usage, history)
            })
            .collect()
    }
}
