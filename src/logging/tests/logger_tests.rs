use crate::logging::{Logger, TimeFormat, Trace};
use di::Ref;
use log::*;

#[test]
fn log_time_local() {
    // Arrange
    let logger = Logger::with(Trace, TimeFormat::Local);
    Logger::init(Ref::new(logger));

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
    let logger = Logger::with(Trace, TimeFormat::Utc);
    Logger::init(Ref::new(logger));

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
    let logger = Logger::with(Trace, TimeFormat::Elapsed);
    Logger::init(Ref::new(logger));

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
    let logger = Logger::with(Trace, TimeFormat::None);
    Logger::init(Ref::new(logger));

    // Act
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
