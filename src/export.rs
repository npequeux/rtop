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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_metrics() -> Metrics {
        Metrics {
            timestamp: "2026-02-04T20:00:00Z".to_string(),
            cpu: CpuMetrics {
                cores: vec![
                    CoreMetric { id: 0, usage: 25.5 },
                    CoreMetric { id: 1, usage: 30.2 },
                ],
                average: 27.85,
            },
            memory: MemoryMetrics {
                total: 16_000_000_000,
                used: 8_000_000_000,
                available: 8_000_000_000,
                percent: 50.0,
                swap_total: 8_000_000_000,
                swap_used: 1_000_000_000,
                swap_percent: 12.5,
            },
            network: NetworkMetrics {
                received: 1_000_000,
                transmitted: 500_000,
                rx_rate: 1024.5,
                tx_rate: 512.3,
            },
            disk: vec![DiskMetrics {
                name: "nvme0n1".to_string(),
                mount_point: "/".to_string(),
                total: 500_000_000_000,
                available: 250_000_000_000,
                percent: 50.0,
            }],
            processes: vec![
                ProcessMetrics {
                    pid: 1234,
                    name: "test_process".to_string(),
                    cpu: 10.5,
                    memory: 1_000_000,
                    memory_percent: 0.01,
                },
            ],
            temperature: Some(TempMetrics {
                sensors: vec![SensorMetric {
                    name: "CPU".to_string(),
                    temperature: 55.0,
                }],
                average: 55.0,
                max: 55.0,
            }),
            system: SystemMetrics {
                hostname: "test-host".to_string(),
                os: "Linux".to_string(),
                kernel: "6.5.0".to_string(),
                uptime: 86400,
                load_average: (1.5, 1.2, 0.9),
            },
        }
    }

    #[test]
    fn test_export_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("metrics.json");
        
        let metrics = create_test_metrics();
        metrics.export_json(&file_path).unwrap();
        
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("timestamp"));
        assert!(content.contains("2026-02-04T20:00:00Z"));
        assert!(content.contains("test-host"));
    }

    #[test]
    fn test_export_csv() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("metrics.csv");
        
        let metrics = create_test_metrics();
        metrics.export_csv(&file_path).unwrap();
        
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        
        assert_eq!(lines.len(), 2); // Header + 1 data row
        assert!(lines[0].contains("timestamp"));
        assert!(lines[0].contains("cpu_avg"));
        assert!(lines[0].contains("memory_percent"));
        assert!(lines[1].contains("2026-02-04T20:00:00Z"));
        assert!(lines[1].contains("27.85"));
        assert!(lines[1].contains("50.00"));
    }

    #[test]
    fn test_metrics_serialization() {
        let metrics = create_test_metrics();
        let json = serde_json::to_string(&metrics).unwrap();
        
        assert!(json.contains("timestamp"));
        assert!(json.contains("cpu"));
        assert!(json.contains("memory"));
        assert!(json.contains("network"));
    }

    #[test]
    fn test_metrics_deserialization() {
        let json = r#"{
            "timestamp": "2026-02-04T20:00:00Z",
            "cpu": {"cores": [], "average": 25.0},
            "memory": {
                "total": 1000, "used": 500, "available": 500,
                "percent": 50.0, "swap_total": 100, "swap_used": 50,
                "swap_percent": 50.0
            },
            "network": {"received": 100, "transmitted": 50, "rx_rate": 10.0, "tx_rate": 5.0},
            "disk": [],
            "processes": [],
            "temperature": null,
            "system": {
                "hostname": "test", "os": "Linux", "kernel": "6.5",
                "uptime": 1000, "load_average": [1.0, 1.0, 1.0]
            }
        }"#;
        
        let metrics: Metrics = serde_json::from_str(json).unwrap();
        assert_eq!(metrics.timestamp, "2026-02-04T20:00:00Z");
        assert_eq!(metrics.cpu.average, 25.0);
        assert_eq!(metrics.system.hostname, "test");
    }
}
