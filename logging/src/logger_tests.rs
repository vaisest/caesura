use crate::*;
use log::*;
use std::sync::Arc;
use std::time::SystemTime;

#[test]
fn log_time_local() {
    // Arrange
    let logger = Logger {
        enabled_threshold: Trace,
        time_format: TimeFormat::Local,
        start: SystemTime::now(),
        package_name: "coda".to_owned(),
    };
    Logger::init(Arc::new(logger));

    // Act
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[test]
fn log_time_utc() {
    // Arrange
    let logger = Logger {
        enabled_threshold: Trace,
        time_format: TimeFormat::Utc,
        start: SystemTime::now(),
        package_name: "coda".to_owned(),
    };
    Logger::init(Arc::new(logger));

    // Act
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[test]
fn log_time_elapsed() {
    // Arrange
    let logger = Logger {
        enabled_threshold: Trace,
        time_format: TimeFormat::Elapsed,
        start: SystemTime::now(),
        package_name: "coda".to_owned(),
    };
    Logger::init(Arc::new(logger));

    // Act
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[test]
fn log_time_none() {
    // Arrange
    let logger = Logger {
        enabled_threshold: Trace,
        time_format: TimeFormat::None,
        start: SystemTime::now(),
        package_name: "coda".to_owned(),
    };
    Logger::init(Arc::new(logger));

    // Act
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
