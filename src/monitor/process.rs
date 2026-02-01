use sysinfo::{System, ProcessRefreshKind, RefreshKind, ProcessesToUpdate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Pid,
    Cpu,
    Memory,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

pub struct ProcessMonitor {
    system: System,
    sort_order: SortOrder,
    reverse: bool,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        );
        system.refresh_processes(ProcessesToUpdate::All, true);
        
        Self {
            system,
            sort_order: SortOrder::Cpu,
            reverse: false,
        }
    }
    
    pub fn update(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
    }
    
    pub fn set_sort_order(&mut self, order: SortOrder) {
        if self.sort_order == order {
            self.reverse = !self.reverse;
        } else {
            self.sort_order = order;
            self.reverse = false;
        }
    }
    
    pub fn get_sorted_processes(&self) -> Vec<ProcessInfo> {
        let mut processes: Vec<ProcessInfo> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect();
        
        match self.sort_order {
            SortOrder::Pid => {
                processes.sort_by_key(|p| p.pid);
            }
            SortOrder::Cpu => {
                processes.sort_by(|a, b| {
                    b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortOrder::Memory => {
                processes.sort_by(|a, b| b.memory.cmp(&a.memory));
            }
        }
        
        if self.reverse {
            processes.reverse();
        }
        
        processes
    }
}
