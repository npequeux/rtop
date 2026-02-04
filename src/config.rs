use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub refresh_rates: RefreshRates,
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub export: ExportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRates {
    #[serde(default = "default_cpu_refresh")]
    pub cpu: u64,
    #[serde(default = "default_memory_refresh")]
    pub memory: u64,
    #[serde(default = "default_network_refresh")]
    pub network: u64,
    #[serde(default = "default_disk_refresh")]
    pub disk: u64,
    #[serde(default = "default_process_refresh")]
    pub process: u64,
    #[serde(default = "default_temp_refresh")]
    pub temp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub enable_colors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
    pub show_temperature: bool,
    #[serde(default = "default_true")]
    pub show_network: bool,
    #[serde(default = "default_true")]
    pub show_disk: bool,
    #[serde(default = "default_max_processes")]
    pub max_processes: usize,
    #[serde(default = "default_false")]
    pub show_kernel_processes: bool,
    #[serde(default = "default_true")]
    pub show_self: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thresholds {
    #[serde(default = "default_cpu_warning")]
    pub cpu_warning: f32,
    #[serde(default = "default_cpu_critical")]
    pub cpu_critical: f32,
    #[serde(default = "default_memory_warning")]
    pub memory_warning: f32,
    #[serde(default = "default_memory_critical")]
    pub memory_critical: f32,
    #[serde(default = "default_temp_warning")]
    pub temp_warning: f32,
    #[serde(default = "default_temp_critical")]
    pub temp_critical: f32,
    #[serde(default = "default_disk_warning")]
    pub disk_warning: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    #[serde(default = "default_false")]
    pub enable_logging: bool,
    #[serde(default)]
    pub log_path: Option<PathBuf>,
    #[serde(default = "default_log_interval")]
    pub log_interval: u64,
}

// Default value functions
fn default_cpu_refresh() -> u64 {
    1000
}
fn default_memory_refresh() -> u64 {
    1000
}
fn default_network_refresh() -> u64 {
    1000
}
fn default_disk_refresh() -> u64 {
    2000
}
fn default_process_refresh() -> u64 {
    2000
}
fn default_temp_refresh() -> u64 {
    1000
}
fn default_theme() -> String {
    "cyan".to_string()
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_max_processes() -> usize {
    20
}
fn default_cpu_warning() -> f32 {
    60.0
}
fn default_cpu_critical() -> f32 {
    80.0
}
fn default_memory_warning() -> f32 {
    70.0
}
fn default_memory_critical() -> f32 {
    90.0
}
fn default_temp_warning() -> f32 {
    65.0
}
fn default_temp_critical() -> f32 {
    80.0
}
fn default_disk_warning() -> f32 {
    80.0
}
fn default_log_interval() -> u64 {
    5000
}

impl Default for RefreshRates {
    fn default() -> Self {
        Self {
            cpu: default_cpu_refresh(),
            memory: default_memory_refresh(),
            network: default_network_refresh(),
            disk: default_disk_refresh(),
            process: default_process_refresh(),
            temp: default_temp_refresh(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            enable_colors: default_true(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_temperature: default_true(),
            show_network: default_true(),
            show_disk: default_true(),
            max_processes: default_max_processes(),
            show_kernel_processes: default_false(),
            show_self: default_true(),
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            cpu_warning: default_cpu_warning(),
            cpu_critical: default_cpu_critical(),
            memory_warning: default_memory_warning(),
            memory_critical: default_memory_critical(),
            temp_warning: default_temp_warning(),
            temp_critical: default_temp_critical(),
            disk_warning: default_disk_warning(),
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            enable_logging: default_false(),
            log_path: None,
            log_interval: default_log_interval(),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(config_path, contents)?;
        Ok(())
    }

    pub fn create_default_config() -> anyhow::Result<()> {
        let config = Config::default();
        config.save()
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("rtop").join("config.toml"))
    }

    pub fn cpu_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.cpu)
    }

    #[allow(dead_code)]
    pub fn memory_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.memory)
    }

    #[allow(dead_code)]
    pub fn network_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.network)
    }

    pub fn disk_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.disk)
    }

    pub fn process_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.process)
    }

    #[allow(dead_code)]
    pub fn temp_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rates.temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.refresh_rates.cpu, 1000);
        assert_eq!(config.refresh_rates.memory, 1000);
        assert_eq!(config.refresh_rates.network, 1000);
        assert_eq!(config.refresh_rates.disk, 2000);
        assert_eq!(config.refresh_rates.process, 2000);
        assert_eq!(config.colors.theme, "cyan");
        assert!(config.colors.enable_colors);
        assert_eq!(config.display.max_processes, 20);
    }

    #[test]
    fn test_refresh_durations() {
        let config = Config::default();
        assert_eq!(config.cpu_refresh_duration(), Duration::from_millis(1000));
        assert_eq!(config.memory_refresh_duration(), Duration::from_millis(1000));
        assert_eq!(config.disk_refresh_duration(), Duration::from_millis(2000));
        assert_eq!(config.process_refresh_duration(), Duration::from_millis(2000));
    }

    #[test]
    fn test_threshold_defaults() {
        let thresholds = Thresholds::default();
        assert_eq!(thresholds.cpu_warning, 60.0);
        assert_eq!(thresholds.cpu_critical, 80.0);
        assert_eq!(thresholds.memory_warning, 70.0);
        assert_eq!(thresholds.memory_critical, 90.0);
        assert_eq!(thresholds.temp_warning, 65.0);
        assert_eq!(thresholds.temp_critical, 80.0);
        assert_eq!(thresholds.disk_warning, 80.0);
    }

    #[test]
    fn test_display_config_defaults() {
        let display = DisplayConfig::default();
        assert!(display.show_temperature);
        assert!(display.show_network);
        assert!(display.show_disk);
        assert!(display.show_self);
        assert!(!display.show_kernel_processes);
        assert_eq!(display.max_processes, 20);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_string = toml::to_string(&config).unwrap();
        assert!(toml_string.contains("cpu"));
        assert!(toml_string.contains("memory"));
        assert!(toml_string.contains("theme"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [refresh_rates]
            cpu = 2000
            memory = 1500
            
            [colors]
            theme = "blue"
            enable_colors = false
            
            [display]
            max_processes = 30
        "#;
        
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.refresh_rates.cpu, 2000);
        assert_eq!(config.refresh_rates.memory, 1500);
        assert_eq!(config.colors.theme, "blue");
        assert!(!config.colors.enable_colors);
        assert_eq!(config.display.max_processes, 30);
    }

    #[test]
    fn test_partial_config() {
        let toml_str = r#"
            [colors]
            theme = "red"
        "#;
        
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.colors.theme, "red");
        // Other values should use defaults
        assert_eq!(config.refresh_rates.cpu, 1000);
        assert_eq!(config.display.max_processes, 20);
    }
}
