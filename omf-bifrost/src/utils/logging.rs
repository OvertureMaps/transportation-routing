// src/utils/logging.rs
use log::LevelFilter;

/// Configure the logger with the specified verbosity level
pub fn configure_logging(verbosity: u8) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}