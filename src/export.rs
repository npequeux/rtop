//! Data export functionality for system metrics.
//!
//! This module provides structures and functions for exporting system monitoring
//! data to various formats including JSON and CSV.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Complete system metrics snapshot for export.
///
/// Contains all monitoring data collected at a specific point in time,
/// including CPU, memory, network, disk, processes, temperature, and system information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// ISO 8601 timestamp of when metrics were collected
    pub timestamp: String,
    /// CPU usage metrics
    pub cpu: CpuMetrics,
    /// Memory and swap usage metrics
    pub memory: MemoryMetrics,
    /// Network transfer metrics
    pub network: NetworkMetrics,
    /// Per-disk usage metrics
    pub disk: Vec<DiskMetrics>,
    /// Top processes metrics
    pub processes: Vec<ProcessMetrics>,
    /// Temperature sensor readings (optional, may not be available on all systems)
    pub temperature: Option<TempMetrics>,
    /// System information
    pub system: SystemMetrics,
}

/// CPU usage metrics for all cores.
#[derive(Debug, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Per-core usage statistics
    pub cores: Vec<CoreMetric>,
    /// Average usage across all cores
    pub average: f32,
}

/// Individual CPU core usage metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct CoreMetric {
    /// Core ID (0-indexed)
    pub id: usize,
    /// Usage percentage (0.0-100.0)
    pub usage: f32,
}

/// Memory and swap usage metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total physical memory in bytes
    pub total: u64,
    /// Used physical memory in bytes
    pub used: u64,
    /// Available physical memory in bytes
    pub available: u64,
    /// Memory usage percentage
    pub percent: f32,
    /// Total swap space in bytes
    pub swap_total: u64,
    /// Used swap space in bytes
    pub swap_used: u64,
    /// Swap usage percentage
    pub swap_percent: f32,
}

/// Network transfer metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Total bytes received since system boot
    pub received: u64,
    /// Total bytes transmitted since system boot
    pub transmitted: u64,
    /// Current receive rate in bytes per second
    pub rx_rate: f64,
    /// Current transmit rate in bytes per second
    pub tx_rate: f64,
}

/// Disk usage metrics for a single disk/partition.
#[derive(Debug, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Device name
    pub name: String,
    /// Mount point path
    pub mount_point: String,
    /// Total disk space in bytes
    pub total: u64,
    /// Available disk space in bytes
    pub available: u64,
    /// Usage percentage
    pub percent: f32,
}

/// Process metrics for a single process.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// CPU usage percentage
    pub cpu: f32,
    /// Memory usage in bytes
    pub memory: u64,
    /// Memory usage percentage
    pub memory_percent: f32,
}

/// Temperature sensor metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct TempMetrics {
    /// Individual sensor readings
    pub sensors: Vec<SensorMetric>,
    /// Average temperature across all sensors
    pub average: f32,
    /// Maximum temperature across all sensors
    pub max: f32,
}

/// Individual temperature sensor reading.
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorMetric {
    /// Sensor name/label
    pub name: String,
    /// Temperature in degrees Celsius
    pub temperature: f32,
}

/// System information metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// System hostname
    pub hostname: String,
    /// Operating system name
    pub os: String,
    /// Kernel version
    pub kernel: String,
    /// System uptime in seconds
    pub uptime: u64,
    /// Load average (1min, 5min, 15min)
    pub load_average: (f64, f64, f64),
}

impl Metrics {
    /// Exports metrics to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - File path where JSON will be written
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written.
    pub fn export_json<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Exports metrics to a CSV file.
    ///
    /// Exports a flattened view of the metrics suitable for time-series analysis.
    ///
    /// # Arguments
    ///
    /// * `path` - File path where CSV will be written
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written.
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
            processes: vec![ProcessMetrics {
                pid: 1234,
                name: "test_process".to_string(),
                cpu: 10.5,
                memory: 1_000_000,
                memory_percent: 0.01,
            }],
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
