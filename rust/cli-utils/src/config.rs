//! This module contains config options for the application
//!


pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct holds the logging configuration for the application
/// # Examples
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ````
/// Create a new Logging strcut with custom values:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File("log.txt".to_string()),
/// };
///```
///
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
