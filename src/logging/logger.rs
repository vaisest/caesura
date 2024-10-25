use chrono::{Local, Utc};
use colored::control::SHOULD_COLORIZE;
use colored::{ColoredString, Colorize};
use di::{injectable, Ref};
use log::*;
use std::time::SystemTime;

use crate::built_info;
use crate::logging::*;
use crate::options::SharedOptions;

pub struct Logger {
    pub enabled_threshold: Verbosity,
    pub time_format: TimeFormat,
    pub start: SystemTime,
}

#[injectable]
impl Logger {
    #[must_use]
    pub fn new(options: Ref<SharedOptions>) -> Self {
        Self {
            enabled_threshold: options.verbosity.unwrap_or_default(),
            time_format: options.log_time.unwrap_or_default(),
            start: SystemTime::now(),
        }
    }

    #[must_use]
    pub fn with(verbosity: Verbosity, log_time: TimeFormat) -> Self {
        Self {
            enabled_threshold: verbosity,
            time_format: log_time,
            start: SystemTime::now(),
        }
    }

    //noinspection RsExperimentalTraitObligations
    pub fn init(logger: Ref<Self>) {
        SHOULD_COLORIZE.set_override(true);
        let filter = logger.enabled_threshold.to_level_filter();
        let logger = Box::new(logger);
        if let Err(error) = set_boxed_logger(logger).map(|()| set_max_level(filter)) {
            trace!("{} to initialize the logger: {}", "Failed".bold(), error);
        }
    }

    /// [`SharedOptions`] are read before [`Logger`] is initialized so if an error occurs
    /// it will be lost to the void unless we force inititialization.
    pub fn force_init() {
        let logger = Logger::with(Trace, TimeFormat::Local);
        Logger::init(Ref::new(logger));
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
        if !metadata.target().starts_with(built_info::PKG_NAME) {
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
