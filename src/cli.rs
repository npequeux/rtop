//! Command-line interface argument parsing for rtop.
//!
//! This module defines the CLI structure using clap for parsing command-line
//! arguments and subcommands.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// rtop - Advanced system monitoring dashboard for terminal.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "rtop")]
#[command(about = "System monitoring dashboard for terminal", long_about = None)]
pub struct Cli {
    /// Enable minimal mode (reduced updates)
    #[arg(short, long)]
    pub minimal: bool,

    /// Disable colors (monochrome mode)
    #[arg(long)]
    pub no_color: bool,

    /// Update interval in milliseconds (overrides config)
    #[arg(short = 'i', long)]
    pub interval: Option<u64>,

    /// Export metrics to file and exit
    #[arg(short = 'e', long)]
    pub export: Option<PathBuf>,

    /// Export format (json, csv)
    #[arg(short = 'f', long, default_value = "json")]
    pub format: String,

    /// Enable logging to file
    #[arg(short = 'l', long)]
    pub log: Option<PathBuf>,

    /// Log interval in seconds
    #[arg(long, default_value = "5")]
    pub log_interval: u64,

    /// Run for specified duration then exit (e.g., "1h", "30m", "60s")
    #[arg(short = 'd', long)]
    pub duration: Option<String>,

    /// Config file path (default: ~/.config/rtop/config.toml)
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,

    /// Generate default config file
    #[arg(long)]
    pub generate_config: bool,

    /// Increase verbosity (can be repeated)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands for rtop.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show current configuration
    ShowConfig,

    /// Generate default configuration file
    InitConfig,

    /// Export current metrics and exit
    Export {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Format (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

impl Cli {
    /// Parses a duration string into a std::time::Duration.
    ///
    /// Accepts formats like "1h", "30m", "60s", or plain numbers (interpreted as seconds).
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration string to parse
    ///
    /// # Returns
    ///
    /// A Duration if parsing succeeds, or an error if the format is invalid.
    ///
    /// # Examples
    ///
    /// Parses "1h" as 3600 seconds, "30m" as 1800 seconds, and "60s" as 60 seconds.
    pub fn parse_duration(duration: &str) -> anyhow::Result<std::time::Duration> {
        let duration = duration.trim();

        if duration.ends_with('s') {
            let secs: u64 = duration.trim_end_matches('s').parse()?;
            Ok(std::time::Duration::from_secs(secs))
        } else if duration.ends_with('m') {
            let mins: u64 = duration.trim_end_matches('m').parse()?;
            Ok(std::time::Duration::from_secs(mins * 60))
        } else if duration.ends_with('h') {
            let hours: u64 = duration.trim_end_matches('h').parse()?;
            Ok(std::time::Duration::from_secs(hours * 3600))
        } else {
            // Default to seconds if no suffix
            let secs: u64 = duration.parse()?;
            Ok(std::time::Duration::from_secs(secs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_seconds() {
        assert_eq!(
            Cli::parse_duration("30s").unwrap(),
            std::time::Duration::from_secs(30)
        );
        assert_eq!(
            Cli::parse_duration("60s").unwrap(),
            std::time::Duration::from_secs(60)
        );
    }

    #[test]
    fn test_parse_duration_minutes() {
        assert_eq!(
            Cli::parse_duration("5m").unwrap(),
            std::time::Duration::from_secs(300)
        );
        assert_eq!(
            Cli::parse_duration("30m").unwrap(),
            std::time::Duration::from_secs(1800)
        );
    }

    #[test]
    fn test_parse_duration_hours() {
        assert_eq!(
            Cli::parse_duration("1h").unwrap(),
            std::time::Duration::from_secs(3600)
        );
        assert_eq!(
            Cli::parse_duration("2h").unwrap(),
            std::time::Duration::from_secs(7200)
        );
    }

    #[test]
    fn test_parse_duration_no_suffix() {
        assert_eq!(
            Cli::parse_duration("45").unwrap(),
            std::time::Duration::from_secs(45)
        );
    }

    #[test]
    fn test_parse_duration_with_whitespace() {
        assert_eq!(
            Cli::parse_duration("  10s  ").unwrap(),
            std::time::Duration::from_secs(10)
        );
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(Cli::parse_duration("invalid").is_err());
        assert!(Cli::parse_duration("abc").is_err());
        assert!(Cli::parse_duration("").is_err());
    }
}
