use std::sync::Arc;
use chrono::{Local, Utc};
use colored::control::SHOULD_COLORIZE;
use colored::{ColoredString, Colorize};
use log::*;
use std::time::SystemTime;

use crate::*;

pub struct Logger {
    pub enabled_threshold: Verbosity,
    pub time_format: TimeFormat,
    pub start: SystemTime,
    pub package_name: String
}

impl Logger {
    //noinspection RsExperimentalTraitObligations
    pub fn init(logger: Arc<Self>) {
        SHOULD_COLORIZE.set_override(true);
        let filter = logger.enabled_threshold.to_level_filter();
        let logger = Box::new(logger);
        if let Err(error) = set_boxed_logger(logger).map(|()| set_max_level(filter)) {
            trace!("{} to initialize the logger: {}", "Failed".bold(), error);
        }
    }

    /// Force init the logger so logs aren't lost to the void prior to builder initialization.
    pub fn force_init(package_name: String) {
        let logger = Logger {
            enabled_threshold: Trace,
            time_format: TimeFormat::Local,
            start: SystemTime::now(),
            package_name
        };
        Logger::init(Arc::new(logger));
    }

    fn format_log(&self, verbosity: Verbosity, message: String) -> String {
        let prefix = self.format_prefix(verbosity);
        let message = format_message(verbosity, message);
        format!("{prefix} {message}")
    }

    #[must_use]
    pub fn format_prefix(&self, verbosity: Verbosity) -> String {
        let time = self.format_time();
        let verbosity_id = verbosity.get_id();
        let icon = verbosity.get_icon();
        format!("{time}{verbosity_id} {icon}")
    }

    fn format_time(&self) -> ColoredString {
        let value = match self.time_format {
            TimeFormat::Local => Local::now().format("%Y-%m-%d %H:%M:%S%.3f ").to_string(),
            TimeFormat::Utc => Utc::now().format("%Y-%m-%d %H:%M:%S%.3fZ ").to_string(),
            TimeFormat::Elapsed => format!(
                "{:>8.3} ",
                self.start.elapsed().unwrap_or_default().as_secs_f64()
            ),
            TimeFormat::None => String::new(),
        };
        value.dark_gray()
    }

    fn is_enabled(&self, verbosity: Verbosity) -> bool {
        verbosity.as_num() <= self.enabled_threshold.as_num()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if !metadata.target().starts_with(&self.package_name) {
            return false;
        }
        self.is_enabled(Verbosity::from_level(metadata.level()))
    }

    #[allow(clippy::print_stderr)]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let verbosity = Verbosity::from_level(record.level());
            let message = format!("{}", record.args());
            let log = self.format_log(verbosity, message);
            eprintln!("{log}");
        }
    }

    fn flush(&self) {}
}

fn format_message(verbosity: Verbosity, message: String) -> String {
    if verbosity.as_num() >= Debug.as_num() {
        format!("{}", message.dimmed())
    } else {
        message
    }
}
