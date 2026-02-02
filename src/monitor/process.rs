use sysinfo::{System, ProcessRefreshKind, RefreshKind, ProcessesToUpdate, Signal, Pid};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Pid,
    Cpu,
    Memory,
    Name,
    User,
}

/// Process signal types (matching btop's comprehensive signal support)
#[derive(Debug, Clone, Copy)]
pub enum ProcessSignal {
    Term,    // SIGTERM (15) - Termination signal
    Kill,    // SIGKILL (9) - Kill signal
    Int,     // SIGINT (2) - Interrupt
    Hup,     // SIGHUP (1) - Hangup
    Quit,    // SIGQUIT (3) - Quit
    Stop,    // SIGSTOP (19) - Stop process
    Cont,    // SIGCONT (18) - Continue process
    Usr1,    // SIGUSR1 (10) - User-defined signal 1
    Usr2,    // SIGUSR2 (12) - User-defined signal 2
}

impl ProcessSignal {
    pub fn to_sysinfo_signal(&self) -> Signal {
        match self {
            ProcessSignal::Term => Signal::Term,
            ProcessSignal::Kill => Signal::Kill,
            ProcessSignal::Int => Signal::Interrupt,
            ProcessSignal::Hup => Signal::Hangup,
            ProcessSignal::Quit => Signal::Quit,
            ProcessSignal::Stop => Signal::Stop,
            ProcessSignal::Cont => Signal::Continue,
            ProcessSignal::Usr1 => Signal::User1,
            ProcessSignal::Usr2 => Signal::User2,
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            ProcessSignal::Term => "TERM (15)",
            ProcessSignal::Kill => "KILL (9)",
            ProcessSignal::Int => "INT (2)",
            ProcessSignal::Hup => "HUP (1)",
            ProcessSignal::Quit => "QUIT (3)",
            ProcessSignal::Stop => "STOP (19)",
            ProcessSignal::Cont => "CONT (18)",
            ProcessSignal::Usr1 => "USR1 (10)",
            ProcessSignal::Usr2 => "USR2 (12)",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            ProcessSignal::Term => "Graceful termination",
            ProcessSignal::Kill => "Force kill (cannot be ignored)",
            ProcessSignal::Int => "Interrupt from keyboard (Ctrl+C)",
            ProcessSignal::Hup => "Hangup detected on controlling terminal",
            ProcessSignal::Quit => "Quit from keyboard",
            ProcessSignal::Stop => "Stop process (cannot be ignored)",
            ProcessSignal::Cont => "Continue if stopped",
            ProcessSignal::Usr1 => "User-defined signal 1",
            ProcessSignal::Usr2 => "User-defined signal 2",
        }
    }
    
    pub fn all_signals() -> Vec<ProcessSignal> {
        vec![
            ProcessSignal::Term,
            ProcessSignal::Kill,
            ProcessSignal::Int,
            ProcessSignal::Hup,
            ProcessSignal::Quit,
            ProcessSignal::Stop,
            ProcessSignal::Cont,
            ProcessSignal::Usr1,
            ProcessSignal::Usr2,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub user: String,
    pub state: String,
    pub threads: usize,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub children: Vec<u32>,
    pub tree_depth: usize,
}

pub struct ProcessMonitor {
    system: System,
    sort_order: SortOrder,
    reverse: bool,
    tree_view: bool,
    process_tree: HashMap<u32, Vec<u32>>,
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
            tree_view: false,
            process_tree: HashMap::new(),
        }
    }
    
    pub fn update(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        if self.tree_view {
            self.build_process_tree();
        }
    }
    
    pub fn set_sort_order(&mut self, order: SortOrder) {
        if self.sort_order == order {
            self.reverse = !self.reverse;
        } else {
            self.sort_order = order;
            self.reverse = false;
        }
    }
    
    pub fn toggle_tree_view(&mut self) {
        self.tree_view = !self.tree_view;
        if self.tree_view {
            self.build_process_tree();
        }
    }
    
    pub fn is_tree_view(&self) -> bool {
        self.tree_view
    }
    
    fn build_process_tree(&mut self) {
        self.process_tree.clear();
        
        for (pid, process) in self.system.processes() {
            if let Some(parent) = process.parent() {
                self.process_tree
                    .entry(parent.as_u32())
                    .or_insert_with(Vec::new)
                    .push(pid.as_u32());
            }
        }
    }
    
    fn get_process_children(&self, pid: u32) -> Vec<u32> {
        self.process_tree.get(&pid).cloned().unwrap_or_default()
    }
    
    pub fn get_sorted_processes(&self) -> Vec<ProcessInfo> {
        let mut processes: Vec<ProcessInfo> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let ppid = process.parent().map(|p| p.as_u32());
                let children = if self.tree_view {
                    self.get_process_children(pid.as_u32())
                } else {
                    Vec::new()
                };
                
                // Get process state
                let state = if process.status().to_string().contains("Run") {
                    "R".to_string()
                } else if process.status().to_string().contains("Sleep") {
                    "S".to_string()
                } else if process.status().to_string().contains("Zombie") {
                    "Z".to_string()
                } else {
                    "?".to_string()
                };
                
                ProcessInfo {
                    pid: pid.as_u32(),
                    ppid,
                    name: process.name().to_string_lossy().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory: process.memory(),
                    user: process.user_id()
                        .map(|uid| uid.to_string())
                        .unwrap_or_else(|| "unknown".to_string()),
                    state,
                    threads: 1, // sysinfo doesn't provide thread count directly
                    read_bytes: process.disk_usage().read_bytes,
                    write_bytes: process.disk_usage().written_bytes,
                    children,
                    tree_depth: 0,
                }
            })
            .collect();
        
        if self.tree_view {
            return self.build_tree_list(processes);
        }
        
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
            SortOrder::Name => {
                processes.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortOrder::User => {
                processes.sort_by(|a, b| a.user.cmp(&b.user));
            }
        }
        
        if self.reverse {
            processes.reverse();
        }
        
        processes
    }
    
    fn build_tree_list(&self, mut processes: Vec<ProcessInfo>) -> Vec<ProcessInfo> {
        let mut result = Vec::new();
        let proc_map: HashMap<u32, ProcessInfo> = processes
            .drain(..)
            .map(|p| (p.pid, p))
            .collect();
        
        // Find root processes (those without parents or with non-existent parents)
        let root_pids: Vec<u32> = proc_map
            .values()
            .filter(|p| p.ppid.is_none() || !proc_map.contains_key(&p.ppid.unwrap()))
            .map(|p| p.pid)
            .collect();
        
        // Recursively build tree
        for pid in root_pids {
            self.add_process_tree(&proc_map, pid, 0, &mut result);
        }
        
        result
    }
    
    fn add_process_tree(
        &self,
        proc_map: &HashMap<u32, ProcessInfo>,
        pid: u32,
        depth: usize,
        result: &mut Vec<ProcessInfo>,
    ) {
        if let Some(mut proc) = proc_map.get(&pid).cloned() {
            proc.tree_depth = depth;
            let children = proc.children.clone();
            result.push(proc);
            
            for child_pid in children {
                self.add_process_tree(proc_map, child_pid, depth + 1, result);
            }
        }
    }
    
    /// Send a signal to a process
    pub fn send_signal(&self, pid: u32, signal: ProcessSignal) -> Result<bool, String> {
        let sysinfo_pid = Pid::from_u32(pid);
        
        if let Some(process) = self.system.process(sysinfo_pid) {
            match process.kill_with(signal.to_sysinfo_signal()) {
                Some(true) => Ok(true),
                Some(false) => Err("Failed to send signal".to_string()),
                None => Err("Signal not supported on this platform".to_string()),
            }
        } else {
            Err("Process not found".to_string())
        }
    }
    
    /// Kill a process (SIGTERM by default, SIGKILL if force)
    pub fn kill_process(&self, pid: u32, force: bool) -> Result<bool, String> {
        let signal = if force {
            ProcessSignal::Kill
        } else {
            ProcessSignal::Term
        };
        self.send_signal(pid, signal)
    }
    
    pub fn get_process_count(&self) -> usize {
        self.system.processes().len()
    }
}
