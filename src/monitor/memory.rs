use std::collections::VecDeque;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

const HISTORY_SIZE: usize = 61;

pub struct MemoryMonitor {
    system: System,
    mem_history: VecDeque<f32>,
    swap_history: VecDeque<f32>,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );

        Self {
            system,
            mem_history: VecDeque::from(vec![0.0; HISTORY_SIZE]),
            swap_history: VecDeque::from(vec![0.0; HISTORY_SIZE]),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_memory();

        let mem_percent =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as f32;
        let swap_percent = if self.system.total_swap() > 0 {
            (self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.0) as f32
        } else {
            0.0
        };

        self.mem_history.pop_front();
        self.mem_history.push_back(mem_percent);

        self.swap_history.pop_front();
        self.swap_history.push_back(swap_percent);
    }

    pub fn get_memory_data(&self) -> (f32, Vec<f32>, u64, u64) {
        let percent =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as f32;
        let history: Vec<f32> = self.mem_history.iter().copied().collect();
        (
            percent,
            history,
            self.system.used_memory(),
            self.system.total_memory(),
        )
    }

    pub fn get_swap_data(&self) -> (f32, Vec<f32>, u64, u64) {
        let percent = if self.system.total_swap() > 0 {
            (self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.0) as f32
        } else {
            0.0
        };
        let history: Vec<f32> = self.swap_history.iter().copied().collect();
        (
            percent,
            history,
            self.system.used_swap(),
            self.system.total_swap(),
        )
    }
}
