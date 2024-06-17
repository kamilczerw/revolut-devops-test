use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use log::LevelFilter;

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum LogEncoder {
    Json,
    Text,
}

/// Revolut interview assignment for DevOps role.
/// The application is self-contained and does not require running any external dependencies.
#[derive(Parser, Debug)]
#[command(version, about = "Hello world application", long_about)]
pub(crate) struct Cli {
    /// Path to the directory where the data will be stored
    #[arg(
        short,
        long = "data-dir",
        value_name = "DATA_DIR",
        default_value = "./.local/data"
    )]
    pub data_dir: PathBuf,

    /// Log level.
    #[arg(short, long, default_value = "info")]
    pub log_level: LogLevel,

    /// Format of the log messages.
    #[arg(long, default_value = "text")]
    pub log_encoder: LogEncoder,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Off => LevelFilter::Off,
        }
    }
}
