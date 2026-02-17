//! Error types for rtop application.
//!
//! This module defines custom error types used throughout the application
//! using the thiserror crate for ergonomic error handling.

use thiserror::Error;

/// Application-specific error types.
///
/// Each variant represents a different category of error that can occur
/// during rtop's operation.
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum RtopError {
    /// Failed to initialize or interact with the terminal
    #[error("Failed to initialize terminal: {0}")]
    TerminalInit(#[from] std::io::Error),

    /// Configuration file loading or parsing error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Requested sensor is not available on this system
    #[error("Sensor not available: {0}")]
    SensorUnavailable(String),

    /// Error during monitor data collection
    #[error("Failed to update monitor: {0}")]
    MonitorUpdate(String),

    /// Error during data export operation
    #[error("Export error: {0}")]
    Export(String),

    /// Insufficient permissions to access required resources
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Invalid user input or command argument
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Convenience type alias for Results using RtopError.
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RtopError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = RtopError::Config("Invalid value".to_string());
        assert_eq!(error.to_string(), "Configuration error: Invalid value");

        let error = RtopError::SensorUnavailable("Temperature".to_string());
        assert_eq!(error.to_string(), "Sensor not available: Temperature");

        let error = RtopError::PermissionDenied("/proc/stat".to_string());
        assert_eq!(error.to_string(), "Permission denied: /proc/stat");
    }

    #[test]
    fn test_error_types() {
        let _config_err = RtopError::Config("test".to_string());
        let _sensor_err = RtopError::SensorUnavailable("test".to_string());
        let _monitor_err = RtopError::MonitorUpdate("test".to_string());
        let _export_err = RtopError::Export("test".to_string());
        let _permission_err = RtopError::PermissionDenied("test".to_string());
        let _input_err = RtopError::InvalidInput("test".to_string());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let rtop_error: RtopError = io_error.into();

        match rtop_error {
            RtopError::TerminalInit(_) => {}
            _ => panic!("Expected TerminalInit variant"),
        }
    }

    #[test]
    fn test_result_type() {
        fn success_fn() -> Result<i32> {
            Ok(42)
        }
        fn failure_fn() -> Result<i32> {
            Err(RtopError::InvalidInput("test".to_string()))
        }

        let success = success_fn();
        assert_eq!(success.unwrap(), 42);

        let failure = failure_fn();
        assert!(failure.is_err());
    }
}
