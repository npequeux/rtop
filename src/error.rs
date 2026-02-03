use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum RtopError {
    #[error("Failed to initialize terminal: {0}")]
    TerminalInit(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Sensor not available: {0}")]
    SensorUnavailable(String),

    #[error("Failed to update monitor: {0}")]
    MonitorUpdate(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RtopError>;
