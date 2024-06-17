use std::{net::SocketAddr, path::PathBuf};

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
        default_value = "./.local/data",
        env = "REVOLUT_DATA_DIR"
    )]
    pub data_dir: PathBuf,

    /// Address to bind the HTTP server to.
    #[arg(
        short = 'a',
        long = "bind-address",
        default_value = "[::1]:4200",
        env = "REVOLUT_BIND_ADDRESS"
    )]
    pub bind_addr: SocketAddr,

    /// Log level.
    #[arg(short, long, default_value = "info", env = "REVOLUT_LOG_LEVEL")]
    pub log_level: LogLevel,

    /// Format of the log messages.
    #[arg(long, default_value = "text", env = "REVOLUT_LOG_ENCODER")]
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
