use sysinfo::System;

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn update(&mut self) {
        self.system.refresh_memory();
        self.system.refresh_cpu_usage();
    }

    pub fn hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn os_version(&self) -> String {
        format!(
            "{} {}",
            System::name().unwrap_or_else(|| "Unknown".to_string()),
            System::os_version().unwrap_or_default()
        )
    }

    pub fn kernel_version(&self) -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    pub fn uptime_formatted(&self) -> String {
        let uptime = self.uptime();
        let days = uptime / 86400;
        let hours = (uptime % 86400) / 3600;
        let minutes = (uptime % 3600) / 60;
        let seconds = uptime % 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    pub fn load_average(&self) -> (f64, f64, f64) {
        let load_avg = System::load_average();
        (load_avg.one, load_avg.five, load_avg.fifteen)
    }

    pub fn total_processes(&self) -> usize {
        self.system.processes().len()
    }
}
