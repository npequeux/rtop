use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub timestamp: String,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub network: NetworkMetrics,
    pub disk: Vec<DiskMetrics>,
    pub processes: Vec<ProcessMetrics>,
    pub temperature: Option<TempMetrics>,
    pub system: SystemMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub cores: Vec<CoreMetric>,
    pub average: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreMetric {
    pub id: usize,
    pub usage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub received: u64,
    pub transmitted: u64,
    pub rx_rate: f64,
    pub tx_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub memory_percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempMetrics {
    pub sensors: Vec<SensorMetric>,
    pub average: f32,
    pub max: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorMetric {
    pub name: String,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime: u64,
    pub load_average: (f64, f64, f64),
}

impl Metrics {
    pub fn export_json<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn export_csv<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let mut file = File::create(path)?;
        
        // Write header
        writeln!(file, "timestamp,cpu_avg,memory_percent,swap_percent,network_rx_rate,network_tx_rate,uptime,load_1m,load_5m,load_15m")?;
        
        // Write data
        writeln!(
            file,
            "{},{:.2},{:.2},{:.2},{:.2},{:.2},{},{:.2},{:.2},{:.2}",
            self.timestamp,
            self.cpu.average,
            self.memory.percent,
            self.memory.swap_percent,
            self.network.rx_rate,
            self.network.tx_rate,
            self.system.uptime,
            self.system.load_average.0,
            self.system.load_average.1,
            self.system.load_average.2,
        )?;
        
        Ok(())
    }
}
